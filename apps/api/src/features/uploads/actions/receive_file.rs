use std::sync::Arc;

use axum::extract::multipart::Field;
use sha2::{Digest, Sha256};
use tokio::{
    fs::File,
    io::{AsyncWriteExt, BufWriter},
};

use crate::{
    core::{action_error::ActionError, error_code::ErrorCode},
    features::uploads::{
        actions::{
            create_unique_file::create_unique_file, record_received_file::record_received_file,
        },
        types::{
            batch::Batch, partial_file::PartialFile, received_file::ReceivedFile,
            uploader::Uploader,
        },
    },
    libs::sanitize_file_name::sanitize_file_name,
};

pub async fn receive_file(
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

async fn write_field(field: &mut Field<'_>, file: File) -> Result<(u64, String), ActionError> {
    let mut writer = BufWriter::new(file);
    let mut hasher = Sha256::new();
    let mut size = 0u64;

    while let Some(chunk) = field.chunk().await? {
        hasher.update(&chunk);
        size += chunk.len() as u64;
        writer
            .write_all(&chunk)
            .await
            .map_err(ActionError::internal)?;
    }

    writer.flush().await.map_err(ActionError::internal)?;
    writer
        .into_inner()
        .sync_all()
        .await
        .map_err(ActionError::internal)?;

    Ok((size, hex::encode(hasher.finalize())))
}
