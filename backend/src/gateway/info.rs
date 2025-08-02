use axum::{response::Json, http::StatusCode, extract::State};
use serde_json::Value;
use crate::types::AppState;

pub async fn get_info(
    State(state): State<AppState>
) -> Result<Json<Value>, StatusCode> {
    match state.tapd_client.get_info().await {
        Ok(info) => Ok(Json(info)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
