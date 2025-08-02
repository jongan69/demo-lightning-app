use axum::{response::Json, http::StatusCode, extract::State};
use serde_json::Value;
use crate::types::AppState;

pub async fn get_balance(
    State(state): State<AppState>
) -> Result<Json<Value>, StatusCode> {
    match state.tapd_client.get_balance().await {
        Ok(balance) => Ok(Json(balance)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
