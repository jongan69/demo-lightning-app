use axum::Router;
use tower_http::cors::CorsLayer;
use tracing::info;
use std::sync::Arc;

// Use the lib module structure
use taproot_backend::{
    api::routes,
    taproot::client::TapdClient,
    types::*,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Load environment variables
    dotenv::dotenv().ok();

    // Initialize Taproot Assets client
    let gateway_url = std::env::var("TAPROOT_GATEWAY_URL")
        .unwrap_or_else(|_| "http://127.0.0.1:8080".to_string());
    let tapd_client = Arc::new(TapdClient::new(gateway_url.clone()));
    
    info!("Connecting to Taproot Assets gateway");

    // Initialize HTTP client and configuration
    let http_client = Arc::new(reqwest::Client::new());
    let base_url = BaseUrl(gateway_url.clone());
    let macaroon_hex = MacaroonHex(
        std::env::var("TAPROOT_MACAROON_HEX")
            .unwrap_or_else(|_| "".to_string())
    );

    // Create application state
    let app_state = AppState {
        tapd_client,
        http_client,
        base_url,
        macaroon_hex,
    };

    // Build application
    let app = Router::new()
        .nest("/api", routes::create_routes())
        .merge(taproot_backend::gateway::routes::create_taproot_routes())
        .layer(CorsLayer::permissive())
        .with_state(app_state);

    // Start server
    let host = std::env::var("SERVER_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = std::env::var("SERVER_PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = format!("{}:{}", host, port);

    info!("Starting server on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}


