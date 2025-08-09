use axum::{
    body::Body,
    http::{self, Request, StatusCode},
};
use http_body_util::BodyExt;
use serde_json::{json, Value};
use tower::ServiceExt; // for `oneshot`

mod common;

#[tokio::test]
async fn test_create_file() {
    let app = common::setup().await;
    let request_id = "d1e2f3a4-5b6c-7d8e-9f0a-b1c2d3e4f5f6";

    let response = app
        .oneshot(
            Request::builder()
                .method(http::Method::POST)
                .uri("/files")
                .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .body(Body::from(
                    serde_json::to_vec(&json!({
                        "request_id": request_id,
                        "storage_key": "some_key",
                        "file_size": 12345,
                        "mime_type": "application/pdf"
                    }))
                    .unwrap(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let body: Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(body["storage_key"], "some_key");
    assert!(body["id"].is_string());
}
