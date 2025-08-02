use axum::{
    routing::{get, post},
    Router,
};
use crate::api::handlers;

pub fn create_routes() -> Router {
    Router::new()
        .route("/assets", get(handlers::list_assets))
        .route("/assets/:id/balance", get(handlers::get_asset_balance))
        .route("/assets/send", post(handlers::send_asset))
        .route("/assets/invoice", post(handlers::create_asset_invoice))
        .route("/transactions", get(handlers::get_transactions))
}