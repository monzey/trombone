use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

// Represents an accountant or employee belonging to a Firm

#[derive(Debug, Serialize, FromRow)]
pub struct User {
    pub id: Uuid,
    pub firm_id: Uuid,
    pub email: String,
    pub password_hash: String,
    pub first_name: String,
    pub last_name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateUserPayload {
    pub firm_id: Uuid,
    pub email: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUserPayload {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
}
