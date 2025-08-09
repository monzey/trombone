use axum::{
    body::Body,
    http::{self, Request, StatusCode},
};
use http_body_util::BodyExt;
use serde_json::{json, Value};
use tower::ServiceExt; // for `oneshot`

mod common;

use uuid::Uuid;

async fn create_test_user(app: &axum::Router) -> Value {
    let firm_id = "a6a7572a-5553-4653-a733-35a0b602790f"; // From seed.sql
    let email = format!("test.user+{}@example.com", Uuid::new_v4());
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method(http::Method::POST)
                .uri("/users")
                .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .body(Body::from(
                    serde_json::to_vec(&json!({
                        "firm_id": firm_id,
                        "email": email,
                        "password": "password123",
                        "first_name": "Test",
                        "last_name": "User"
                    }))
                    .unwrap(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    let body = response.into_body().collect().await.unwrap().to_bytes();

    println!("Response body: {}", String::from_utf8_lossy(&body));
    serde_json::from_slice(&body).unwrap()
}

#[tokio::test]
async fn test_create_user() {
    let app = common::setup().await;
    let body = create_test_user(&app).await;

    assert!(
        body["email"].as_str().unwrap().starts_with("test.user+")
            && body["email"].as_str().unwrap().ends_with("@example.com")
    );
    assert_eq!(body["first_name"], "Test");
    assert_eq!(body["last_name"], "User");
    assert!(body["id"].is_string());
}

#[tokio::test]
async fn test_get_user() {
    let app = common::setup().await;
    let user = create_test_user(&app).await;
    let user_id = user["id"].as_str().unwrap();

    let response = app
        .oneshot(
            Request::builder()
                .method(http::Method::GET)
                .uri(format!("/users/{}", user_id))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let body: Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(body["id"], user_id);
}

#[tokio::test]
async fn test_update_user() {
    let app = common::setup().await;
    let user = create_test_user(&app).await;
    let user_id = user["id"].as_str().unwrap();

    print!("{}", user_id);

    let updated_email = format!("updated.user+{}@example.com", Uuid::new_v4());

    let response = app
        .oneshot(
            Request::builder()
                .method(http::Method::PATCH)
                .uri(format!("/users/{}", user_id))
                .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .body(Body::from(
                    serde_json::to_vec(&json!({
                        "first_name": "UpdatedFirst",
                        "last_name": "UpdatedLast",
                        "email": updated_email
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

    assert_eq!(body["first_name"], "UpdatedFirst");
    assert_eq!(body["last_name"], "UpdatedLast");
    assert_eq!(body["email"], updated_email);
}

#[tokio::test]
async fn test_delete_user() {
    let app = common::setup().await;
    let user = create_test_user(&app).await;
    let user_id = user["id"].as_str().unwrap();

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method(http::Method::DELETE)
                .uri(format!("/users/{}", user_id))
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
                .uri(format!("/users/{}", user_id))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}
