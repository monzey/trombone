use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

// Represents a single line item within a Collection (e.g., "Sales Invoices for July")

#[derive(Debug, Serialize, FromRow)]
pub struct Request {
    pub id: Uuid,
    pub collection_id: Uuid,
    pub title: String,
    pub description: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateRequestPayload {
    pub collection_id: Uuid,
    pub title: String,
    pub description: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateRequestPayload {
    pub title: Option<String>,
    pub description: Option<String>,
    pub status: Option<String>, // e.g., "open", "in_progress", "completed"
}
