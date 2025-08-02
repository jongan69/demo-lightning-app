use axum::{response::Json, http::StatusCode, extract::State};
use serde_json::Value;
use crate::types::AppState;

pub async fn list_assets(
    State(state): State<AppState>
) -> Result<Json<Value>, StatusCode> {
    match state.tapd_client.list_assets().await {
        Ok(assets) => Ok(Json(serde_json::to_value(assets).unwrap_or_default())),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn mint_asset(
    State(state): State<AppState>,
    Json(payload): Json<Value>
) -> Result<Json<Value>, StatusCode> {
    match state.tapd_client.mint_asset_raw(payload).await {
        Ok(result) => Ok(Json(result)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}