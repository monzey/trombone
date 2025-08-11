use crate::model::client::{Client, ClientResponse, CreateClientPayload, UpdateClientPayload};
use crate::model::firm::Firm;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use uuid::Uuid;

use crate::app_state::AppState;

pub async fn create(
    State(app_state): State<AppState>,
    Json(payload): Json<CreateClientPayload>,
) -> Result<Json<ClientResponse>, StatusCode> {
    let client = sqlx::query!(
        r#"
        INSERT INTO clients (firm_id, company_name, email)
        VALUES ($1, $2, $3)
        RETURNING id
        "#,
        payload.firm_id,
        payload.company_name,
        payload.email,
    )
    .fetch_one(&app_state.db_pool)
    .await
    .map_err(|e| {
        eprintln!("Failed to create client: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    get_one(State(app_state), Path(client.id)).await
}

pub async fn get_all(
    State(app_state): State<AppState>,
) -> Result<Json<Vec<ClientResponse>>, StatusCode> {
    let clients = sqlx::query!(
        r#"
        SELECT
            c.id, c.company_name, c.email, c.created_at, c.updated_at,
            f.id as "firm_id", f.name as "firm_name", f.created_at as "firm_created_at", f.updated_at as "firm_updated_at"
        FROM clients c
        JOIN firms f ON c.firm_id = f.id
        "#
    )
    .fetch_all(&app_state.db_pool)
    .await
    .map_err(|e| {
        eprintln!("Failed to fetch clients: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let client_responses = clients
        .into_iter()
        .map(|row| ClientResponse {
            id: row.id,
            company_name: row.company_name,
            email: row.email,
            created_at: row.created_at,
            updated_at: row.updated_at,
            firm: Firm {
                id: row.firm_id,
                name: row.firm_name,
                created_at: row.firm_created_at,
                updated_at: row.firm_updated_at,
            },
        })
        .collect();

    Ok(Json(client_responses))
}

pub async fn get_one(
    State(app_state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<ClientResponse>, StatusCode> {
    let client = sqlx::query!(
        r#"
        SELECT
            c.id, c.company_name, c.email, c.created_at, c.updated_at,
            f.id as "firm_id", f.name as "firm_name", f.created_at as "firm_created_at", f.updated_at as "firm_updated_at"
        FROM clients c
        JOIN firms f ON c.firm_id = f.id
        WHERE c.id = $1
        "#,
        id
    )
    .fetch_one(&app_state.db_pool)
    .await
    .map_err(|_| StatusCode::NOT_FOUND)?;

    let client_response = ClientResponse {
        id: client.id,
        company_name: client.company_name,
        email: client.email,
        created_at: client.created_at,
        updated_at: client.updated_at,
        firm: Firm {
            id: client.firm_id,
            name: client.firm_name,
            created_at: client.firm_created_at,
            updated_at: client.firm_updated_at,
        },
    };

    Ok(Json(client_response))
}

pub async fn update(
    State(app_state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateClientPayload>,
) -> Result<Json<ClientResponse>, StatusCode> {
    let mut client = sqlx::query_as!(Client, "SELECT * FROM clients WHERE id = $1", id)
        .fetch_one(&app_state.db_pool)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    if let Some(company_name) = payload.company_name {
        client.company_name = company_name;
    }

    if let Some(email) = payload.email {
        client.email = email;
    }

    sqlx::query!(
        r#"
        UPDATE clients
        SET company_name = $1, email = $2, updated_at = now()
        WHERE id = $3
        "#,
        client.company_name,
        client.email,
        id
    )
    .execute(&app_state.db_pool)
    .await
    .map_err(|_| StatusCode::NOT_FOUND)?;

    get_one(State(app_state), Path(id)).await
}

pub async fn delete(
    State(app_state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, StatusCode> {
    let rows_affected = sqlx::query!("DELETE FROM clients WHERE id = $1", id)
        .execute(&app_state.db_pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .rows_affected();

    if rows_affected == 0 {
        return Err(StatusCode::NOT_FOUND);
    }

    Ok(StatusCode::NO_CONTENT)
}

