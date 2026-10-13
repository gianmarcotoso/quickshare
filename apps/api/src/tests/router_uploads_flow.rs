use axum::{
    body::Body,
    http::{Request, StatusCode, header},
};
use sha2::{Digest, Sha256};

use crate::{
    features::uploads::types::batch::BATCH_LOG,
    tests::support::{json, multipart_body, test_app},
};

#[tokio::test]
async fn requests_without_the_session_token_are_rejected() {
    let app = test_app();

    for authorization in [
        None,
        Some("Bearer wrong"),
        Some(app.state.session().token()),
    ] {
        let mut request = Request::post("/api/batches");
        if let Some(value) = authorization {
            request = request.header(header::AUTHORIZATION, value);
        }

        let response = app.send(request.body(Body::empty()).unwrap()).await;

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
        assert_eq!(json(response).await["code"], "ERR_INVALID_TOKEN");
    }

    let response = app
        .send(
            Request::get("/api/session")
                .header(header::AUTHORIZATION, app.bearer())
                .body(Body::empty())
                .unwrap(),
        )
        .await;
    assert_eq!(response.status(), StatusCode::NO_CONTENT);
}

#[tokio::test]
async fn each_batch_gets_its_own_folder_named_after_its_start_time() {
    let app = test_app();
    let first = app.create_batch().await;
    let second = app.create_batch().await;

    assert_ne!(first, second);

    for batch in [&first, &second] {
        assert!(app.batch_directory(batch).join(BATCH_LOG).is_file());

        let parts: Vec<&str> = batch.split('_').collect();
        assert!(parts.len() == 6 || parts.len() == 7, "{batch}");
        assert_eq!(parts[0].len(), 4);
        assert!(
            parts[1..6]
                .iter()
                .all(|part| part.len() == 2 && part.chars().all(|c| c.is_ascii_digit()))
        );
    }
}

#[tokio::test]
async fn a_file_is_saved_in_the_batch_folder_with_its_hash() {
    let app = test_app();
    let batch = app.create_batch().await;
    let content = b"hello quickshare";

    let response = app
        .send(app.upload_request(&batch, multipart_body("file", "hello.txt", content)))
        .await;

    assert_eq!(response.status(), StatusCode::CREATED);

    let body = json(response).await;
    let sha256 = hex::encode(Sha256::digest(content));

    assert_eq!(body["name"], "hello.txt");
    assert_eq!(body["size"], content.len());
    assert_eq!(body["sha256"], sha256);

    let directory = app.batch_directory(&batch);
    assert_eq!(std::fs::read(directory.join("hello.txt")).unwrap(), content);

    let log = std::fs::read_to_string(directory.join(BATCH_LOG)).unwrap();
    assert!(log.contains(&format!(
        "session token:     {}",
        app.state.session().token()
    )));
    assert!(log.contains("session started:   "));
    assert!(log.contains("executable sha256: test-executable-sha256"));
    assert!(log.contains("hello.txt"));
    assert!(log.contains(&sha256));
    assert!(log.contains("16 B (16 bytes)"));
    assert!(log.contains("smartphone · iPhone 17.5 · Safari 17.5"));
    assert!(log.contains("192.168.1.20"));
}

#[tokio::test]
async fn closing_a_batch_seals_its_log_with_the_hash_of_the_previous_lines() {
    let app = test_app();
    let batch = app.create_batch().await;

    app.send(app.upload_request(&batch, multipart_body("file", "a.txt", b"abc")))
        .await;

    assert_eq!(
        app.close_batch(&batch).await.status(),
        StatusCode::NO_CONTENT
    );

    let log = std::fs::read_to_string(app.batch_directory(&batch).join(BATCH_LOG)).unwrap();
    let (sealed, last_line) = log.trim_end().rsplit_once('\n').unwrap();
    let expected = hex::encode(Sha256::digest(format!("{sealed}\n").as_bytes()));

    assert!(sealed.contains("batch closed: 1 files, 3 B (3 bytes)"));
    assert_eq!(
        last_line,
        format!("log sha256 (all previous lines): {expected}")
    );

    let response = app
        .send(app.upload_request(&batch, multipart_body("file", "b.txt", b"x")))
        .await;
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
    assert_eq!(json(response).await["code"], "ERR_BATCH_NOT_FOUND");
    assert_eq!(
        app.close_batch(&batch).await.status(),
        StatusCode::NOT_FOUND
    );
}

#[tokio::test]
async fn files_with_the_same_name_are_not_overwritten() {
    let app = test_app();
    let batch = app.create_batch().await;

    for content in [b"first".as_slice(), b"second".as_slice()] {
        let response = app
            .send(app.upload_request(&batch, multipart_body("file", "same.txt", content)))
            .await;
        assert_eq!(response.status(), StatusCode::CREATED);
    }

    let directory = app.batch_directory(&batch);
    assert_eq!(std::fs::read(directory.join("same.txt")).unwrap(), b"first");
    assert_eq!(
        std::fs::read(directory.join("same (1).txt")).unwrap(),
        b"second"
    );

    let log = std::fs::read_to_string(directory.join(BATCH_LOG)).unwrap();
    assert!(log.contains("original:   same.txt"));
}

#[tokio::test]
async fn file_names_cannot_escape_the_batch_folder() {
    let app = test_app();
    let batch = app.create_batch().await;

    let response = app
        .send(app.upload_request(&batch, multipart_body("file", "../../escape.txt", b"x")))
        .await;

    assert_eq!(response.status(), StatusCode::CREATED);
    assert_eq!(json(response).await["name"], "escape.txt");
    assert!(app.batch_directory(&batch).join("escape.txt").exists());
    assert!(!app.data.path().join("escape.txt").exists());
}

#[tokio::test]
async fn uploads_to_unknown_batches_are_rejected() {
    let app = test_app();

    let response = app
        .send(app.upload_request("../outside", multipart_body("file", "a.txt", b"x")))
        .await;

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn a_request_without_a_file_is_rejected() {
    let app = test_app();
    let batch = app.create_batch().await;

    let response = app
        .send(app.upload_request(&batch, multipart_body("other", "a.txt", b"x")))
        .await;

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    assert_eq!(json(response).await["code"], "ERR_FILE_MISSING");
}

#[tokio::test]
async fn files_over_the_limit_are_rejected_and_not_kept() {
    let app = test_app();
    let batch = app.create_batch().await;
    let content = vec![0u8; 2 * 1024 * 1024];

    let response = app
        .send(app.upload_request(&batch, multipart_body("file", "big.bin", &content)))
        .await;

    assert_eq!(response.status(), StatusCode::PAYLOAD_TOO_LARGE);
    assert_eq!(json(response).await["code"], "ERR_FILE_TOO_LARGE");

    assert!(!app.batch_directory(&batch).join("big.bin").exists());
}
