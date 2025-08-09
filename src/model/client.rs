use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

// Represents the firm's client (e.g., a bakery, a freelance consultant)

#[derive(Debug, Serialize, FromRow)]
pub struct Client {
    pub id: Uuid,
    pub firm_id: Uuid,
    pub company_name: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateClientPayload {
    pub firm_id: Uuid,
    pub company_name: String,
    pub email: String,
}
