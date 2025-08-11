use axum::{
    body::Body,
    http::{self, Request, StatusCode},
};
use http_body_util::BodyExt;
use serde_json::{json, Value};
use tower::ServiceExt; // for `oneshot`

mod common;

async fn create_test_collection(app: &axum::Router) -> Value {
    let client_id = "e2b1c3d4-5f6a-7b8c-9d0e-f1a2b3c4d5e6";
    let user_id = "b1c2d3e4-5f6a-7b8c-9d0e-f1a2b3c4d5e6";

    let response = app
        .clone()
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
    serde_json::from_slice(&body).unwrap()
}

#[tokio::test]
async fn test_create_collection() {
    let app = common::setup().await;
    let body = create_test_collection(&app).await;

    assert_eq!(body["title"], "Q3 2025 VAT");
    assert!(body["id"].is_string());
    assert!(body["client"].is_object());
    assert!(body["user"].is_object());
}

#[tokio::test]
async fn test_get_collection() {
    let app = common::setup().await;
    let collection = create_test_collection(&app).await;
    let collection_id = collection["id"].as_str().unwrap();

    let response = app
        .oneshot(
            Request::builder()
                .method(http::Method::GET)
                .uri(format!("/collections/{}", collection_id))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let body: Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(body["id"], collection_id);
    assert!(body["client"].is_object());
    assert!(body["user"].is_object());
}

#[tokio::test]
async fn test_get_all_collections() {
    let app = common::setup().await;
    create_test_collection(&app).await;
    create_test_collection(&app).await;

    let response = app
        .oneshot(
            Request::builder()
                .method(http::Method::GET)
                .uri("/collections")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let body: Vec<Value> = serde_json::from_slice(&body).unwrap();

    assert!(body.len() >= 2);
    assert!(body[0]["client"].is_object());
    assert!(body[0]["user"].is_object());
}

#[tokio::test]
async fn test_update_collection() {
    let app = common::setup().await;
    let collection = create_test_collection(&app).await;
    let collection_id = collection["id"].as_str().unwrap();

    let response = app
        .oneshot(
            Request::builder()
                .method(http::Method::PATCH)
                .uri(format!("/collections/{}", collection_id))
                .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .body(Body::from(
                    serde_json::to_vec(&json!({
                        "title": "Q4 2025 VAT"
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

    assert_eq!(body["title"], "Q4 2025 VAT");
    assert!(body["client"].is_object());
    assert!(body["user"].is_object());
}

#[tokio::test]
async fn test_delete_collection() {
    let app = common::setup().await;
    let collection = create_test_collection(&app).await;
    let collection_id = collection["id"].as_str().unwrap();

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method(http::Method::DELETE)
                .uri(format!("/collections/{}", collection_id))
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
                .uri(format!("/collections/{}", collection_id))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}
