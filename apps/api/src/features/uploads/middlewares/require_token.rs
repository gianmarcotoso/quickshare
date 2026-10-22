use axum::{
    Json,
    extract::{Request, State},
    http::{StatusCode, header},
    middleware::Next,
    response::{IntoResponse, Response},
};

use crate::{
    app::state::AppState,
    core::error_code::{ErrorCode, ErrorDto},
};

pub async fn require_token(
    State(state): State<AppState>,
    request: Request,
    next: Next,
) -> Response {
    let token = request
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.strip_prefix("Bearer "));

    if token != Some(state.session().token()) {
        return (
            StatusCode::UNAUTHORIZED,
            Json(ErrorDto::new(ErrorCode::InvalidToken)),
        )
            .into_response();
    }

    next.run(request).await
}
