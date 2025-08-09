use crate::model::client::{Client, CreateClientPayload};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use sqlx::PgPool;
use uuid::Uuid;

pub async fn create(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateClientPayload>,
) -> Result<Json<Client>, StatusCode> {
    let client = sqlx::query_as!(
        Client,
        r#"
        INSERT INTO clients (firm_id, company_name, email)
        VALUES ($1, $2, $3)
        RETURNING id, firm_id, company_name, email, created_at, updated_at
        "#,
        payload.firm_id,
        payload.company_name,
        payload.email,
    )
    .fetch_one(&pool)
    .await
    .map_err(|e| {
        eprintln!("Failed to create client: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(client))
}

pub async fn get_all(State(pool): State<PgPool>) -> Result<Json<Vec<Client>>, StatusCode> {
    let clients = sqlx::query_as!(Client, "SELECT * FROM clients")
        .fetch_all(&pool)
        .await
        .map_err(|e| {
            eprintln!("Failed to fetch clients: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(clients))
}

pub async fn get_one(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<Client>, StatusCode> {
    let client = sqlx::query_as!(Client, "SELECT * FROM clients WHERE id = $1", id)
        .fetch_one(&pool)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    Ok(Json(client))
}

pub async fn update(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
    Json(payload): Json<CreateClientPayload>, // Using Create for now
) -> Result<Json<Client>, StatusCode> {
    let client = sqlx::query_as!(
        Client,
        "UPDATE clients SET company_name = $1, email = $2 WHERE id = $3 RETURNING *",
        payload.company_name,
        payload.email,
        id
    )
    .fetch_one(&pool)
    .await
    .map_err(|_| StatusCode::NOT_FOUND)?;

    Ok(Json(client))
}

pub async fn delete(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, StatusCode> {
    let rows_affected = sqlx::query!("DELETE FROM clients WHERE id = $1", id)
        .execute(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .rows_affected();

    if rows_affected == 0 {
        return Err(StatusCode::NOT_FOUND);
    }

    Ok(StatusCode::NO_CONTENT)
}
