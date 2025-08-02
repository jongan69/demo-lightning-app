use crate::error::AppError;
use crate::types::AppState;
use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Json},
};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tracing::{info, instrument};

#[derive(Debug, Serialize, Deserialize)]
pub struct BurnRequest {
    pub asset_id: String,
    pub asset_id_str: Option<String>,
    pub amount_to_burn: String,
    pub confirmation_text: String,
    pub note: Option<String>,
}

#[instrument(skip(client, macaroon_hex, request))]
pub async fn burn_assets(
    client: &Client,
    base_url: &str,
    macaroon_hex: &str,
    request: BurnRequest,
) -> Result<serde_json::Value, AppError> {
    info!("Burning assets for asset ID: {}", request.asset_id);
    let url = format!("{base_url}/v1/taproot-assets/burn");
    let response = client
        .post(&url)
        .header("Grpc-Metadata-macaroon", macaroon_hex)
        .json(&request)
        .send()
        .await?;
    Ok(response
        .json::<serde_json::Value>()
        .await?)
}

#[instrument(skip(client, macaroon_hex))]
pub async fn list_burns(
    client: &Client,
    base_url: &str,
    macaroon_hex: &str,
) -> Result<serde_json::Value, AppError> {
    info!("Listing burns");
    let url = format!("{base_url}/v1/taproot-assets/burns");
    let response = client
        .get(&url)
        .header("Grpc-Metadata-macaroon", macaroon_hex)
        .send()
        .await?;
    Ok(response
        .json::<serde_json::Value>()
        .await?)
}

pub async fn burn(
    State(state): State<AppState>,
    Json(req): Json<BurnRequest>,
) -> impl IntoResponse {
    match burn_assets(
        &state.http_client,
        &state.base_url.0,
        &state.macaroon_hex.0,
        req,
    )
    .await
    {
        Ok(value) => (StatusCode::OK, Json(value)).into_response(),
        Err(e) => {
            let status = e.status_code();
            (
                status,
                Json(serde_json::json!({
                    "error": e.to_string(),
                    "type": format!("{:?}", e)
                })),
            )
                .into_response()
        }
    }
}

pub async fn list(
    State(state): State<AppState>,
) -> impl IntoResponse {
    match list_burns(&state.http_client, &state.base_url.0, &state.macaroon_hex.0).await {
        Ok(value) => (StatusCode::OK, Json(value)).into_response(),
        Err(e) => {
            let status = e.status_code();
            (
                status,
                Json(serde_json::json!({
                    "error": e.to_string(),
                    "type": format!("{:?}", e)
                })),
            )
                .into_response()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_burn_request_serialization() {
        let request = BurnRequest {
            asset_id: "test_asset_id".to_string(),
            asset_id_str: Some("test_asset_id_str".to_string()),
            amount_to_burn: "100".to_string(),
            confirmation_text: "I understand this action cannot be undone".to_string(),
            note: Some("Test burn".to_string()),
        };

        let json = serde_json::to_string(&request).unwrap();
        let deserialized: BurnRequest = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.asset_id, "test_asset_id");
        assert_eq!(deserialized.asset_id_str, Some("test_asset_id_str".to_string()));
        assert_eq!(deserialized.amount_to_burn, "100");
        assert_eq!(deserialized.confirmation_text, "I understand this action cannot be undone");
        assert_eq!(deserialized.note, Some("Test burn".to_string()));
    }

    #[test]
    fn test_burn_request_without_optional_fields() {
        let request = BurnRequest {
            asset_id: "test_asset_id".to_string(),
            asset_id_str: None,
            amount_to_burn: "50".to_string(),
            confirmation_text: "I understand this action cannot be undone".to_string(),
            note: None,
        };

        let json = serde_json::to_string(&request).unwrap();
        let deserialized: BurnRequest = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.asset_id, "test_asset_id");
        assert_eq!(deserialized.asset_id_str, None);
        assert_eq!(deserialized.amount_to_burn, "50");
        assert_eq!(deserialized.confirmation_text, "I understand this action cannot be undone");
        assert_eq!(deserialized.note, None);
    }
}
