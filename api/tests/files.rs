use axum::{
    body::Body,
    http::{self, Request, StatusCode},
};
use http_body_util::BodyExt;
use serde_json::{json, Value};
use tower::ServiceExt; // for `oneshot`

mod common;

async fn create_test_file(app: &axum::Router) -> Value {
    let request_id = "d1e2f3a4-5b6c-7d8e-9f0a-b1c2d3e4f5f6";

    let response = app
        .clone()
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
    serde_json::from_slice(&body).unwrap()
}

#[tokio::test]
async fn test_create_file() {
    let app = common::setup().await;
    let body = create_test_file(&app).await;

    assert_eq!(body["storage_key"], "some_key");
    assert!(body["id"].is_string());
}

#[tokio::test]
async fn test_get_file() {
    let app = common::setup().await;
    let file = create_test_file(&app).await;
    let file_id = file["id"].as_str().unwrap();

    let response = app
        .oneshot(
            Request::builder()
                .method(http::Method::GET)
                .uri(format!("/files/{}", file_id))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let body: Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(body["id"], file_id);
}

#[tokio::test]
async fn test_get_all_files() {
    let app = common::setup().await;
    create_test_file(&app).await;
    create_test_file(&app).await;

    let response = app
        .oneshot(
            Request::builder()
                .method(http::Method::GET)
                .uri("/files")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let body: Vec<Value> = serde_json::from_slice(&body).unwrap();

    assert!(body.len() >= 2);
}

#[tokio::test]
async fn test_update_file() {
    let app = common::setup().await;
    let file = create_test_file(&app).await;
    let file_id = file["id"].as_str().unwrap();

    let response = app
        .oneshot(
            Request::builder()
                .method(http::Method::PATCH)
                .uri(format!("/files/{}", file_id))
                .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .body(Body::from(
                    serde_json::to_vec(&json!({
                        "storage_key": "updated_key"
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

    assert_eq!(body["storage_key"], "updated_key");
}

#[tokio::test]
async fn test_delete_file() {
    let app = common::setup().await;
    let file = create_test_file(&app).await;
    let file_id = file["id"].as_str().unwrap();

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method(http::Method::DELETE)
                .uri(format!("/files/{}", file_id))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NO_CONTENT);

    // Verify it's gone
    let response = app
        .oneshot(
            Request::builder()
                .method(http::Method::GET)
                .uri(format!("/files/{}", file_id))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}
