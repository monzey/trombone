use crate::app_error::AppError;
use crate::model::user::{CreateUserPayload, UpdateUserPayload, User};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use bcrypt::{hash, DEFAULT_COST};
use sqlx::PgPool;
use uuid::Uuid;

pub async fn create(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateUserPayload>,
) -> Result<Json<User>, AppError> {
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

    let user = sqlx::query_as!(
        User,
        r#"
        INSERT INTO users (first_name, last_name, email, password_hash, firm_id)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING id, firm_id, email, password_hash, first_name, last_name, created_at, updated_at
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

    Ok(Json(user))
}

pub async fn get_all(State(pool): State<PgPool>) -> Result<Json<Vec<User>>, StatusCode> {
    let users = sqlx::query_as!(User, "SELECT * FROM users")
        .fetch_all(&pool)
        .await
        .map_err(|e| {
            eprintln!("Failed to fetch users: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(users))
}

pub async fn get_one(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<User>, StatusCode> {
    let user = sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", id)
        .fetch_one(&pool)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    Ok(Json(user))
}

pub async fn update(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateUserPayload>,
) -> Result<Json<User>, StatusCode> {
    let mut user = sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", id)
        .fetch_one(&pool)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    if let Some(first_name) = payload.first_name {
        user.first_name = first_name;
    }

    if let Some(last_name) = payload.last_name {
        user.last_name = last_name;
    }

    if let Some(email) = payload.email {
        user.email = email;
    }

    let updated_user = sqlx::query_as!(
        User,
        "UPDATE users SET first_name = $1, last_name = $2, email = $3 WHERE id = $4 RETURNING *",
        user.first_name,
        user.last_name,
        user.email,
        id
    )
    .fetch_one(&pool)
    .await
    .map_err(|e| {
        eprintln!("Failed to update user: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(updated_user))
}

pub async fn delete(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, StatusCode> {
    let rows_affected = sqlx::query!("DELETE FROM users WHERE id = $1", id)
        .execute(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .rows_affected();

    if rows_affected == 0 {
        return Err(StatusCode::NOT_FOUND);
    }

    Ok(StatusCode::NO_CONTENT)
}
