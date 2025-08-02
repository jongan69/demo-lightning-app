use axum::{response::Json, http::StatusCode, extract::State};
use serde_json::Value;
use crate::types::AppState;

pub async fn new_address(
    State(state): State<AppState>,
    Json(payload): Json<Value>
) -> Result<Json<Value>, StatusCode> {
    match state.tapd_client.new_address(payload).await {
        Ok(address) => Ok(Json(address)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn list_addresses(
    State(state): State<AppState>
) -> Result<Json<Value>, StatusCode> {
    match state.tapd_client.list_addresses().await {
        Ok(addresses) => Ok(Json(addresses)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}