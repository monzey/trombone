
use axum::{
    body::Body,
    http::{self, Request, StatusCode},
};
use http_body_util::BodyExt;
use serde_json::{json, Value};
use tower::ServiceExt; // for `oneshot`

mod common;

async fn create_test_firm(app: &axum::Router) -> Value {
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method(http::Method::POST)
                .uri("/firms")
                .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .body(Body::from(
                    serde_json::to_vec(&json!({ "name": "Another Test Firm" })).unwrap(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    let body = response.into_body().collect().await.unwrap().to_bytes();
    serde_json::from_slice(&body).unwrap()
}

#[tokio::test]
async fn test_create_firm() {
    let app = common::setup().await;
    let body = create_test_firm(&app).await;
    assert_eq!(body["name"], "Another Test Firm");
    assert!(body["id"].is_string());
}

#[tokio::test]
async fn test_get_firm() {
    let app = common::setup().await;
    let firm = create_test_firm(&app).await;
    let firm_id = firm["id"].as_str().unwrap();

    let response = app
        .oneshot(
            Request::builder()
                .method(http::Method::GET)
                .uri(format!("/firms/{}", firm_id))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let body: Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(body["id"], firm_id);
}

#[tokio::test]
async fn test_update_firm() {
    let app = common::setup().await;
    let firm = create_test_firm(&app).await;
    let firm_id = firm["id"].as_str().unwrap();

    let response = app
        .oneshot(
            Request::builder()
                .method(http::Method::PATCH)
                .uri(format!("/firms/{}", firm_id))
                .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .body(Body::from(
                    serde_json::to_vec(&json!({ "name": "Updated Test Firm" })).unwrap(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let body: Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(body["name"], "Updated Test Firm");
}

#[tokio::test]
async fn test_delete_firm() {
    let app = common::setup().await;
    let firm = create_test_firm(&app).await;
    let firm_id = firm["id"].as_str().unwrap();

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method(http::Method::DELETE)
                .uri(format!("/firms/{}", firm_id))
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
                .uri(format!("/firms/{}", firm_id))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}
