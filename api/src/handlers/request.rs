use crate::handlers::collection as collection_handler;
use crate::model::request::{CreateRequestPayload, Request, RequestResponse, UpdateRequestPayload};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use uuid::Uuid;

use crate::app_state::AppState;

// GET /requests
pub async fn get_all(
    State(app_state): State<AppState>,
) -> Result<Json<Vec<RequestResponse>>, StatusCode> {
    // This is inefficient due to N+1, but simple. A real implementation would use a more complex query.
    let requests = sqlx::query_as!(Request, "SELECT * FROM requests")
        .fetch_all(&app_state.db_pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mut responses = Vec::new();
    for request in requests {
        let response = get_one(State(app_state.clone()), Path(request.id)).await?.0;
        responses.push(response);
    }

    Ok(Json(responses))
}

// GET /requests/:id
pub async fn get_one(
    State(app_state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<RequestResponse>, StatusCode> {
    let request = sqlx::query_as!(Request, "SELECT * FROM requests WHERE id = $1", id)
        .fetch_one(&app_state.db_pool)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    let collection_response =
        collection_handler::get_one(State(app_state), Path(request.collection_id))
            .await?
            .0;

    let request_response = RequestResponse {
        id: request.id,
        collection: collection_response,
        title: request.title,
        description: request.description,
        status: request.status,
        created_at: request.created_at,
        updated_at: request.updated_at,
    };

    Ok(Json(request_response))
}

// POST /requests
pub async fn create(
    State(app_state): State<AppState>,
    Json(payload): Json<CreateRequestPayload>,
) -> Result<Json<RequestResponse>, StatusCode> {
    let request = sqlx::query!(
        r#"
        INSERT INTO requests (collection_id, title, description, status)
        VALUES ($1, $2, $3, 'pending')
        RETURNING id
        "#,
        payload.collection_id,
        payload.title,
        payload.description
    )
    .fetch_one(&app_state.db_pool)
    .await
    .map_err(|e| {
        eprintln!("Failed to create request: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    get_one(State(app_state), Path(request.id)).await
}

// PATCH /requests/:id
pub async fn update(
    State(app_state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateRequestPayload>,
) -> Result<Json<RequestResponse>, StatusCode> {
    let mut request = sqlx::query_as!(Request, "SELECT * FROM requests WHERE id = $1", id)
        .fetch_one(&app_state.db_pool)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    if let Some(title) = payload.title {
        request.title = title;
    }

    if let Some(description) = payload.description {
        request.description = Some(description);
    }

    if let Some(status) = payload.status {
        request.status = status;
    }

    sqlx::query!(
        r#"
        UPDATE requests
        SET title = $1, description = $2, status = $3, updated_at = now()
        WHERE id = $4
        "#,
        request.title,
        request.description,
        request.status,
        id
    )
    .execute(&app_state.db_pool)
    .await
    .map_err(|_| StatusCode::NOT_FOUND)?;

    get_one(State(app_state), Path(id)).await
}

// DELETE /requests/:id
pub async fn delete(
    State(app_state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, StatusCode> {
    let rows_affected = sqlx::query!("DELETE FROM requests WHERE id = $1", id)
        .execute(&app_state.db_pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .rows_affected();

    if rows_affected == 0 {
        return Err(StatusCode::NOT_FOUND);
    }

    Ok(StatusCode::NO_CONTENT)
}

