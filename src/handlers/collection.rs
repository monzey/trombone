use crate::model::collection::{Collection, CreateCollectionPayload, UpdateCollectionPayload};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use sqlx::PgPool;
use uuid::Uuid;

// GET /collections
pub async fn get_all(State(pool): State<PgPool>) -> Result<Json<Vec<Collection>>, StatusCode> {
    let collections = sqlx::query_as!(Collection, "SELECT * FROM collections")
        .fetch_all(&pool)
        .await
        .map_err(|e| {
            eprintln!("Failed to fetch collections: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(collections))
}

// GET /collections/:id
pub async fn get_one(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<Collection>, StatusCode> {
    let collection = sqlx::query_as!(Collection, "SELECT * FROM collections WHERE id = $1", id)
        .fetch_one(&pool)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    Ok(Json(collection))
}

// POST /collections
pub async fn create(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateCollectionPayload>,
) -> Result<Json<Collection>, StatusCode> {
    let collection = sqlx::query_as!(
        Collection,
        r#"
        INSERT INTO collections (client_id, user_id, title, status, access_token, expires_at)
        VALUES ($1, $2, $3, 'pending', 'token', now() + interval '1 day')
        RETURNING id, client_id, user_id, title, status, access_token, expires_at, created_at, updated_at
        "#,
        payload.client_id,
        payload.user_id,
        payload.title,
    )
    .fetch_one(&pool)
    .await
    .map_err(|e| {
        eprintln!("Failed to create collection: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(collection))
}

// PATCH /collections/:id
pub async fn update(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateCollectionPayload>,
) -> Result<Json<Collection>, StatusCode> {
    let collection = sqlx::query_as!(
        Collection,
        r#"
        UPDATE collections
        SET title = $1, updated_at = now(), status = $2
        WHERE id = $3 RETURNING *
        "#,
        payload.title,
        payload.status,
        id
    )
    .fetch_one(&pool)
    .await
    .map_err(|_| StatusCode::NOT_FOUND)?;

    Ok(Json(collection))
}

// DELETE /collections/:id
pub async fn delete(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, StatusCode> {
    let rows_affected = sqlx::query!("DELETE FROM collections WHERE id = $1", id)
        .execute(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .rows_affected();

    if rows_affected == 0 {
        return Err(StatusCode::NOT_FOUND);
    }

    Ok(StatusCode::NO_CONTENT)
}
