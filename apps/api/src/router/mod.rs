mod api;

use axum::Router;
use tower_http::{
    LatencyUnit,
    trace::{DefaultMakeSpan, DefaultOnFailure, DefaultOnRequest, DefaultOnResponse, TraceLayer},
};
use tracing::Level;

use crate::{
    app::state::AppState, core::app_config::API_PATH,
    features::panel::controllers::panel as panel_controller,
};

pub fn router(state: AppState) -> Router {
    Router::new()
        .nest(API_PATH, api::routes(&state).with_state(state))
        .fallback(panel_controller::asset)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().level(Level::DEBUG))
                .on_request(DefaultOnRequest::new().level(Level::DEBUG))
                .on_response(
                    DefaultOnResponse::new()
                        .level(Level::DEBUG)
                        .latency_unit(LatencyUnit::Millis),
                )
                .on_failure(DefaultOnFailure::new().level(Level::ERROR)),
        )
}
