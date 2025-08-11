use axum::{
    body::Body,
    http::{self, Request, StatusCode},
};
use http_body_util::BodyExt;
use serde_json::{json, Value};
use tower::ServiceExt; // for `oneshot`

mod common;

async fn create_test_client(app: &axum::Router, token: &str) -> Value {
    let firm_id = "a6a7572a-5553-4653-a733-35a0b602790f"; // From seed.sql
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method(http::Method::POST)
                .uri("/clients")
                .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .header(http::header::AUTHORIZATION, format!("Bearer {}", token)) // Add Authorization header
                .body(Body::from(
                    serde_json::to_vec(&json!({
                        "firm_id": firm_id,
                        "company_name": "Test Client Company",
                        "email": "client@example.com"
                    }))
                    .unwrap(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    let status = response.status();
    let body = response.into_body().collect().await.unwrap().to_bytes();
    println!("Response Status: {}", status);
    println!("Response Body: {}", String::from_utf8_lossy(&body));
    assert!(status.is_success(), "Expected a successful status code, got: {}", status);
    serde_json::from_slice(&body).unwrap()
}

#[tokio::test]
async fn test_create_client() {
    let (app, token) = common::setup().await; // Get token from setup
    let body = create_test_client(&app, &token).await; // Pass token to create_test_client

    assert_eq!(body["company_name"], "Test Client Company");
    assert_eq!(body["email"], "client@example.com");
    assert!(body["id"].is_string());
}

#[tokio::test]
async fn test_get_client() {
    let (app, token) = common::setup().await; // Get token from setup
    let client = create_test_client(&app, &token).await;
    let client_id = client["id"].as_str().unwrap();

    let response = app
        .oneshot(
            Request::builder()
                .method(http::Method::GET)
                .uri(format!("/clients/{}", client_id))
                .header(http::header::AUTHORIZATION, format!("Bearer {}", token)) // Add Authorization header
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let body: Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(body["id"], client_id);
}

#[tokio::test]
async fn test_update_client() {
    let (app, token) = common::setup().await; // Get token from setup
    let client = create_test_client(&app, &token).await;
    let client_id = client["id"].as_str().unwrap();

    let response = app
        .oneshot(
            Request::builder()
                .method(http::Method::PATCH)
                .uri(format!("/clients/{}", client_id))
                .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .header(http::header::AUTHORIZATION, format!("Bearer {}", token)) // Add Authorization header
                .body(Body::from(
                    serde_json::to_vec(&json!({
                        "firm_id": client["firm_id"],
                        "company_name": "Updated Client Company",
                        "email": "updated.client@example.com"
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

    assert_eq!(body["company_name"], "Updated Client Company");
    assert_eq!(body["email"], "updated.client@example.com");
}

#[tokio::test]
async fn test_delete_client() {
    let (app, token) = common::setup().await; // Get token from setup
    let client = create_test_client(&app, &token).await;
    let client_id = client["id"].as_str().unwrap();

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method(http::Method::DELETE)
                .uri(format!("/clients/{}", client_id))
                .header(http::header::AUTHORIZATION, format!("Bearer {}", token)) // Add Authorization header
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
                .uri(format!("/clients/{}", client_id))
                .header(http::header::AUTHORIZATION, format!("Bearer {}", token)) // Add Authorization header
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}
