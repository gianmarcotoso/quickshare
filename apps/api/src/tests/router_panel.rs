use axum::{
    body::Body,
    http::{Request, StatusCode},
};

use crate::tests::support::{json, test_app};

#[tokio::test]
async fn the_openapi_spec_is_served_under_the_api() {
    let app = test_app();

    let response = app
        .send(
            Request::get("/api/openapi.json")
                .body(Body::empty())
                .unwrap(),
        )
        .await;

    assert_eq!(response.status(), StatusCode::OK);
    assert!(json(response).await["paths"]["/batches/{id}/files"]["post"].is_object());
}

#[tokio::test]
async fn unknown_api_paths_and_missing_assets_are_not_found() {
    let app = test_app();

    for path in ["/api", "/api/unknown", "/missing.js"] {
        let response = app
            .send(Request::get(path).body(Body::empty()).unwrap())
            .await;

        assert_eq!(response.status(), StatusCode::NOT_FOUND, "{path}");
    }
}
