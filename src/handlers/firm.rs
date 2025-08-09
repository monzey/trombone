use crate::app_error::AppError;
use crate::model::firm::{CreateFirmPayload, Firm};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use sqlx::PgPool;
use uuid::Uuid;

pub async fn create(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateFirmPayload>,
) -> Result<Json<Firm>, AppError> {
    let firm = sqlx::query_as!(
        Firm,
        r#"
        INSERT INTO firms (name)
        VALUES ($1)
        RETURNING id, name, created_at, updated_at
        "#,
        payload.name,
    )
    .fetch_one(&pool)
    .await
    .map_err(|e| {
        eprintln!("Error inserting firm : {}", e);
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Error creating firm.")
    })?;

    Ok(Json(firm))
}

pub async fn get_all(State(pool): State<PgPool>) -> Result<Json<Vec<Firm>>, StatusCode> {
    let firms = sqlx::query_as!(Firm, "SELECT * FROM firms")
        .fetch_all(&pool)
        .await
        .map_err(|e| {
            eprintln!("Failed to fetch firms: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(firms))
}

pub async fn get_one(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<Firm>, StatusCode> {
    let firm = sqlx::query_as!(Firm, "SELECT * FROM firms WHERE id = $1", id)
        .fetch_one(&pool)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    Ok(Json(firm))
}

pub async fn update(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
    Json(payload): Json<CreateFirmPayload>, // Using Create for now
) -> Result<Json<Firm>, StatusCode> {
    let firm = sqlx::query_as!(
        Firm,
        "UPDATE firms SET name = $1 WHERE id = $2 RETURNING *",
        payload.name,
        id
    )
    .fetch_one(&pool)
    .await
    .map_err(|_| StatusCode::NOT_FOUND)?;

    Ok(Json(firm))
}

pub async fn delete(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, StatusCode> {
    let rows_affected = sqlx::query!("DELETE FROM firms WHERE id = $1", id)
        .execute(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .rows_affected();

    if rows_affected == 0 {
        return Err(StatusCode::NOT_FOUND);
    }

    Ok(StatusCode::NO_CONTENT)
}
