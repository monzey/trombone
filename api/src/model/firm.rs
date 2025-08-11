use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use crate::model::client::ClientResponse;
use crate::model::user::UserResponse;

// Represents the accounting firm, which is the paying customer.

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Firm {
    pub id: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FirmResponse {
    pub id: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub users: Vec<UserResponse>,
    pub clients: Vec<ClientResponse>,
}

#[derive(Debug, Deserialize)]
pub struct CreateFirmPayload {
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateFirmPayload {
    pub name: Option<String>,
}
