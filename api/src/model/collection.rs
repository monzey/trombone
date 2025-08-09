use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

// Represents a specific request for a set of documents (e.g., "Q3 2025 VAT")

#[derive(Debug, Serialize, FromRow)]
pub struct Collection {
    pub id: Uuid,
    pub client_id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub status: String,
    pub access_token: String,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateCollectionPayload {
    pub client_id: Uuid,
    pub user_id: Uuid,
    pub title: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateCollectionPayload {
    pub title: Option<String>,
    pub status: Option<String>,
    pub access_token: Option<String>,
    pub expires_at: Option<DateTime<Utc>>,
}
