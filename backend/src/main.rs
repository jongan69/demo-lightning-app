use axum::{
    response::Json,
    routing::get,
    Router,
};
use tower_http::cors::CorsLayer;
use tracing::info;

mod api;
mod storage;
mod taproot;
mod types;

use api::routes;
use storage::database;
use types::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Load environment variables
    dotenv::dotenv().ok();

    // For development, create a simple in-memory state instead of database
    info!("Running in development mode without database");

    // Build application
    let app = Router::new()
        .route("/health", get(health_check))
        .nest("/api", routes::create_routes())
        .layer(CorsLayer::permissive());

    // Start server
    let host = std::env::var("SERVER_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = std::env::var("SERVER_PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = format!("{}:{}", host, port);

    info!("Starting server on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn health_check() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "healthy",
        "timestamp": chrono::Utc::now(),
        "service": "taproot-backend"
    }))
}
