use axum::{
    extract::{Path, State, WebSocketUpgrade, ws::{WebSocket, Message}},
    response::{Response, Json},
    http::StatusCode,
};
use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio::time::{interval, Duration};
use tracing::{info, error, instrument};
use crate::{
    error::AppError,
    types::AppState,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct BuyOfferRequest {
    pub asset_specifier: serde_json::Value,
    pub max_units: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BuyOrderRequest {
    pub asset_specifier: serde_json::Value,
    pub asset_max_amt: String,
    pub expiry: String,
    pub peer_pub_key: String,
    pub timeout_seconds: u32,
    pub skip_asset_channel_check: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SellOfferRequest {
    pub asset_specifier: serde_json::Value,
    pub max_units: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SellOrderRequest {
    pub asset_specifier: serde_json::Value,
    pub payment_max_amt: String,
    pub expiry: String,
    pub peer_pub_key: String,
    pub timeout_seconds: u32,
    pub skip_asset_channel_check: bool,
}

// Core RFQ functions
#[instrument(skip(client, macaroon_hex, request))]
pub async fn buy_offer(
    client: &reqwest::Client,
    base_url: &str,
    macaroon_hex: &str,
    request: BuyOfferRequest,
    asset_id: &str,
) -> Result<Value, AppError> {
    info!("Creating buy offer for asset ID: {}", asset_id);
    let url = format!("{base_url}/v1/taproot-assets/rfq/buyoffer/asset-id/{asset_id}");
    let response = client
        .post(&url)
        .header("Grpc-Metadata-macaroon", macaroon_hex)
        .json(&request)
        .send()
        .await?;
    
    if !response.status().is_success() {
        let error_text = response.text().await?;
        return Err(AppError::RequestError(error_text));
    }
    
    let result = response.json::<Value>().await?;
    Ok(result)
}

#[instrument(skip(client, macaroon_hex, request))]
pub async fn buy_order(
    client: &reqwest::Client,
    base_url: &str,
    macaroon_hex: &str,
    request: BuyOrderRequest,
    asset_id: &str,
) -> Result<Value, AppError> {
    info!("Creating buy order for asset ID: {}", asset_id);
    let url = format!("{base_url}/v1/taproot-assets/rfq/buyorder/asset-id/{asset_id}");
    let response = client
        .post(&url)
        .header("Grpc-Metadata-macaroon", macaroon_hex)
        .json(&request)
        .send()
        .await?;
    
    if !response.status().is_success() {
        let error_text = response.text().await?;
        return Err(AppError::RequestError(error_text));
    }
    
    let result = response.json::<Value>().await?;
    Ok(result)
}

#[instrument(skip(client, macaroon_hex))]
pub async fn get_notifications(
    client: &reqwest::Client,
    base_url: &str,
    macaroon_hex: &str,
) -> Result<Value, AppError> {
    info!("Fetching RFQ notifications");
    let url = format!("{base_url}/v1/taproot-assets/rfq/ntfs");
    let response = client
        .post(&url)
        .header("Grpc-Metadata-macaroon", macaroon_hex)
        .json(&serde_json::json!({}))
        .send()
        .await?;
    
    if !response.status().is_success() {
        let error_text = response.text().await?;
        return Err(AppError::RequestError(error_text));
    }
    
    let result = response.json::<Value>().await?;
    Ok(result)
}

#[instrument(skip(client, macaroon_hex))]
pub async fn get_asset_rates(
    client: &reqwest::Client,
    base_url: &str,
    macaroon_hex: &str,
) -> Result<Value, AppError> {
    info!("Fetching asset rates");
    let url = format!("{base_url}/v1/taproot-assets/rfq/priceoracle/assetrates");
    let response = client
        .get(&url)
        .header("Grpc-Metadata-macaroon", macaroon_hex)
        .send()
        .await?;
    
    if !response.status().is_success() {
        let error_text = response.text().await?;
        return Err(AppError::RequestError(error_text));
    }
    
    let result = response.json::<Value>().await?;
    Ok(result)
}

#[instrument(skip(client, macaroon_hex))]
pub async fn get_peer_quotes(
    client: &reqwest::Client,
    base_url: &str,
    macaroon_hex: &str,
) -> Result<Value, AppError> {
    info!("Fetching peer-accepted quotes");
    let url = format!("{base_url}/v1/taproot-assets/rfq/quotes/peeraccepted");
    let response = client
        .get(&url)
        .header("Grpc-Metadata-macaroon", macaroon_hex)
        .send()
        .await?;
    
    if !response.status().is_success() {
        let error_text = response.text().await?;
        return Err(AppError::RequestError(error_text));
    }
    
    let result = response.json::<Value>().await?;
    Ok(result)
}

#[instrument(skip(client, macaroon_hex, request))]
pub async fn sell_offer(
    client: &reqwest::Client,
    base_url: &str,
    macaroon_hex: &str,
    request: SellOfferRequest,
    asset_id: &str,
) -> Result<Value, AppError> {
    info!("Creating sell offer for asset ID: {}", asset_id);
    let url = format!("{base_url}/v1/taproot-assets/rfq/selloffer/asset-id/{asset_id}");
    let response = client
        .post(&url)
        .header("Grpc-Metadata-macaroon", macaroon_hex)
        .json(&request)
        .send()
        .await?;
    
    if !response.status().is_success() {
        let error_text = response.text().await?;
        return Err(AppError::RequestError(error_text));
    }
    
    let result = response.json::<Value>().await?;
    Ok(result)
}

#[instrument(skip(client, macaroon_hex, request))]
pub async fn sell_order(
    client: &reqwest::Client,
    base_url: &str,
    macaroon_hex: &str,
    request: SellOrderRequest,
    asset_id: &str,
) -> Result<Value, AppError> {
    info!("Creating sell order for asset ID: {}", asset_id);
    let url = format!("{base_url}/v1/taproot-assets/rfq/sellorder/asset-id/{asset_id}");
    let response = client
        .post(&url)
        .header("Grpc-Metadata-macaroon", macaroon_hex)
        .json(&request)
        .send()
        .await?;
    
    if !response.status().is_success() {
        let error_text = response.text().await?;
        return Err(AppError::RequestError(error_text));
    }
    
    let result = response.json::<Value>().await?;
    Ok(result)
}

// Axum handlers
pub async fn buy_offer_handler(
    State(state): State<AppState>,
    Path(asset_id): Path<String>,
    Json(request): Json<BuyOfferRequest>,
) -> Result<Json<Value>, StatusCode> {
    match buy_offer(
        &state.http_client,
        &state.base_url.0,
        &state.macaroon_hex.0,
        request,
        &asset_id,
    ).await {
        Ok(result) => Ok(Json(result)),
        Err(e) => {
            error!("Buy offer failed: {}", e);
            Err(e.status_code())
        }
    }
}

pub async fn buy_order_handler(
    State(state): State<AppState>,
    Path(asset_id): Path<String>,
    Json(request): Json<BuyOrderRequest>,
) -> Result<Json<Value>, StatusCode> {
    match buy_order(
        &state.http_client,
        &state.base_url.0,
        &state.macaroon_hex.0,
        request,
        &asset_id,
    ).await {
        Ok(result) => Ok(Json(result)),
        Err(e) => {
            error!("Buy order failed: {}", e);
            Err(e.status_code())
        }
    }
}

pub async fn notifications_handler(
    State(state): State<AppState>,
) -> Result<Json<Value>, StatusCode> {
    match get_notifications(
        &state.http_client,
        &state.base_url.0,
        &state.macaroon_hex.0,
    ).await {
        Ok(result) => Ok(Json(result)),
        Err(e) => {
            error!("Get notifications failed: {}", e);
            Err(e.status_code())
        }
    }
}

pub async fn asset_rates_handler(
    State(state): State<AppState>,
) -> Result<Json<Value>, StatusCode> {
    match get_asset_rates(
        &state.http_client,
        &state.base_url.0,
        &state.macaroon_hex.0,
    ).await {
        Ok(result) => Ok(Json(result)),
        Err(e) => {
            error!("Get asset rates failed: {}", e);
            Err(e.status_code())
        }
    }
}

pub async fn peer_quotes_handler(
    State(state): State<AppState>,
) -> Result<Json<Value>, StatusCode> {
    match get_peer_quotes(
        &state.http_client,
        &state.base_url.0,
        &state.macaroon_hex.0,
    ).await {
        Ok(result) => Ok(Json(result)),
        Err(e) => {
            error!("Get peer quotes failed: {}", e);
            Err(e.status_code())
        }
    }
}

pub async fn sell_offer_handler(
    State(state): State<AppState>,
    Path(asset_id): Path<String>,
    Json(request): Json<SellOfferRequest>,
) -> Result<Json<Value>, StatusCode> {
    match sell_offer(
        &state.http_client,
        &state.base_url.0,
        &state.macaroon_hex.0,
        request,
        &asset_id,
    ).await {
        Ok(result) => Ok(Json(result)),
        Err(e) => {
            error!("Sell offer failed: {}", e);
            Err(e.status_code())
        }
    }
}

pub async fn sell_order_handler(
    State(state): State<AppState>,
    Path(asset_id): Path<String>,
    Json(request): Json<SellOrderRequest>,
) -> Result<Json<Value>, StatusCode> {
    match sell_order(
        &state.http_client,
        &state.base_url.0,
        &state.macaroon_hex.0,
        request,
        &asset_id,
    ).await {
        Ok(result) => Ok(Json(result)),
        Err(e) => {
            error!("Sell order failed: {}", e);
            Err(e.status_code())
        }
    }
}

// WebSocket handler for RFQ events
pub async fn rfq_events_ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
) -> Response {
    ws.on_upgrade(|socket| handle_rfq_websocket(socket, state))
}

async fn handle_rfq_websocket(socket: WebSocket, state: AppState) {
    let (mut sender, mut receiver) = socket.split();
    
    info!("Establishing WebSocket connection for RFQ event notifications");
    
    // Send initial acknowledgment
    if let Err(e) = sender.send(Message::Text("{}".to_string())).await {
        error!("Failed to send initial message: {}", e);
        return;
    }
    
    let client = state.http_client.clone();
    let base_url = state.base_url.0.clone();
    let macaroon_hex = state.macaroon_hex.0.clone();
    
    // Create a channel for communication between polling task and main handler
    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<String>();
    
    // Create polling task
    let poll_task = tokio::spawn(async move {
        let mut poll_interval = interval(Duration::from_secs(5)); // Default 5 seconds
        
        loop {
            poll_interval.tick().await;
            
            match get_notifications(&client, &base_url, &macaroon_hex).await {
                Ok(events) => {
                    let event_json = serde_json::to_string(&events)
                        .unwrap_or_else(|_| "{}".to_string());
                    
                    if tx.send(event_json).is_err() {
                        error!("Failed to send RFQ event to channel");
                        break;
                    }
                }
                Err(e) => {
                    error!("Failed to fetch RFQ notifications: {}", e);
                    
                    let error_msg = serde_json::json!({
                        "error": e.to_string(),
                        "type": "rfq_notification_error"
                    });
                    
                    if tx.send(error_msg.to_string()).is_err() {
                        error!("Failed to send error message to channel");
                        break;
                    }
                }
            }
        }
    });
    
    // Handle incoming messages and keep connection alive
    let mut ping_interval = interval(Duration::from_secs(30));
    
    loop {
        tokio::select! {
            msg = receiver.next() => {
                match msg {
                    Some(Ok(Message::Text(_text))) => {
                        // Client message received - RFQ notifications don't need specific handling
                        info!("Received client message for RFQ notifications");
                    },
                    Some(Ok(Message::Close(_))) => {
                        info!("WebSocket connection closed by client");
                        break;
                    },
                    Some(Ok(Message::Ping(data))) => {
                        if sender.send(Message::Pong(data)).await.is_err() {
                            error!("Failed to send pong");
                            break;
                        }
                    },
                    Some(Err(e)) => {
                        error!("WebSocket error: {}", e);
                        break;
                    },
                    None => {
                        info!("WebSocket stream ended");
                        break;
                    },
                    _ => {}
                }
            },
            event_msg = rx.recv() => {
                if let Some(msg) = event_msg {
                    if sender.send(Message::Text(msg)).await.is_err() {
                        error!("Failed to send event message to client");
                        break;
                    }
                } else {
                    // Channel closed
                    break;
                }
            },
            _ = ping_interval.tick() => {
                if sender.send(Message::Ping(b"ping".to_vec())).await.is_err() {
                    error!("Failed to send ping");
                    break;
                }
            },
        }
    }
    
    // Clean up polling task
    poll_task.abort();
}
