use axum::{response::Json, http::StatusCode};
use serde_json::Value;
use crate::types::AppState;

pub async fn health() -> Json<Value> {
    Json(serde_json::json!({
        "status": "healthy",
        "timestamp": chrono::Utc::now().to_rfc3339()
    }))
}

pub async fn readiness(
    axum::extract::State(state): axum::extract::State<AppState>
) -> Result<Json<Value>, StatusCode> {
    // Simple readiness check - you can enhance this based on your needs
    match state.tapd_client.get_info().await {
        Ok(_) => Ok(Json(serde_json::json!({
            "status": "ready",
            "services": {"taproot_assets": "up"}
        }))),
        Err(_) => Err(StatusCode::SERVICE_UNAVAILABLE),
    }
}