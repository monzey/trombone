use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use crate::model::client::ClientResponse;
use crate::model::user::UserResponse;

// Represents a specific request for a set of documents (e.g., "Q3 2025 VAT")

#[derive(Debug, FromRow, Clone)]
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CollectionResponse {
    pub id: Uuid,
    pub client: ClientResponse,
    pub user: UserResponse,
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
