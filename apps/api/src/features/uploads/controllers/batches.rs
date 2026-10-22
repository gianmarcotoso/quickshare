use axum::{
    Json,
    extract::{Multipart, Path, State},
    http::StatusCode,
};

use crate::{
    app::state::AppState,
    core::{action_error::ActionError, error_code::ErrorDto},
    features::uploads::{
        actions::{
            close_batch::close_batch, create_batch::create_batch, receive_file::receive_file,
        },
        dtos::{
            responses::{batch_dto::BatchDto, received_file_dto::ReceivedFileDto},
            upload_file_dto::UploadFileDto,
        },
        types::uploader::Uploader,
    },
};

#[utoipa::path(
    post,
    path = "/batches",
    tag = "uploads",
    operation_id = "createBatch",
    security(("token" = [])),
    responses(
        (status = 201, description = "Batch folder and log created", body = BatchDto),
        (status = 401, description = "ERR_INVALID_TOKEN", body = ErrorDto),
    )
)]
pub async fn create(
    State(state): State<AppState>,
    uploader: Uploader,
) -> Result<(StatusCode, Json<BatchDto>), ActionError> {
    let batch = create_batch(&state, &uploader)?;

    Ok((StatusCode::CREATED, Json(batch.as_ref().into())))
}

#[utoipa::path(
    post,
    path = "/batches/{id}/files",
    tag = "uploads",
    operation_id = "uploadFile",
    security(("token" = [])),
    params(("id" = String, Path)),
    request_body(content = UploadFileDto, content_type = "multipart/form-data"),
    responses(
        (status = 201, description = "Received and saved", body = ReceivedFileDto),
        (status = 400, description = "ERR_FILE_MISSING, ERR_UPLOAD_INTERRUPTED", body = ErrorDto),
        (status = 401, description = "ERR_INVALID_TOKEN", body = ErrorDto),
        (status = 404, description = "ERR_BATCH_NOT_FOUND", body = ErrorDto),
        (status = 413, description = "ERR_FILE_TOO_LARGE", body = ErrorDto),
    )
)]
pub async fn upload(
    State(state): State<AppState>,
    Path(id): Path<String>,
    uploader: Uploader,
    multipart: Multipart,
) -> Result<(StatusCode, Json<ReceivedFileDto>), ActionError> {
    let received = receive_file(&state, &id, multipart, uploader).await?;

    Ok((StatusCode::CREATED, Json(received.into())))
}

#[utoipa::path(
    post,
    path = "/batches/{id}/close",
    tag = "uploads",
    operation_id = "closeBatch",
    security(("token" = [])),
    params(("id" = String, Path)),
    responses(
        (status = 204, description = "Batch closed, log sealed with its hash"),
        (status = 401, description = "ERR_INVALID_TOKEN", body = ErrorDto),
        (status = 404, description = "ERR_BATCH_NOT_FOUND", body = ErrorDto),
    )
)]
pub async fn close(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<StatusCode, ActionError> {
    close_batch(&state, &id)?;

    Ok(StatusCode::NO_CONTENT)
}
