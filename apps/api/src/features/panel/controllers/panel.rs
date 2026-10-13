use axum::{
    extract::Request,
    http::{StatusCode, header},
    response::{Html, IntoResponse, Response},
};
use rust_embed::RustEmbed;

use crate::core::{app_config::API_PATH, version::VERSION};

#[derive(RustEmbed)]
#[folder = "../web/dist"]
#[allow_missing = true]
struct PanelAssets;

const INDEX: &str = "index.html";

pub const VERSION_META: &str = "quickshare-version";

pub async fn asset(request: Request) -> Response {
    let path = request.uri().path().trim_start_matches('/');

    let api = API_PATH.trim_start_matches('/');

    if path == api || path.starts_with(&format!("{api}/")) {
        return StatusCode::NOT_FOUND.into_response();
    }

    if !path.is_empty()
        && path != INDEX
        && let Some(file) = PanelAssets::get(path)
    {
        let cache_control = if path.starts_with("assets/") {
            "public, max-age=31536000, immutable"
        } else {
            "no-cache"
        };

        return (
            [
                (header::CONTENT_TYPE, file.metadata.mimetype().to_string()),
                (header::CACHE_CONTROL, cache_control.to_string()),
            ],
            file.data,
        )
            .into_response();
    }

    if path
        .rsplit('/')
        .next()
        .is_some_and(|name| name.contains('.'))
    {
        return StatusCode::NOT_FOUND.into_response();
    }

    index()
}

fn index() -> Response {
    let Some(file) = PanelAssets::get(INDEX) else {
        return (StatusCode::NOT_FOUND, "Panel not built").into_response();
    };

    let html = String::from_utf8_lossy(&file.data);

    (
        [(header::CACHE_CONTROL, "no-cache")],
        Html(inject_version(&html)),
    )
        .into_response()
}

pub fn inject_version(html: &str) -> String {
    html.replacen(
        "<head>",
        &format!(
            r#"<head><meta name="{VERSION_META}" content="{}" />"#,
            VERSION
        ),
        1,
    )
}
