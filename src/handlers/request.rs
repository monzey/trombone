use crate::model::request::{CreateRequestPayload, Request};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use sqlx::PgPool;
use uuid::Uuid;

// GET /requests
pub async fn get_all(State(pool): State<PgPool>) -> Result<Json<Vec<Request>>, StatusCode> {
    let requests = sqlx::query_as!(Request, "SELECT * FROM requests")
        .fetch_all(&pool)
        .await
        .map_err(|e| {
            eprintln!("Failed to fetch requests: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(requests))
}

// GET /requests/:id
pub async fn get_one(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<Request>, StatusCode> {
    let request = sqlx::query_as!(Request, "SELECT * FROM requests WHERE id = $1", id)
        .fetch_one(&pool)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    Ok(Json(request))
}

// POST /requests
pub async fn create(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateRequestPayload>,
) -> Result<Json<Request>, StatusCode> {
    let request = sqlx::query_as!(
        Request,
        r#"
        INSERT INTO requests (collection_id, title, description, status)
        VALUES ($1, $2, $3, 'pending')
        RETURNING id, collection_id, title, description, status, created_at, updated_at
        "#,
        payload.collection_id,
        payload.title,
        payload.description
    )
    .fetch_one(&pool)
    .await
    .map_err(|e| {
        eprintln!("Failed to create request: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(request))
}

// PATCH /requests/:id
pub async fn update(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
    Json(payload): Json<CreateRequestPayload>,
) -> Result<Json<Request>, StatusCode> {
    let request = sqlx::query_as!(
        Request,
        r#"
        UPDATE requests
        SET title = $1, description = $2, status = 'updated'
        WHERE id = $3 RETURNING *
        "#,
        payload.title,
        payload.description,
        id
    )
    .fetch_one(&pool)
    .await
    .map_err(|_| StatusCode::NOT_FOUND)?;

    Ok(Json(request))
}

// DELETE /requests/:id
pub async fn delete(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, StatusCode> {
    let rows_affected = sqlx::query!("DELETE FROM requests WHERE id = $1", id)
        .execute(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .rows_affected();

    if rows_affected == 0 {
        return Err(StatusCode::NOT_FOUND);
    }

    Ok(StatusCode::NO_CONTENT)
}
