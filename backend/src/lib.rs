pub mod api;
pub mod config;
pub mod crypto;
pub mod error;
pub mod gateway;
pub mod storage;
pub mod taproot;
pub mod types;

// Re-export main types for easier testing
pub use types::{AppState, ApiResponse, TaprootAsset, AssetTransfer, Transaction};
pub use error::AppError;
pub use config::Config; 