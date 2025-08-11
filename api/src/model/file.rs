use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use crate::model::request::RequestResponse;

// Represents a file uploaded by an end-client for a specific Request

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct File {
    pub id: Uuid,
    pub request_id: Uuid,
    pub file_name: String,
    pub storage_key: String,
    pub file_size: i64,
    pub mime_type: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FileResponse {
    pub id: Uuid,
    pub request: RequestResponse,
    pub file_name: String,
    pub storage_key: String,
    pub file_size: i64,
    pub mime_type: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// Payloads for file creation would typically be handled via multipart forms,
// not direct JSON, so we don't define Create/Update payloads here.