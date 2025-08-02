use axum::{
    routing::{get, post, any},
    Router,
};
use crate::types::AppState;

use super::{health, assets, addresses, info, wallet, burn, channels, events, rfq};

pub fn create_taproot_routes() -> Router<AppState> {
    Router::new()
        // Health endpoints
        .route("/health", get(health::health))
        .route("/readiness", get(health::readiness))
        
        // Taproot Assets API endpoints under /v1/taproot-assets
        .nest("/v1/taproot-assets", 
            Router::new()
                // Core endpoints - these will be implemented as needed
                .route("/assets/list", get(assets::list_assets))
                .route("/assets/mint", post(assets::mint_asset))
                .route("/addresses/new", post(addresses::new_address))
                .route("/addresses/list", get(addresses::list_addresses))
                .route("/info", get(info::get_info))
                .route("/wallet/balance", get(wallet::get_balance))
                .route("/burn", post(burn::burn))
                .route("/burns", get(burn::list))
                // Channel endpoints
                .nest("/channels", channels::create_channels_routes())
                // Add more routes as needed...
                // RFQ endpoints
                .route("/rfq/buyoffer/asset-id/:asset_id", post(rfq::buy_offer_handler))
                .route("/rfq/buyorder/asset-id/:asset_id", post(rfq::buy_order_handler))
                .route("/rfq/selloffer/asset-id/:asset_id", post(rfq::sell_offer_handler))
                .route("/rfq/sellorder/asset-id/:asset_id", post(rfq::sell_order_handler))
                .route("/rfq/ntfs", post(rfq::notifications_handler))
                .route("/rfq/priceoracle/assetrates", get(rfq::asset_rates_handler))
                .route("/rfq/quotes/peeraccepted", get(rfq::peer_quotes_handler))
                .route("/rfq/events", any(rfq::rfq_events_ws_handler))
        )
        // Event endpoints (top level)
        .nest("/events", events::create_events_routes())
}