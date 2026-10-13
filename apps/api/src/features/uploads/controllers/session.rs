use axum::http::StatusCode;

#[utoipa::path(
    get,
    path = "/session",
    tag = "uploads",
    operation_id = "checkSession",
    security(("token" = [])),
    responses(
        (status = 204, description = "The token is valid"),
        (status = 401, description = "ERR_INVALID_TOKEN", body = crate::core::error_code::ErrorDto),
    )
)]
pub async fn check() -> StatusCode {
    StatusCode::NO_CONTENT
}
