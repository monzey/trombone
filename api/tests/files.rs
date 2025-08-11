use axum::{
    body::Body,
    http::{self, Request, StatusCode},
};
use http_body_util::BodyExt; 
use tower::ServiceExt; 

use trombone::model::file::FileResponse;

mod common;

#[tokio::test]
async fn test_get_one_file() {
    let app = common::setup().await;

    // IDs from seed.sql
    let file_id = "f1a2b3c4-5d6e-7f8d-9f0f-f1b2d3a4b5e6";
    let request_id = "d1e2f3a4-5b6c-7d8e-9f0a-b1c2d3e4f5f6";

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
    let file_response: FileResponse = serde_json::from_slice(&body)
        .expect("Failed to deserialize FileResponse");

    // Assert top-level file fields
    assert_eq!(file_response.id.to_string(), file_id);
    assert_eq!(file_response.file_name, "default_file.txt"); // Corrected name

    // Assert nested request ID
    assert_eq!(file_response.request.id.to_string(), request_id);
}

#[tokio::test]
async fn test_get_all_files_for_request() {
    let app = common::setup().await;

    // This ID is from the seed.sql file
    let request_id = "d1e2f3a4-5b6c-7d8e-9f0a-b1c2d3e4f5f6";

    let response = app
        .oneshot(
            Request::builder()
                .method(http::Method::GET)
                .uri(format!("/requests/{}/files", request_id))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let files: Vec<FileResponse> = serde_json::from_slice(&body)
        .expect("Failed to deserialize Vec<FileResponse>");

    // The seed script creates ONE file for this request
    assert_eq!(files.len(), 1); // Corrected length

    // Spot-check the first file
    let first_file = &files[0];
    assert_eq!(first_file.request.id.to_string(), request_id);
    assert_eq!(first_file.file_name, "default_file.txt");
}

#[tokio::test]
async fn test_get_one_file_not_found() {
    let app = common::setup().await;
    let non_existent_id = "00000000-0000-0000-0000-000000000000";

    let response = app
        .oneshot(
            Request::builder()
                .method(http::Method::GET)
                .uri(format!("/files/{}", non_existent_id))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}