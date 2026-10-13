use std::net::SocketAddr;

use axum::{
    Json,
    extract::{ConnectInfo, Multipart, Path, State},
    http::{HeaderMap, StatusCode},
};

use crate::{
    app::state::AppState,
    core::{
        action_error::ActionError,
        error_code::{ErrorCode, ErrorDto},
    },
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

const FILE_FIELD: &str = "file";

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
    ConnectInfo(address): ConnectInfo<SocketAddr>,
    headers: HeaderMap,
) -> Result<(StatusCode, Json<BatchDto>), ActionError> {
    let batch = create_batch(&state, &Uploader::from_request(address, &headers))?;

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
    ConnectInfo(address): ConnectInfo<SocketAddr>,
    headers: HeaderMap,
    mut multipart: Multipart,
) -> Result<(StatusCode, Json<ReceivedFileDto>), ActionError> {
    let batch = state
        .session()
        .batches()
        .get(&id)
        .cloned()
        .ok_or(ActionError::NotFound(ErrorCode::BatchNotFound))?;

    while let Some(field) = multipart.next_field().await? {
        if field.name() == Some(FILE_FIELD) {
            let received =
                receive_file(batch, field, Uploader::from_request(address, &headers)).await?;

            return Ok((StatusCode::CREATED, Json(received.into())));
        }
    }

    Err(ActionError::LogicViolation(ErrorCode::FileMissing))
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
