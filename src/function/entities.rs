use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;


#[derive(Debug, Deserialize, Serialize)]
pub struct FunctionResponse {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub language: Option<String>,
    pub latest_version: Option<String>,
    pub workspace_id: Uuid,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct WorkspaceResponse {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
}

#[derive(Deserialize, Clone)]
pub struct CargoTomlSchema {
    pub package: Package,
}

#[derive(Deserialize, Clone)]
pub struct Package {
    pub name: String,
    pub version: String,
    pub visibility: Option<String>,
    pub description: Option<String>,
}