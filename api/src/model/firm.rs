use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

// Represents the accounting firm, which is the paying customer.

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Firm {
    pub id: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateFirmPayload {
    pub name: String,
}
