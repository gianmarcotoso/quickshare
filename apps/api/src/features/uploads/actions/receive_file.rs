use axum::extract::Multipart;

use crate::{
    app::state::AppState,
    core::{action_error::ActionError, error_code::ErrorCode},
    features::uploads::{
        actions::save_file::save_file,
        constants::FILE_FIELD,
        types::{received_file::ReceivedFile, uploader::Uploader},
    },
};

pub async fn receive_file(
    state: &AppState,
    batch_id: &str,
    mut multipart: Multipart,
    uploader: Uploader,
) -> Result<ReceivedFile, ActionError> {
    let batch = state
        .session()
        .batches()
        .get(batch_id)
        .cloned()
        .ok_or(ActionError::NotFound(ErrorCode::BatchNotFound))?;

    while let Some(field) = multipart.next_field().await? {
        if field.name() == Some(FILE_FIELD) {
            return save_file(batch, field, uploader).await;
        }
    }

    Err(ActionError::LogicViolation(ErrorCode::FileMissing))
}
