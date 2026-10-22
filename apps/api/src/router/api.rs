use axum::{
    Json, Router,
    extract::DefaultBodyLimit,
    middleware,
    routing::{get, post},
};
use utoipa::OpenApi;

use crate::{
    app::state::AppState,
    core::app_config::API_PATH,
    features::uploads::{
        controllers::{batches as batches_controller, session as session_controller},
        middlewares::require_token::require_token,
    },
    openapi::{ApiDoc, OPENAPI_PATH},
};

pub fn routes(state: &AppState) -> Router<AppState> {
    let body_limit = match state.config().max_upload_bytes {
        Some(bytes) => DefaultBodyLimit::max(bytes as usize),
        None => DefaultBodyLimit::disable(),
    };

    let mut openapi = ApiDoc::openapi();
    openapi.servers = Some(vec![utoipa::openapi::Server::new(API_PATH)]);

    let protected = Router::new()
        .route("/session", get(session_controller::check))
        .route("/batches", post(batches_controller::create))
        .route(
            "/batches/{id}/files",
            post(batches_controller::upload).layer(body_limit),
        )
        .route("/batches/{id}/close", post(batches_controller::close))
        .route_layer(middleware::from_fn_with_state(state.clone(), require_token));

    protected.route(OPENAPI_PATH, get(move || async move { Json(openapi) }))
}
