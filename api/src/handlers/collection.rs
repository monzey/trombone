use crate::model::client::ClientResponse;
use crate::model::collection::{
    Collection, CollectionResponse, CreateCollectionPayload, UpdateCollectionPayload,
};
use crate::model::firm::Firm;
use crate::model::user::UserResponse;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use uuid::Uuid;

use crate::app_state::AppState;

// GET /collections
pub async fn get_all(
    State(app_state): State<AppState>,
) -> Result<Json<Vec<CollectionResponse>>, StatusCode> {
    let records = sqlx::query!(
        r#"
        SELECT
            c.id as collection_id, c.title, c.status, c.access_token, c.expires_at, c.created_at as collection_created_at, c.updated_at as collection_updated_at,
            cl.id as client_id, cl.company_name, cl.email as client_email, cl.created_at as client_created_at, cl.updated_at as client_updated_at,
            u.id as user_id, u.email as user_email, u.first_name, u.last_name, u.created_at as user_created_at, u.updated_at as user_updated_at,
            f_cl.id as "client_firm_id", f_cl.name as "client_firm_name", f_cl.created_at as "client_firm_created_at", f_cl.updated_at as "client_firm_updated_at",
            f_u.id as "user_firm_id", f_u.name as "user_firm_name", f_u.created_at as "user_firm_created_at", f_u.updated_at as "user_firm_updated_at"
        FROM collections c
        JOIN clients cl ON c.client_id = cl.id
        JOIN users u ON c.user_id = u.id
        JOIN firms f_cl ON cl.firm_id = f_cl.id
        JOIN firms f_u ON u.firm_id = f_u.id
        "#
    )
    .fetch_all(&app_state.db_pool)
    .await
    .map_err(|e| {
        eprintln!("Failed to fetch collections: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let responses = records
        .into_iter()
        .map(|row| {
            let client_firm = Firm {
                id: row.client_firm_id,
                name: row.client_firm_name,
                created_at: row.client_firm_created_at,
                updated_at: row.client_firm_updated_at,
            };
            let client = ClientResponse {
                id: row.client_id,
                firm: client_firm,
                company_name: row.company_name,
                email: row.client_email,
                created_at: row.client_created_at,
                updated_at: row.client_updated_at,
            };

            let user_firm = Firm {
                id: row.user_firm_id,
                name: row.user_firm_name,
                created_at: row.user_firm_created_at,
                updated_at: row.user_firm_updated_at,
            };
            let user = UserResponse {
                id: row.user_id,
                firm: Some(user_firm),
                email: row.user_email,
                first_name: row.first_name,
                last_name: row.last_name,
                created_at: row.user_created_at,
                updated_at: row.user_updated_at,
            };

            CollectionResponse {
                id: row.collection_id,
                client,
                user,
                title: row.title,
                status: row.status,
                access_token: row.access_token,
                expires_at: row.expires_at,
                created_at: row.collection_created_at,
                updated_at: row.collection_updated_at,
            }
        })
        .collect();

    Ok(Json(responses))
}

// GET /collections/:id
pub async fn get_one(
    State(app_state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<CollectionResponse>, StatusCode> {
    let row = sqlx::query!(
        r#"
        SELECT
            c.id as collection_id, c.title, c.status, c.access_token, c.expires_at, c.created_at as collection_created_at, c.updated_at as collection_updated_at,
            cl.id as client_id, cl.company_name, cl.email as client_email, cl.created_at as client_created_at, cl.updated_at as client_updated_at,
            u.id as user_id, u.email as user_email, u.first_name, u.last_name, u.created_at as user_created_at, u.updated_at as user_updated_at,
            f_cl.id as "client_firm_id", f_cl.name as "client_firm_name", f_cl.created_at as "client_firm_created_at", f_cl.updated_at as "client_firm_updated_at",
            f_u.id as "user_firm_id", f_u.name as "user_firm_name", f_u.created_at as "user_firm_created_at", f_u.updated_at as "user_firm_updated_at"
        FROM collections c
        JOIN clients cl ON c.client_id = cl.id
        JOIN users u ON c.user_id = u.id
        JOIN firms f_cl ON cl.firm_id = f_cl.id
        JOIN firms f_u ON u.firm_id = f_u.id
        WHERE c.id = $1
        "#,
        id
    )
    .fetch_one(&app_state.db_pool)
    .await
    .map_err(|_| StatusCode::NOT_FOUND)?;

    let client_firm = Firm {
        id: row.client_firm_id,
        name: row.client_firm_name,
        created_at: row.client_firm_created_at,
        updated_at: row.client_firm_updated_at,
    };
    let client = ClientResponse {
        id: row.client_id,
        firm: client_firm,
        company_name: row.company_name,
        email: row.client_email,
        created_at: row.client_created_at,
        updated_at: row.client_updated_at,
    };

    let user_firm = Firm {
        id: row.user_firm_id,
        name: row.user_firm_name,
        created_at: row.user_firm_created_at,
        updated_at: row.user_firm_updated_at,
    };
    let user = UserResponse {
        id: row.user_id,
        firm: Some(user_firm),
        email: row.user_email,
        first_name: row.first_name,
        last_name: row.last_name,
        created_at: row.user_created_at,
        updated_at: row.user_updated_at,
    };

    let response = CollectionResponse {
        id: row.collection_id,
        client,
        user,
        title: row.title,
        status: row.status,
        access_token: row.access_token,
        expires_at: row.expires_at,
        created_at: row.collection_created_at,
        updated_at: row.collection_updated_at,
    };

    Ok(Json(response))
}

// POST /collections
pub async fn create(
    State(app_state): State<AppState>,
    Json(payload): Json<CreateCollectionPayload>,
) -> Result<Json<CollectionResponse>, StatusCode> {
    let collection = sqlx::query!(
        r#"
        INSERT INTO collections (client_id, user_id, title, status, access_token, expires_at)
        VALUES ($1, $2, $3, 'pending', 'token', now() + interval '1 day')
        RETURNING id
        "#,
        payload.client_id,
        payload.user_id,
        payload.title,
    )
    .fetch_one(&app_state.db_pool)
    .await
    .map_err(|e| {
        eprintln!("Failed to create collection: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    get_one(State(app_state), Path(collection.id)).await
}

// PATCH /collections/:id
pub async fn update(
    State(app_state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateCollectionPayload>,
) -> Result<Json<CollectionResponse>, StatusCode> {
    let mut collection = sqlx::query_as!(Collection, "SELECT * FROM collections WHERE id = $1", id)
        .fetch_one(&app_state.db_pool)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    if let Some(title) = payload.title {
        collection.title = title;
    }

    if let Some(status) = payload.status {
        collection.status = status;
    }

    if let Some(access_token) = payload.access_token {
        collection.access_token = access_token;
    }

    if let Some(expires_at) = payload.expires_at {
        collection.expires_at = expires_at;
    }

    sqlx::query!(
        r#"
        UPDATE collections
        SET title = $1, status = $2, access_token = $3, expires_at = $4, updated_at = now()
        WHERE id = $5
        "#,
        collection.title,
        collection.status,
        collection.access_token,
        collection.expires_at,
        id
    )
    .execute(&app_state.db_pool)
    .await
    .map_err(|_| StatusCode::NOT_FOUND)?;

    get_one(State(app_state), Path(id)).await
}

// DELETE /collections/:id
pub async fn delete(
    State(app_state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, StatusCode> {
    let rows_affected = sqlx::query!("DELETE FROM collections WHERE id = $1", id)
        .execute(&app_state.db_pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .rows_affected();

    if rows_affected == 0 {
        return Err(StatusCode::NOT_FOUND);
    }

    Ok(StatusCode::NO_CONTENT)
}
