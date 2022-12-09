use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct ApplicationResponse {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub language: Option<String>,
    pub latest_version: Option<String>,
    pub workspace_id: Uuid,
    pub created_at: NaiveDateTime,
}