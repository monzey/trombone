use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

// Represents a file uploaded by an end-client for a specific Request

#[derive(Debug, Serialize, FromRow)]
pub struct File {
    pub id: Uuid,
    pub request_id: Uuid,
    pub storage_key: String,
    pub file_size: i64,
    pub mime_type: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateFilePayload {
    pub request_id: Uuid,
    pub storage_key: String,
    pub file_size: i64,
    pub mime_type: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateFilePayload {
    pub request_id: Option<Uuid>,
    pub storage_key: Option<String>,
    pub file_size: Option<i64>,
    pub mime_type: Option<String>,
}