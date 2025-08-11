use crate::handlers::request as request_handler;
use crate::model::file::{File, FileResponse};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use uuid::Uuid;

use crate::app_state::AppState;

// GET /requests/:request_id/files
pub async fn get_all_for_request(
    State(app_state): State<AppState>,
    Path(request_id): Path<Uuid>,
) -> Result<Json<Vec<FileResponse>>, StatusCode> {
    let request_response = request_handler::get_one(State(app_state.clone()), Path(request_id))
        .await? // Ensure the request exists
        .0;

    let files = sqlx::query_as!(File, "SELECT id, request_id, file_name, storage_key, file_size, mime_type, created_at, updated_at FROM files WHERE request_id = $1", request_id)
        .fetch_all(&app_state.db_pool)
        .await
        .map_err(|e| {
            eprintln!("Failed to fetch files for request: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let file_responses = files
        .into_iter()
        .map(|file| FileResponse {
            id: file.id,
            request: request_response.clone(), // Clone the fetched RequestResponse for each file
            file_name: file.file_name,
            storage_key: file.storage_key,
            file_size: file.file_size,
            mime_type: file.mime_type,
            created_at: file.created_at,
            updated_at: file.updated_at,
        })
        .collect();

    Ok(Json(file_responses))
}

// GET /files/:id
pub async fn get_one(
    State(app_state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<FileResponse>, StatusCode> {
    let file = sqlx::query_as!(File, "SELECT id, request_id, file_name, storage_key, file_size, mime_type, created_at, updated_at FROM files WHERE id = $1", id)
        .fetch_one(&app_state.db_pool)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    let request_response = request_handler::get_one(State(app_state), Path(file.request_id))
        .await?
        .0;

    let file_response = FileResponse {
        id: file.id,
        request: request_response,
        file_name: file.file_name,
        storage_key: file.storage_key,
        file_size: file.file_size,
        mime_type: file.mime_type,
        created_at: file.created_at,
        updated_at: file.updated_at,
    };

    Ok(Json(file_response))
}

// POST /requests/:request_id/files - Placeholder for file upload
pub async fn upload(
    Path(request_id): Path<Uuid>,
    // Multipart form data would be extracted here
) -> Result<Json<FileResponse>, StatusCode> {
    // 1. Process multipart form data, save file to storage (e.g., S3, local disk)
    // 2. Create a `File` record in the database with metadata
    // 3. Return the `FileResponse` by calling `get_one`
    eprintln!("Placeholder for file upload for request {}", request_id);
    Err(StatusCode::NOT_IMPLEMENTED)
}

// DELETE /files/:id - Placeholder for file deletion
pub async fn delete(Path(id): Path<Uuid>) -> Result<StatusCode, StatusCode> {
    // 1. Delete file from storage
    // 2. Delete `File` record from the database
    eprintln!("Placeholder for file deletion for file {}", id);
    Err(StatusCode::NOT_IMPLEMENTED)
}
