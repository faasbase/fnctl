use axum::{http::StatusCode, response::IntoResponse, Json};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

// TODO: replace the fields in this struct with your own input data structure
#[derive(Serialize, Deserialize, JsonSchema, Debug)]
pub struct Input {
    pub firstname: String,
    pub lastname: String,
}

// TODO: replace the fields in this struct with your own output data structure
#[derive(Serialize, Deserialize, JsonSchema, Debug)]
pub struct Output {
    pub name: String,
    pub data: Value,
}

pub async fn handler(Json(data): Json<Input>) -> impl IntoResponse {
    // TODO: implement your function here

    let output = Output {
        name: "hello".to_string(),
        data: serde_json::to_value(data).unwrap(),
    };
    (
        StatusCode::OK,
        Json(json!({
            "status": "ok",
            "data": output,
        })),
    )
}
