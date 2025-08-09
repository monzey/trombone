use axum::{
    body::Body,
    http::{self, Request, StatusCode},
};
use http_body_util::BodyExt;
use serde_json::{json, Value};
use tower::ServiceExt; // for `oneshot`

mod common;

#[tokio::test]
async fn test_create_request() {
    let app = common::setup().await;
    let collection_id = "c1d2e3f4-5a6b-7c8d-9e0f-a1b2c3d4e5f6";

    let response = app
        .oneshot(
            Request::builder()
                .method(http::Method::POST)
                .uri("/requests")
                .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .body(Body::from(
                    serde_json::to_vec(&json!({
                        "collection_id": collection_id,
                        "title": "Sales Invoices for July",
                        "description": "Please upload all sales invoices for the month of July."
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

    assert_eq!(body["title"], "Sales Invoices for July");
    assert!(body["id"].is_string());
}
