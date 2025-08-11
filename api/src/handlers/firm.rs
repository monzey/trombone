use crate::model::client::{Client, ClientResponse};
use crate::model::firm::{CreateFirmPayload, Firm, FirmResponse, UpdateFirmPayload};
use crate::model::user::{User, UserResponse};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use sqlx::PgPool;
use uuid::Uuid;

// GET /firms
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

// GET /firms/:id
pub async fn get_one(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<FirmResponse>, StatusCode> {
    let firm = sqlx::query_as!(Firm, "SELECT * FROM firms WHERE id = $1", id)
        .fetch_one(&pool)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    let users_raw = sqlx::query_as!(User, "SELECT * FROM users WHERE firm_id = $1", id)
        .fetch_all(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let users = users_raw.into_iter().map(|u| UserResponse { 
        id: u.id, 
        firm: firm.clone(), 
        email: u.email, 
        first_name: u.first_name, 
        last_name: u.last_name, 
        created_at: u.created_at, 
        updated_at: u.updated_at 
    }).collect();

    let clients_raw = sqlx::query_as!(Client, "SELECT * FROM clients WHERE firm_id = $1", id)
        .fetch_all(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let clients = clients_raw.into_iter().map(|c| ClientResponse {
        id: c.id,
        firm: firm.clone(),
        company_name: c.company_name,
        email: c.email,
        created_at: c.created_at,
        updated_at: c.updated_at,
    }).collect();

    let firm_response = FirmResponse {
        id: firm.id,
        name: firm.name,
        created_at: firm.created_at,
        updated_at: firm.updated_at,
        users,
        clients,
    };

    Ok(Json(firm_response))
}

// POST /firms
pub async fn create(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateFirmPayload>,
) -> Result<Json<FirmResponse>, StatusCode> {
    let firm = sqlx::query!(
        r#"
        INSERT INTO firms (name)
        VALUES ($1)
        RETURNING id
        "#,
        payload.name,
    )
    .fetch_one(&pool)
    .await
    .map_err(|e| {
        eprintln!("Failed to create firm: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    get_one(State(pool), Path(firm.id)).await
}

// PATCH /firms/:id
pub async fn update(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateFirmPayload>,
) -> Result<Json<FirmResponse>, StatusCode> {
    let firm = sqlx::query_as!(Firm, "SELECT * FROM firms WHERE id = $1", id)
        .fetch_one(&pool)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    let new_name = payload.name.unwrap_or(firm.name);

    sqlx::query!(
        "UPDATE firms SET name = $1, updated_at = now() WHERE id = $2",
        new_name,
        id
    )
    .execute(&pool)
    .await
    .map_err(|_| StatusCode::NOT_FOUND)?;

    get_one(State(pool), Path(id)).await
}

// DELETE /firms/:id
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
