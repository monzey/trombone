use crate::app_error::AppError;
use crate::model::firm::Firm;
use crate::model::user::{CreateUserPayload, UpdateUserPayload, User, UserResponse};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use bcrypt::{hash, DEFAULT_COST};
use sqlx::PgPool;
use uuid::Uuid;

// GET /users
pub async fn get_all(State(pool): State<PgPool>) -> Result<Json<Vec<UserResponse>>, AppError> {
    let users = sqlx::query!(
        r#"
        SELECT
            u.id, u.email, u.first_name, u.last_name, u.created_at, u.updated_at,
            f.id as "firm_id", f.name as "firm_name", f.created_at as "firm_created_at", f.updated_at as "firm_updated_at"
        FROM users u
        JOIN firms f ON u.firm_id = f.id
        "#
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| {
        eprintln!("Failed to fetch users: {}", e);
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Failed to fetch users")
    })?;

    let user_responses = users
        .into_iter()
        .map(|row| UserResponse {
            id: row.id,
            email: row.email,
            first_name: row.first_name,
            last_name: row.last_name,
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

    Ok(Json(user_responses))
}

// GET /users/:id
pub async fn get_one(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<UserResponse>, AppError> {
    let user = sqlx::query!(
        r#"
        SELECT
            u.id, u.email, u.first_name, u.last_name, u.created_at, u.updated_at,
            f.id as "firm_id", f.name as "firm_name", f.created_at as "firm_created_at", f.updated_at as "firm_updated_at"
        FROM users u
        JOIN firms f ON u.firm_id = f.id
        WHERE u.id = $1
        "#,
        id
    )
    .fetch_one(&pool)
    .await
    .map_err(|_| AppError::new(StatusCode::NOT_FOUND, "User not found"))?;

    let user_response = UserResponse {
        id: user.id,
        email: user.email,
        first_name: user.first_name,
        last_name: user.last_name,
        created_at: user.created_at,
        updated_at: user.updated_at,
        firm: Firm {
            id: user.firm_id,
            name: user.firm_name,
            created_at: user.firm_created_at,
            updated_at: user.firm_updated_at,
        },
    };

    Ok(Json(user_response))
}

// POST /users
pub async fn create(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateUserPayload>,
) -> Result<Json<UserResponse>, AppError> {
    if payload.password.len() < 8 {
        return Err(AppError::new(
            StatusCode::BAD_REQUEST,
            "Password must be at least 8 characters long.",
        ));
    }

    let hashed_password = hash(&payload.password, DEFAULT_COST).map_err(|e| {
        eprintln!("Error while hashing password: {}", e);
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Error hashing password.")
    })?;

    let user = sqlx::query!(
        r#"
        INSERT INTO users (first_name, last_name, email, password_hash, firm_id)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING id
        "#,
        payload.first_name,
        payload.last_name,
        payload.email.to_lowercase(),
        hashed_password,
        payload.firm_id
    )
    .fetch_one(&pool)
    .await
    .map_err(|e| {
        eprintln!("Error inserting user : {}", e);
        if let Some(db_err) = e.as_database_error() {
            if db_err.is_unique_violation() {
                return AppError::new(StatusCode::CONFLICT, "This email is already in use.");
            }
        }
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Error creating user.")
    })?;

    get_one(State(pool), Path(user.id)).await
}

// PATCH /users/:id
pub async fn update(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateUserPayload>,
) -> Result<Json<UserResponse>, AppError> {
    let mut user = sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", id)
        .fetch_one(&pool)
        .await
        .map_err(|_| AppError::new(StatusCode::NOT_FOUND, "User not found"))?;

    if let Some(first_name) = payload.first_name {
        user.first_name = first_name;
    }

    if let Some(last_name) = payload.last_name {
        user.last_name = last_name;
    }

    if let Some(email) = payload.email {
        user.email = email;
    }

    sqlx::query!(
        "UPDATE users SET first_name = $1, last_name = $2, email = $3, updated_at = now() WHERE id = $4",
        user.first_name,
        user.last_name,
        user.email,
        id
    )
    .execute(&pool)
    .await
    .map_err(|e| {
        eprintln!("Failed to update user: {}", e);
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Error updating user")
    })?;

    get_one(State(pool), Path(id)).await
}

// DELETE /users/:id
pub async fn delete(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    let rows_affected = sqlx::query!("DELETE FROM users WHERE id = $1", id)
        .execute(&pool)
        .await
        .map_err(|_| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Error deleting user"))?
        .rows_affected();

    if rows_affected == 0 {
        return Err(AppError::new(StatusCode::NOT_FOUND, "User not found"));
    }

    Ok(StatusCode::NO_CONTENT)
}