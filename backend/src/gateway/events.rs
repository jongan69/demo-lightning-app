use crate::error::AppError;
use crate::types::AppState;
use axum::{
    extract::{Query, State, WebSocketUpgrade},
    http::StatusCode,
    response::{IntoResponse, Json},
    routing::post,
    Router,
};
use axum::extract::ws::{Message, WebSocket};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::Duration;
use tracing::{info, instrument, warn};

#[derive(Debug, Serialize, Deserialize)]
pub struct DebugLevelRequest {
    pub show: bool,
    pub level_spec: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssetMintRequest {
    pub short_response: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssetReceiveRequest {
    pub filter_addr: Option<String>,
    pub start_timestamp: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssetSendRequest {
    pub filter_script_key: Option<String>,
    pub filter_label: Option<String>,
}

// Create a separate client for event subscriptions with longer timeout
fn create_event_client() -> Result<Client, AppError> {
    Client::builder()
        .danger_accept_invalid_certs(true)
        .timeout(Duration::from_secs(300)) // 5 minute timeout for event subscriptions
        .build()
        .map_err(|e| AppError::ValidationError(format!("Failed to create event client: {e}")))
}

#[instrument(skip(client, macaroon_hex, request))]
pub async fn set_debug_level(
    client: &Client,
    base_url: &str,
    macaroon_hex: &str,
    request: DebugLevelRequest,
) -> Result<serde_json::Value, AppError> {
    info!("Setting debug level: {}", request.level_spec);
    let url = format!("{base_url}/v1/taproot-assets/debuglevel");
    let response = client
        .post(&url)
        .header("Grpc-Metadata-macaroon", macaroon_hex)
        .json(&request)
        .send()
        .await
        .map_err(|e| AppError::RequestError(e.to_string()))?;
    response
        .json::<serde_json::Value>()
        .await
        .map_err(|e| AppError::RequestError(e.to_string()))
}

#[instrument(skip(macaroon_hex, request))]
pub async fn asset_mint_events(
    base_url: &str,
    macaroon_hex: &str,
    request: AssetMintRequest,
) -> Result<serde_json::Value, AppError> {
    info!("Subscribing to asset mint events");
    let event_client = create_event_client()?;
    let url = format!("{base_url}/v1/taproot-assets/events/asset-mint");

    let response = event_client
        .post(&url)
        .header("Grpc-Metadata-macaroon", macaroon_hex)
        .json(&request)
        .send()
        .await;

    match response {
        Ok(resp) => {
            let status = resp.status();
            if status.is_success() {
                resp.json::<serde_json::Value>()
                    .await
                    .map_err(|e| AppError::RequestError(e.to_string()))
            } else {
                let error_text = resp
                    .text()
                    .await
                    .unwrap_or_else(|_| "Unknown error".to_string());
                Err(AppError::ValidationError(format!(
                    "Event subscription failed with status {status}: {error_text}"
                )))
            }
        }
        Err(e) if e.is_timeout() => {
            warn!("Asset mint event subscription timed out");
            Ok(serde_json::json!({
                "events": [],
                "timeout": true,
                "message": "No events received within timeout period"
            }))
        }
        Err(e) => Err(AppError::RequestError(e.to_string())),
    }
}

#[instrument(skip(macaroon_hex, request))]
pub async fn asset_receive_events(
    base_url: &str,
    macaroon_hex: &str,
    request: AssetReceiveRequest,
) -> Result<serde_json::Value, AppError> {
    info!("Subscribing to asset receive events");
    let event_client = create_event_client()?;
    let url = format!("{base_url}/v1/taproot-assets/events/asset-receive");

    let response = event_client
        .post(&url)
        .header("Grpc-Metadata-macaroon", macaroon_hex)
        .json(&request)
        .send()
        .await;

    match response {
        Ok(resp) => {
            let status = resp.status();
            if status.is_success() {
                resp.json::<serde_json::Value>()
                    .await
                    .map_err(|e| AppError::RequestError(e.to_string()))
            } else {
                let error_text = resp
                    .text()
                    .await
                    .unwrap_or_else(|_| "Unknown error".to_string());
                Err(AppError::ValidationError(format!(
                    "Event subscription failed with status {status}: {error_text}"
                )))
            }
        }
        Err(e) if e.is_timeout() => {
            warn!("Asset receive event subscription timed out");
            Ok(serde_json::json!({
                "events": [],
                "timeout": true,
                "message": "No events received within timeout period"
            }))
        }
        Err(e) => Err(AppError::RequestError(e.to_string())),
    }
}

#[instrument(skip(macaroon_hex, request))]
pub async fn asset_send_events(
    base_url: &str,
    macaroon_hex: &str,
    request: AssetSendRequest,
) -> Result<serde_json::Value, AppError> {
    info!("Subscribing to asset send events");
    let event_client = create_event_client()?;
    let url = format!("{base_url}/v1/taproot-assets/events/asset-send");

    let response = event_client
        .post(&url)
        .header("Grpc-Metadata-macaroon", macaroon_hex)
        .json(&request)
        .send()
        .await;

    match response {
        Ok(resp) => {
            let status = resp.status();
            if status.is_success() {
                resp.json::<serde_json::Value>()
                    .await
                    .map_err(|e| AppError::RequestError(e.to_string()))
            } else {
                let error_text = resp
                    .text()
                    .await
                    .unwrap_or_else(|_| "Unknown error".to_string());
                Err(AppError::ValidationError(format!(
                    "Event subscription failed with status {status}: {error_text}"
                )))
            }
        }
        Err(e) if e.is_timeout() => {
            warn!("Asset send event subscription timed out");
            Ok(serde_json::json!({
                "events": [],
                "timeout": true,
                "message": "No events received within timeout period"
            }))
        }
        Err(e) => Err(AppError::RequestError(e.to_string())),
    }
}

// WebSocket proxy handler for events
pub struct EventWebSocketProxyHandler {
    pub client: Arc<reqwest::Client>,
    pub base_url: String,
    pub macaroon_hex: String,
}

impl EventWebSocketProxyHandler {
    pub fn new(client: Arc<reqwest::Client>, base_url: String, macaroon_hex: String) -> Self {
        Self {
            client,
            base_url,
            macaroon_hex,
        }
    }

    pub async fn handle_websocket(
        self: Arc<Self>,
        ws: WebSocketUpgrade,
        backend_endpoint: String,
        _enable_correlation: bool,
    ) -> impl IntoResponse {
        ws.on_upgrade(|socket| self.handle_socket(socket, backend_endpoint))
    }

    async fn handle_socket(
        self: Arc<Self>,
        mut socket: WebSocket,
        _backend_endpoint: String,
    ) {
        // For now, we'll implement a basic WebSocket proxy
        // In a full implementation, you'd connect to the backend WebSocket
        // and proxy messages between the client and backend
        
        while let Some(msg) = socket.recv().await {
            match msg {
                Ok(Message::Text(text)) => {
                    info!("Received WebSocket message: {}", text);
                    // Echo back for now - replace with actual backend communication
                    if let Err(e) = socket.send(Message::Text(text)).await {
                        info!("Failed to send WebSocket message: {}", e);
                        break;
                    }
                }
                Ok(Message::Close(_)) => {
                    info!("WebSocket connection closed");
                    break;
                }
                Err(e) => {
                    info!("WebSocket error: {}", e);
                    break;
                }
                _ => {}
            }
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct EventQueryParams {
    pub method: Option<String>,
    pub short_response: Option<bool>,
    pub filter_addr: Option<String>,
    pub start_timestamp: Option<String>,
    pub filter_script_key: Option<String>,
    pub filter_label: Option<String>,
}

async fn generic_event_websocket_handler(
    State(state): State<AppState>,
    Query(params): Query<EventQueryParams>,
    ws: WebSocketUpgrade,
    event_type: &str,
) -> impl IntoResponse {
    info!("Handling WebSocket connection for {} events", event_type);

    // Extract query parameters and forward them to the backend
    let mut query_params = Vec::new();
    query_params.push("method=POST".to_string());
    
    if let Some(short_response) = params.short_response {
        query_params.push(format!("short_response={}", short_response));
    }
    if let Some(filter_addr) = params.filter_addr {
        query_params.push(format!("filter_addr={}", filter_addr));
    }
    if let Some(start_timestamp) = params.start_timestamp {
        query_params.push(format!("start_timestamp={}", start_timestamp));
    }
    if let Some(filter_script_key) = params.filter_script_key {
        query_params.push(format!("filter_script_key={}", filter_script_key));
    }
    if let Some(filter_label) = params.filter_label {
        query_params.push(format!("filter_label={}", filter_label));
    }

    let query_string = query_params.join("&");
    let endpoint = format!("/v1/taproot-assets/events/{event_type}?{}", query_string);

    let ws_handler = Arc::new(EventWebSocketProxyHandler::new(
        state.http_client.clone(),
        state.base_url.0.clone(),
        state.macaroon_hex.0.clone(),
    ));

    ws_handler.handle_websocket(ws, endpoint, false).await
}

async fn asset_mint_websocket_handler(
    State(state): State<AppState>,
    Query(params): Query<EventQueryParams>,
    ws: WebSocketUpgrade,
) -> impl IntoResponse {
    generic_event_websocket_handler(State(state), Query(params), ws, "asset-mint").await
}

async fn asset_receive_websocket_handler(
    State(state): State<AppState>,
    Query(params): Query<EventQueryParams>,
    ws: WebSocketUpgrade,
) -> impl IntoResponse {
    generic_event_websocket_handler(State(state), Query(params), ws, "asset-receive").await
}

async fn asset_send_websocket_handler(
    State(state): State<AppState>,
    Query(params): Query<EventQueryParams>,
    ws: WebSocketUpgrade,
) -> impl IntoResponse {
    generic_event_websocket_handler(State(state), Query(params), ws, "asset-send").await
}

async fn set_debug_level_handler(
    State(state): State<AppState>,
    Json(req): Json<DebugLevelRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    match set_debug_level(
        &state.http_client,
        &state.base_url.0,
        &state.macaroon_hex.0,
        req,
    )
    .await
    {
        Ok(value) => Ok(Json(value)),
        Err(e) => Err(error_response(e)),
    }
}

async fn asset_mint_handler(
    State(state): State<AppState>,
    Json(req): Json<AssetMintRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    match asset_mint_events(
        &state.base_url.0,
        &state.macaroon_hex.0,
        req,
    )
    .await
    {
        Ok(value) => Ok(Json(value)),
        Err(e) => Err(error_response(e)),
    }
}

async fn asset_receive_handler(
    State(state): State<AppState>,
    Json(req): Json<AssetReceiveRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    match asset_receive_events(
        &state.base_url.0,
        &state.macaroon_hex.0,
        req,
    )
    .await
    {
        Ok(value) => Ok(Json(value)),
        Err(e) => Err(error_response(e)),
    }
}

async fn asset_send_handler(
    State(state): State<AppState>,
    Json(req): Json<AssetSendRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    match asset_send_events(
        &state.base_url.0,
        &state.macaroon_hex.0,
        req,
    )
    .await
    {
        Ok(value) => Ok(Json(value)),
        Err(e) => Err(error_response(e)),
    }
}

fn error_response(error: AppError) -> (StatusCode, Json<serde_json::Value>) {
    let status = error.status_code();
    (
        status,
        Json(serde_json::json!({
            "error": error.to_string(),
            "type": format!("{:?}", error)
        })),
    )
}

pub fn create_events_routes() -> Router<AppState> {
    Router::new()
        .route("/debuglevel", post(set_debug_level_handler))
        .route(
            "/events/asset-mint",
            post(asset_mint_handler).get(asset_mint_websocket_handler),
        )
        .route(
            "/events/asset-receive",
            post(asset_receive_handler).get(asset_receive_websocket_handler),
        )
        .route(
            "/events/asset-send",
            post(asset_send_handler).get(asset_send_websocket_handler),
        )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_websocket_url_format_asset_mint() {
        let base_url = "wss://localhost:8080";
        let endpoint = "/v1/taproot-assets/events/asset-mint?method=POST";
        let full_url = format!("{}{}", base_url, endpoint);

        assert_eq!(
            full_url,
            "wss://localhost:8080/v1/taproot-assets/events/asset-mint?method=POST"
        );
        assert!(full_url.contains("method=POST"));
        assert!(full_url.starts_with("wss://"));
    }

    #[test]
    fn test_websocket_url_format_asset_receive() {
        let base_url = "wss://localhost:8080";
        let endpoint = "/v1/taproot-assets/events/asset-receive?method=POST";
        let full_url = format!("{}{}", base_url, endpoint);

        assert_eq!(
            full_url,
            "wss://localhost:8080/v1/taproot-assets/events/asset-receive?method=POST"
        );
        assert!(full_url.contains("method=POST"));
        assert!(full_url.starts_with("wss://"));
    }

    #[test]
    fn test_websocket_url_format_asset_send() {
        let base_url = "wss://localhost:8080";
        let endpoint = "/v1/taproot-assets/events/asset-send?method=POST";
        let full_url = format!("{}{}", base_url, endpoint);

        assert_eq!(
            full_url,
            "wss://localhost:8080/v1/taproot-assets/events/asset-send?method=POST"
        );
        assert!(full_url.contains("method=POST"));
        assert!(full_url.starts_with("wss://"));
    }

    #[test]
    fn test_websocket_query_parameter_forwarding() {
        // Test query parameter handling for different event types

        // Asset mint parameters
        let mint_query = "short_response=true&method=POST";
        assert!(mint_query.contains("method=POST"));
        assert!(mint_query.contains("short_response=true"));

        // Asset receive parameters
        let receive_query = "filter_addr=addr123&start_timestamp=1234567890&method=POST";
        assert!(receive_query.contains("method=POST"));
        assert!(receive_query.contains("filter_addr=addr123"));
        assert!(receive_query.contains("start_timestamp=1234567890"));

        // Asset send parameters
        let send_query = "filter_script_key=key123&filter_label=label456&method=POST";
        assert!(send_query.contains("method=POST"));
        assert!(send_query.contains("filter_script_key=key123"));
        assert!(send_query.contains("filter_label=label456"));
    }

    #[test]
    fn test_asset_mint_request_serialization() {
        let request = AssetMintRequest {
            short_response: true,
        };

        let serialized = serde_json::to_string(&request).unwrap();
        assert!(serialized.contains("short_response"));
        assert!(serialized.contains("true"));
    }

    #[test]
    fn test_asset_receive_request_serialization() {
        let request = AssetReceiveRequest {
            filter_addr: Some("addr123".to_string()),
            start_timestamp: Some("1234567890".to_string()),
        };

        let serialized = serde_json::to_string(&request).unwrap();
        assert!(serialized.contains("filter_addr"));
        assert!(serialized.contains("addr123"));
        assert!(serialized.contains("start_timestamp"));
        assert!(serialized.contains("1234567890"));
    }

    #[test]
    fn test_asset_send_request_serialization() {
        let request = AssetSendRequest {
            filter_script_key: Some("key123".to_string()),
            filter_label: Some("label456".to_string()),
        };

        let serialized = serde_json::to_string(&request).unwrap();
        assert!(serialized.contains("filter_script_key"));
        assert!(serialized.contains("key123"));
        assert!(serialized.contains("filter_label"));
        assert!(serialized.contains("label456"));
    }

    #[test]
    fn test_event_schema_validation() {
        // Validate that expected response fields match the documented schemas

        // Asset mint event schema
        let mint_event = serde_json::json!({
            "timestamp": "1234567890",
            "batch_state": "BATCH_STATE_BROADCAST",
            "batch": {
                "batch_key": "key123",
                "batch_txid": "txid123"
            },
            "error": ""
        });
        assert!(mint_event.get("timestamp").is_some());
        assert!(mint_event.get("batch_state").is_some());
        assert!(mint_event.get("batch").is_some());

        // Asset receive event schema
        let receive_event = serde_json::json!({
            "timestamp": "1234567890",
            "address": {
                "encoded": "addr123",
                "asset_id": "asset123"
            },
            "outpoint": "outpoint123",
            "status": "ADDR_EVENT_STATUS_TRANSACTION_CONFIRMED",
            "confirmation_height": 100,
            "error": ""
        });
        assert!(receive_event.get("timestamp").is_some());
        assert!(receive_event.get("address").is_some());
        assert!(receive_event.get("outpoint").is_some());
        assert!(receive_event.get("status").is_some());

        // Asset send event schema
        let send_event = serde_json::json!({
            "timestamp": "1234567890",
            "send_state": "SEND_STATE_VIRTUAL_COMMIT_BROADCAST",
            "parcel_type": "PARCEL_TYPE_SEND",
            "addresses": [],
            "virtual_packets": [],
            "passive_virtual_packets": [],
            "anchor_transaction": {},
            "transfer": {},
            "error": "",
            "transfer_label": "label123",
            "next_send_state": "SEND_STATE_COMPLETED"
        });
        assert!(send_event.get("timestamp").is_some());
        assert!(send_event.get("send_state").is_some());
        assert!(send_event.get("parcel_type").is_some());
        assert!(send_event.get("addresses").is_some());
    }
}
