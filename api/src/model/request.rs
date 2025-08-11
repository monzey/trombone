use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use crate::model::collection::CollectionResponse;

// Represents a single line item within a Collection (e.g., "Sales Invoices for July")

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Request {
    pub id: Uuid,
    pub collection_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RequestResponse {
    pub id: Uuid,
    pub collection: CollectionResponse,
    pub title: String,
    pub description: Option<String>,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateRequestPayload {
    pub collection_id: Uuid,
    pub title: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateRequestPayload {
    pub title: Option<String>,
    pub description: Option<String>,
    pub status: Option<String>,
}
