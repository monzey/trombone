use axum::{
    body::Body,
    http::{self, Request, StatusCode},
};
use http_body_util::BodyExt;
use serde_json::{json, Value};
use tower::ServiceExt; // for `oneshot`

mod common;

#[tokio::test]
async fn test_create_collection() {
    let app = common::setup().await;
    let client_id = "e2b1c3d4-5f6a-7b8c-9d0e-f1a2b3c4d5e6";
    let user_id = "b1c2d3e4-5f6a-7b8c-9d0e-f1a2b3c4d5e6";

    let response = app
        .oneshot(
            Request::builder()
                .method(http::Method::POST)
                .uri("/collections")
                .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .body(Body::from(
                    serde_json::to_vec(&json!({
                        "client_id": client_id,
                        "user_id": user_id,
                        "title": "Q3 2025 VAT"
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

    assert_eq!(body["title"], "Q3 2025 VAT");
    assert!(body["id"].is_string());
}
