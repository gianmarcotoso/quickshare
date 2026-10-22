use std::sync::Arc;

use axum::extract::multipart::Field;

use crate::{
    core::{action_error::ActionError, error_code::ErrorCode},
    features::uploads::{
        actions::{
            create_unique_file::create_unique_file, record_received_file::record_received_file,
            write_field::write_field,
        },
        batch::Batch,
        partial_file::PartialFile,
        types::{received_file::ReceivedFile, uploader::Uploader},
    },
    libs::sanitize_file_name::sanitize_file_name,
};

pub async fn save_file(
    batch: Arc<Batch>,
    mut field: Field<'_>,
    uploader: Uploader,
) -> Result<ReceivedFile, ActionError> {
    let original_name = field.file_name().unwrap_or_default().to_string();
    let (name, path, file) =
        create_unique_file(batch.directory(), &sanitize_file_name(&original_name))
            .await
            .map_err(ActionError::internal)?;
    let partial = PartialFile::new(batch.clone(), path, original_name.clone());

    let (size, sha256) = write_field(&mut field, file).await.inspect_err(|error| {
        tracing::debug!(name = %original_name, %error, "upload failed");
    })?;

    let received = ReceivedFile {
        name,
        original_name,
        size,
        sha256,
        received_at: chrono::Utc::now(),
        uploader,
    };

    if !record_received_file(&batch, &received).map_err(ActionError::internal)? {
        return Err(ActionError::NotFound(ErrorCode::BatchNotFound));
    }

    partial.keep();

    Ok(received)
}
