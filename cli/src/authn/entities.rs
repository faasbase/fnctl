use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DeviceCodeResponse {
    pub user_code: String,
    pub device_code: String,
    pub expires_in: u64,
    pub interval: u64,
    pub verification_uri: String,
    pub verification_uri_complete: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenErrorResponse {
    pub error: String,
    pub error_description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenSuccessResponse {
    pub access_token: String,
    pub id_token: String,
    pub refresh_token: String,
    pub scope: String,
    pub token_type: String,
    pub expires_in: u64,
}