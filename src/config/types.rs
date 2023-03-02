use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;


#[derive(Debug, Deserialize, Serialize)]
pub struct ServerErrorResponse {
    pub error: String,
    pub status: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FaasbaseResponse<T> {
    pub data: T,
    pub status: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct WorkspaceResponse {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
}