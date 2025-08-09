use crate::model::file::{CreateFilePayload, File};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use sqlx::PgPool;
use uuid::Uuid;

// GET /files
pub async fn get_all(State(pool): State<PgPool>) -> Result<Json<Vec<File>>, StatusCode> {
    let files = sqlx::query_as!(File, "SELECT * FROM files")
        .fetch_all(&pool)
        .await
        .map_err(|e| {
            eprintln!("Failed to fetch files: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(files))
}

// GET /files/:id
pub async fn get_one(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<File>, StatusCode> {
    let file = sqlx::query_as!(File, "SELECT * FROM files WHERE id = $1", id)
        .fetch_one(&pool)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    Ok(Json(file))
}

// POST /files
pub async fn create(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateFilePayload>,
) -> Result<Json<File>, StatusCode> {
    let file = sqlx::query_as!(
        File,
        r#"
        INSERT INTO files (request_id, storage_key, file_size, mime_type)
        VALUES ($1, $2, $3, $4)
        RETURNING id, request_id, storage_key, file_size, mime_type, created_at, updated_at
        "#,
        payload.request_id,
        payload.storage_key,
        payload.file_size,
        payload.mime_type,
    )
    .fetch_one(&pool)
    .await
    .map_err(|e| {
        eprintln!("Failed to create file: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(file))
}

// PATCH /files/:id
pub async fn update(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
    Json(payload): Json<CreateFilePayload>, // Placeholder
) -> Result<Json<File>, StatusCode> {
    // Placeholder implementation
    let file = sqlx::query_as!(
        File,
        "UPDATE files SET storage_key = $1 WHERE id = $2 RETURNING *",
        "new_placeholder_key", // placeholder
        id
    )
    .fetch_one(&pool)
    .await
    .map_err(|_| StatusCode::NOT_FOUND)?;

    Ok(Json(file))
}

// DELETE /files/:id
pub async fn delete(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, StatusCode> {
    let rows_affected = sqlx::query!("DELETE FROM files WHERE id = $1", id)
        .execute(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .rows_affected();

    if rows_affected == 0 {
        return Err(StatusCode::NOT_FOUND);
    }

    Ok(StatusCode::NO_CONTENT)
}
