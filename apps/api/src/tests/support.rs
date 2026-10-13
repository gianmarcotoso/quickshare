use std::{net::SocketAddr, path::PathBuf};

use axum::{
    Router,
    body::{Body, to_bytes},
    extract::connect_info::MockConnectInfo,
    http::{Request, Response, StatusCode, header},
};
use tempfile::TempDir;
use tower::ServiceExt;

use crate::{
    app::{self, state::AppState},
    core::{app_config::AppConfig, app_config::DEFAULT_LOG_LEVEL},
    router::router,
};

pub const BOUNDARY: &str = "quickshare-test-boundary";
pub const IPHONE: &str = "Mozilla/5.0 (iPhone; CPU iPhone OS 17_5 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.5 Mobile/15E148 Safari/604.1";

pub struct TestApp {
    pub data: TempDir,
    pub state: AppState,
    pub router: Router,
}

pub fn test_config(data: &TempDir) -> AppConfig {
    AppConfig {
        bind: "127.0.0.1".to_string(),
        port: 0,
        data_path: data.path().to_path_buf(),
        max_upload_bytes: Some(1024 * 1024),
        log_level: DEFAULT_LOG_LEVEL.to_string(),
    }
}

pub fn test_app() -> TestApp {
    let data = TempDir::new().unwrap();
    let state = app::init(test_config(&data), "test-executable-sha256".to_string());
    let router = router(state.clone()).layer(MockConnectInfo(SocketAddr::from((
        [192, 168, 1, 20],
        50000,
    ))));

    TestApp {
        data,
        state,
        router,
    }
}

pub fn multipart_body(field: &str, file_name: &str, content: &[u8]) -> Vec<u8> {
    let mut body = format!(
        "--{BOUNDARY}\r\nContent-Disposition: form-data; name=\"{field}\"; filename=\"{file_name}\"\r\nContent-Type: application/octet-stream\r\n\r\n"
    )
    .into_bytes();
    body.extend_from_slice(content);
    body.extend_from_slice(format!("\r\n--{BOUNDARY}--\r\n").as_bytes());

    body
}

impl TestApp {
    pub async fn send(&self, request: Request<Body>) -> Response<Body> {
        self.router.clone().oneshot(request).await.unwrap()
    }

    pub fn bearer(&self) -> String {
        format!("Bearer {}", self.state.session().token())
    }

    pub fn post(&self, path: &str) -> Request<Body> {
        Request::post(path)
            .header(header::AUTHORIZATION, self.bearer())
            .header(header::USER_AGENT, IPHONE)
            .body(Body::empty())
            .unwrap()
    }

    pub fn upload_request(&self, batch: &str, body: Vec<u8>) -> Request<Body> {
        Request::post(format!("/api/batches/{batch}/files"))
            .header(
                header::CONTENT_TYPE,
                format!("multipart/form-data; boundary={BOUNDARY}"),
            )
            .header(header::AUTHORIZATION, self.bearer())
            .header(header::USER_AGENT, IPHONE)
            .body(Body::from(body))
            .unwrap()
    }

    pub async fn create_batch(&self) -> String {
        let response = self.send(self.post("/api/batches")).await;
        assert_eq!(response.status(), StatusCode::CREATED);

        json(response).await["id"].as_str().unwrap().to_string()
    }

    pub async fn close_batch(&self, batch: &str) -> Response<Body> {
        self.send(self.post(&format!("/api/batches/{batch}/close")))
            .await
    }

    pub fn batch_directory(&self, batch: &str) -> PathBuf {
        self.data.path().join(batch)
    }
}

pub async fn json(response: Response<Body>) -> serde_json::Value {
    serde_json::from_slice(&to_bytes(response.into_body(), usize::MAX).await.unwrap()).unwrap()
}
