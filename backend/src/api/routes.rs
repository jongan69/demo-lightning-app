use axum::{
    routing::{get, post},
    Router,
};
use crate::api::handlers;
use crate::types::AppState;

pub fn create_routes() -> Router<AppState> {
    Router::new()
        .route("/assets", get(handlers::list_assets))
        .route("/assets/balance", get(handlers::get_asset_balance))
        .route("/assets/send", post(handlers::send_asset))
        .route("/assets/address", post(handlers::create_asset_address))
        .route("/assets/mint", post(handlers::mint_asset))
        .route("/transactions", get(handlers::get_transactions))
}