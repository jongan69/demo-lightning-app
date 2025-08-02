use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Clone)]
pub struct AppState {
    pub db_pool: PgPool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaprootAsset {
    pub asset_id: String,
    pub name: String,
    pub balance: u64,
    pub decimals: u8,
    pub asset_type: AssetType,
    pub meta_data: Option<AssetMetaData>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum AssetType {
    Normal,
    Collectible,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssetMetaData {
    pub description: Option<String>,
    pub image_url: Option<String>,
    pub issuer: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssetTransfer {
    pub asset_id: String,
    pub amount: u64,
    pub destination: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssetInvoice {
    pub asset_id: String,
    pub amount: u64,
    pub description: Option<String>,
    pub expiry: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Transaction {
    pub id: Uuid,
    pub tx_type: TransactionType,
    pub asset_id: Option<String>,
    pub amount: u64,
    pub status: TransactionStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TransactionType {
    Send,
    Receive,
    Issue,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TransactionStatus {
    Pending,
    Confirmed,
    Failed,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
    pub message: Option<String>,
}