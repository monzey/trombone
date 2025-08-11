use crate::app_error::AppError;
use crate::model::client::{Client, ClientResponse};
use crate::model::firm::{CreateFirmPayload, Firm, FirmResponse, UpdateFirmPayload};
use crate::model::user::{User, UserResponse};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use uuid::Uuid;

use crate::app_state::AppState;

pub async fn create(
    State(app_state): State<AppState>,
    Json(payload): Json<CreateFirmPayload>,
) -> Result<Json<FirmResponse>, AppError> {
    let firm = sqlx::query!(
        r#"
        INSERT INTO firms (name)
        VALUES ($1)
        RETURNING id
        "#,
        payload.name,
    )
    .fetch_one(&app_state.db_pool)
    .await
    .map_err(|e| {
        eprintln!("Error inserting firm : {}", e);
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Error creating firm.")
    })?;

    get_one(State(app_state), Path(firm.id)).await
}

pub async fn get_all(State(app_state): State<AppState>) -> Result<Json<Vec<Firm>>, StatusCode> {
    let firms = sqlx::query_as!(Firm, "SELECT * FROM firms")
        .fetch_all(&app_state.db_pool)
        .await
        .map_err(|e| {
            eprintln!("Failed to fetch firms: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(firms))
}

pub async fn get_one(
    State(app_state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<FirmResponse>, AppError> {
    // Changed return type to AppError
    let firm = sqlx::query_as!(Firm, "SELECT * FROM firms WHERE id = $1", id)
        .fetch_one(&app_state.db_pool)
        .await
        .map_err(|_| AppError::new(StatusCode::NOT_FOUND, "Firm not found"))?;

    let users_raw = sqlx::query_as!(User, "SELECT * FROM users WHERE firm_id = $1", id)
        .fetch_all(&app_state.db_pool)
        .await
        .map_err(|_| {
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to fetch users for firm",
            )
        })?;

    let users = users_raw
        .into_iter()
        .map(|u| UserResponse {
            id: u.id,
            firm: Some(firm.clone()),
            email: u.email,
            first_name: u.first_name,
            last_name: u.last_name,
            created_at: u.created_at,
            updated_at: u.updated_at,
        })
        .collect();

    let clients_raw = sqlx::query_as!(Client, "SELECT * FROM clients WHERE firm_id = $1", id)
        .fetch_all(&app_state.db_pool)
        .await
        .map_err(|_| {
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to fetch clients for firm",
            )
        })?;

    let clients = clients_raw
        .into_iter()
        .map(|c| ClientResponse {
            id: c.id,
            firm: firm.clone(),
            company_name: c.company_name,
            email: c.email,
            created_at: c.created_at,
            updated_at: c.updated_at,
        })
        .collect();

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

pub async fn update(
    State(app_state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateFirmPayload>,
) -> Result<Json<FirmResponse>, AppError> {
    // Changed return type to AppError
    let firm = sqlx::query_as!(Firm, "SELECT * FROM firms WHERE id = $1", id)
        .fetch_one(&app_state.db_pool)
        .await
        .map_err(|_| AppError::new(StatusCode::NOT_FOUND, "Firm not found"))?;

    let new_name = payload.name.unwrap_or(firm.name);

    sqlx::query!(
        "UPDATE firms SET name = $1, updated_at = now() WHERE id = $2",
        new_name,
        id
    )
    .execute(&app_state.db_pool)
    .await
    .map_err(|_| AppError::new(StatusCode::NOT_FOUND, "Firm not found"))?;

    get_one(State(app_state), Path(id)).await
}

pub async fn delete(
    State(app_state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    // Changed return type to AppError
    let rows_affected = sqlx::query!("DELETE FROM firms WHERE id = $1", id)
        .execute(&app_state.db_pool)
        .await
        .map_err(|_| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Error deleting firm"))?
        .rows_affected();

    if rows_affected == 0 {
        return Err(AppError::new(StatusCode::NOT_FOUND, "Firm not found"));
    }

    Ok(StatusCode::NO_CONTENT)
}

