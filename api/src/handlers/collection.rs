use crate::model::collection::{Collection, CollectionResponse, CreateCollectionPayload, UpdateCollectionPayload};
use crate::model::client::Client;
use crate::model::user::User;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use sqlx::PgPool;
use uuid::Uuid;

// GET /collections
pub async fn get_all(State(pool): State<PgPool>) -> Result<Json<Vec<CollectionResponse>>, StatusCode> {
    let collections = sqlx::query!(
        r#"
        SELECT
            c.id, c.title, c.status, c.access_token, c.expires_at, c.created_at, c.updated_at,
            cl.id as "client_id", cl.firm_id as "client_firm_id", cl.company_name as "client_company_name", cl.email as "client_email", cl.created_at as "client_created_at", cl.updated_at as "client_updated_at",
            u.id as "user_id", u.firm_id as "user_firm_id", u.email as "user_email", u.password_hash as "user_password_hash", u.first_name as "user_first_name", u.last_name as "user_last_name", u.created_at as "user_created_at", u.updated_at as "user_updated_at"
        FROM collections c
        JOIN clients cl ON c.client_id = cl.id
        JOIN users u ON c.user_id = u.id
        "#
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| {
        eprintln!("Failed to fetch collections: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let collection_responses = collections.into_iter().map(|row| {
        CollectionResponse {
            id: row.id,
            title: row.title,
            status: row.status,
            access_token: row.access_token,
            expires_at: row.expires_at,
            created_at: row.created_at,
            updated_at: row.updated_at,
            client: Client {
                id: row.client_id,
                firm_id: row.client_firm_id,
                company_name: row.client_company_name,
                email: row.client_email,
                created_at: row.client_created_at,
                updated_at: row.client_updated_at,
            },
            user: User {
                id: row.user_id,
                firm_id: row.user_firm_id,
                email: row.user_email,
                password_hash: row.user_password_hash,
                first_name: row.user_first_name,
                last_name: row.user_last_name,
                created_at: row.user_created_at,
                updated_at: row.user_updated_at,
            },
        }
    }).collect();

    Ok(Json(collection_responses))
}

// GET /collections/:id
pub async fn get_one(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<CollectionResponse>, StatusCode> {
    let collection = sqlx::query!(
        r#"
        SELECT
            c.id, c.title, c.status, c.access_token, c.expires_at, c.created_at, c.updated_at,
            cl.id as "client_id", cl.firm_id as "client_firm_id", cl.company_name as "client_company_name", cl.email as "client_email", cl.created_at as "client_created_at", cl.updated_at as "client_updated_at",
            u.id as "user_id", u.firm_id as "user_firm_id", u.email as "user_email", u.password_hash as "user_password_hash", u.first_name as "user_first_name", u.last_name as "user_last_name", u.created_at as "user_created_at", u.updated_at as "user_updated_at"
        FROM collections c
        JOIN clients cl ON c.client_id = cl.id
        JOIN users u ON c.user_id = u.id
        WHERE c.id = $1
        "#,
        id
    )
    .fetch_one(&pool)
    .await
    .map_err(|_| StatusCode::NOT_FOUND)?;

    let collection_response = CollectionResponse {
        id: collection.id,
        title: collection.title,
        status: collection.status,
        access_token: collection.access_token,
        expires_at: collection.expires_at,
        created_at: collection.created_at,
        updated_at: collection.updated_at,
        client: Client {
            id: collection.client_id,
            firm_id: collection.client_firm_id,
            company_name: collection.client_company_name,
            email: collection.client_email,
            created_at: collection.client_created_at,
            updated_at: collection.client_updated_at,
        },
        user: User {
            id: collection.user_id,
            firm_id: collection.user_firm_id,
            email: collection.user_email,
            password_hash: collection.user_password_hash,
            first_name: collection.user_first_name,
            last_name: collection.user_last_name,
            created_at: collection.user_created_at,
            updated_at: collection.user_updated_at,
        },
    };

    Ok(Json(collection_response))
}

// POST /collections
pub async fn create(
    State(pool): State<PgPool>,
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
    .fetch_one(&pool)
    .await
    .map_err(|e| {
        eprintln!("Failed to create collection: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    get_one(State(pool), Path(collection.id)).await
}

// PATCH /collections/:id
pub async fn update(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateCollectionPayload>,
) -> Result<Json<CollectionResponse>, StatusCode> {
    let mut collection = sqlx::query_as!(Collection, "SELECT * FROM collections WHERE id = $1", id)
        .fetch_one(&pool)
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
    .execute(&pool)
    .await
    .map_err(|_| StatusCode::NOT_FOUND)?;

    get_one(State(pool), Path(id)).await
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
