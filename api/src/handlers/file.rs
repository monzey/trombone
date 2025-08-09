use crate::model::file::{CreateFilePayload, UpdateFilePayload, File};
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
    Json(payload): Json<UpdateFilePayload>,
) -> Result<Json<File>, StatusCode> {
    let mut file = sqlx::query_as!(File, "SELECT * FROM files WHERE id = $1", id)
        .fetch_one(&pool)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    if let Some(request_id) = payload.request_id {
        file.request_id = request_id;
    }

    if let Some(storage_key) = payload.storage_key {
        file.storage_key = storage_key;
    }

    if let Some(file_size) = payload.file_size {
        file.file_size = file_size;
    }

    if let Some(mime_type) = payload.mime_type {
        file.mime_type = mime_type;
    }

    let file = sqlx::query_as!(
        File,
        r#"
        UPDATE files
        SET request_id = $1, storage_key = $2, file_size = $3, mime_type = $4
        WHERE id = $5
        RETURNING id, request_id, storage_key, file_size, mime_type, created_at, updated_at
        "#,
        file.request_id,
        file.storage_key,
        file.file_size,
        file.mime_type,
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
