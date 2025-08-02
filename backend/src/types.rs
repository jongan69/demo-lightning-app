use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Clone)]
pub struct AppState {
    pub tapd_client: std::sync::Arc<crate::taproot::client::TapdClient>,
    pub http_client: std::sync::Arc<reqwest::Client>,
    pub base_url: BaseUrl,
    pub macaroon_hex: MacaroonHex,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TaprootAsset {
    pub asset_id: String,
    pub name: String,
    pub balance: u64,
    pub decimals: u8,
    pub asset_type: AssetType,
    pub meta_data: Option<AssetMetaData>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum AssetType {
    Normal,
    Collectible,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
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
    pub fee_rate: Option<u32>,
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

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum TransactionType {
    Send,
    Receive,
    Issue,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
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

// Types for taproot gateway compatibility
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct BaseUrl(pub String);

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct MacaroonHex(pub String);

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_taproot_asset_serialization() {
        let asset = TaprootAsset {
            asset_id: "test_asset_id".to_string(),
            name: "Test Asset".to_string(),
            balance: 1000,
            decimals: 8,
            asset_type: AssetType::Normal,
            meta_data: Some(AssetMetaData {
                description: Some("Test description".to_string()),
                image_url: Some("https://example.com/image.png".to_string()),
                issuer: Some("Test Issuer".to_string()),
            }),
        };

        let json = serde_json::to_string(&asset).unwrap();
        let deserialized: TaprootAsset = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.asset_id, "test_asset_id");
        assert_eq!(deserialized.name, "Test Asset");
        assert_eq!(deserialized.balance, 1000);
        assert_eq!(deserialized.decimals, 8);
        assert!(matches!(deserialized.asset_type, AssetType::Normal));
        assert!(deserialized.meta_data.is_some());
        
        let meta = deserialized.meta_data.unwrap();
        assert_eq!(meta.description, Some("Test description".to_string()));
        assert_eq!(meta.image_url, Some("https://example.com/image.png".to_string()));
        assert_eq!(meta.issuer, Some("Test Issuer".to_string()));
    }

    #[test]
    fn test_taproot_asset_without_metadata() {
        let asset = TaprootAsset {
            asset_id: "test_asset_id".to_string(),
            name: "Test Asset".to_string(),
            balance: 1000,
            decimals: 8,
            asset_type: AssetType::Collectible,
            meta_data: None,
        };

        let json = serde_json::to_string(&asset).unwrap();
        let deserialized: TaprootAsset = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.asset_id, "test_asset_id");
        assert_eq!(deserialized.name, "Test Asset");
        assert_eq!(deserialized.balance, 1000);
        assert_eq!(deserialized.decimals, 8);
        assert!(matches!(deserialized.asset_type, AssetType::Collectible));
        assert!(deserialized.meta_data.is_none());
    }

    #[test]
    fn test_asset_transfer_serialization() {
        let transfer = AssetTransfer {
            asset_id: "test_asset_id".to_string(),
            amount: 100,
            destination: "test_destination".to_string(),
            fee_rate: Some(5),
        };

        let json = serde_json::to_string(&transfer).unwrap();
        let deserialized: AssetTransfer = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.asset_id, "test_asset_id");
        assert_eq!(deserialized.amount, 100);
        assert_eq!(deserialized.destination, "test_destination");
        assert_eq!(deserialized.fee_rate, Some(5));
    }

    #[test]
    fn test_asset_transfer_without_fee_rate() {
        let transfer = AssetTransfer {
            asset_id: "test_asset_id".to_string(),
            amount: 100,
            destination: "test_destination".to_string(),
            fee_rate: None,
        };

        let json = serde_json::to_string(&transfer).unwrap();
        let deserialized: AssetTransfer = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.asset_id, "test_asset_id");
        assert_eq!(deserialized.amount, 100);
        assert_eq!(deserialized.destination, "test_destination");
        assert_eq!(deserialized.fee_rate, None);
    }

    #[test]
    fn test_asset_invoice_serialization() {
        let invoice = AssetInvoice {
            asset_id: "test_asset_id".to_string(),
            amount: 100,
            description: Some("Test invoice".to_string()),
            expiry: Some(1234567890),
        };

        let json = serde_json::to_string(&invoice).unwrap();
        let deserialized: AssetInvoice = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.asset_id, "test_asset_id");
        assert_eq!(deserialized.amount, 100);
        assert_eq!(deserialized.description, Some("Test invoice".to_string()));
        assert_eq!(deserialized.expiry, Some(1234567890));
    }

    #[test]
    fn test_asset_invoice_without_optional_fields() {
        let invoice = AssetInvoice {
            asset_id: "test_asset_id".to_string(),
            amount: 100,
            description: None,
            expiry: None,
        };

        let json = serde_json::to_string(&invoice).unwrap();
        let deserialized: AssetInvoice = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.asset_id, "test_asset_id");
        assert_eq!(deserialized.amount, 100);
        assert_eq!(deserialized.description, None);
        assert_eq!(deserialized.expiry, None);
    }

    #[test]
    fn test_transaction_serialization() {
        let now = Utc::now();
        let transaction = Transaction {
            id: Uuid::new_v4(),
            tx_type: TransactionType::Send,
            asset_id: Some("test_asset_id".to_string()),
            amount: 100,
            status: TransactionStatus::Pending,
            created_at: now,
            updated_at: now,
        };

        let json = serde_json::to_string(&transaction).unwrap();
        let deserialized: Transaction = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.tx_type, TransactionType::Send);
        assert_eq!(deserialized.asset_id, Some("test_asset_id".to_string()));
        assert_eq!(deserialized.amount, 100);
        assert!(matches!(deserialized.status, TransactionStatus::Pending));
    }

    #[test]
    fn test_transaction_without_asset_id() {
        let now = Utc::now();
        let transaction = Transaction {
            id: Uuid::new_v4(),
            tx_type: TransactionType::Receive,
            asset_id: None,
            amount: 100,
            status: TransactionStatus::Confirmed,
            created_at: now,
            updated_at: now,
        };

        let json = serde_json::to_string(&transaction).unwrap();
        let deserialized: Transaction = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.tx_type, TransactionType::Receive);
        assert_eq!(deserialized.asset_id, None);
        assert_eq!(deserialized.amount, 100);
        assert!(matches!(deserialized.status, TransactionStatus::Confirmed));
    }

    #[test]
    fn test_api_response_success() {
        let response = ApiResponse {
            success: true,
            data: Some("test_data".to_string()),
            error: None,
            message: Some("Success message".to_string()),
        };

        let json = serde_json::to_string(&response).unwrap();
        let deserialized: ApiResponse<String> = serde_json::from_str(&json).unwrap();

        assert!(deserialized.success);
        assert_eq!(deserialized.data, Some("test_data".to_string()));
        assert_eq!(deserialized.error, None);
        assert_eq!(deserialized.message, Some("Success message".to_string()));
    }

    #[test]
    fn test_api_response_error() {
        let response: ApiResponse<String> = ApiResponse {
            success: false,
            data: None,
            error: Some("Error message".to_string()),
            message: Some("Failed".to_string()),
        };

        let json = serde_json::to_string(&response).unwrap();
        let deserialized: ApiResponse<String> = serde_json::from_str(&json).unwrap();

        assert!(!deserialized.success);
        assert_eq!(deserialized.data, None);
        assert_eq!(deserialized.error, Some("Error message".to_string()));
        assert_eq!(deserialized.message, Some("Failed".to_string()));
    }

    #[test]
    fn test_api_response_without_optional_fields() {
        let response: ApiResponse<String> = ApiResponse {
            success: true,
            data: None,
            error: None,
            message: None,
        };

        let json = serde_json::to_string(&response).unwrap();
        let deserialized: ApiResponse<String> = serde_json::from_str(&json).unwrap();

        assert!(deserialized.success);
        assert_eq!(deserialized.data, None);
        assert_eq!(deserialized.error, None);
        assert_eq!(deserialized.message, None);
    }

    #[test]
    fn test_asset_type_serialization() {
        let normal = AssetType::Normal;
        let collectible = AssetType::Collectible;

        let normal_json = serde_json::to_string(&normal).unwrap();
        let collectible_json = serde_json::to_string(&collectible).unwrap();

        let deserialized_normal: AssetType = serde_json::from_str(&normal_json).unwrap();
        let deserialized_collectible: AssetType = serde_json::from_str(&collectible_json).unwrap();

        assert!(matches!(deserialized_normal, AssetType::Normal));
        assert!(matches!(deserialized_collectible, AssetType::Collectible));
    }

    #[test]
    fn test_transaction_type_serialization() {
        let send = TransactionType::Send;
        let receive = TransactionType::Receive;
        let issue = TransactionType::Issue;

        let send_json = serde_json::to_string(&send).unwrap();
        let receive_json = serde_json::to_string(&receive).unwrap();
        let issue_json = serde_json::to_string(&issue).unwrap();

        let deserialized_send: TransactionType = serde_json::from_str(&send_json).unwrap();
        let deserialized_receive: TransactionType = serde_json::from_str(&receive_json).unwrap();
        let deserialized_issue: TransactionType = serde_json::from_str(&issue_json).unwrap();

        assert!(matches!(deserialized_send, TransactionType::Send));
        assert!(matches!(deserialized_receive, TransactionType::Receive));
        assert!(matches!(deserialized_issue, TransactionType::Issue));
    }

    #[test]
    fn test_transaction_status_serialization() {
        let pending = TransactionStatus::Pending;
        let confirmed = TransactionStatus::Confirmed;
        let failed = TransactionStatus::Failed;

        let pending_json = serde_json::to_string(&pending).unwrap();
        let confirmed_json = serde_json::to_string(&confirmed).unwrap();
        let failed_json = serde_json::to_string(&failed).unwrap();

        let deserialized_pending: TransactionStatus = serde_json::from_str(&pending_json).unwrap();
        let deserialized_confirmed: TransactionStatus = serde_json::from_str(&confirmed_json).unwrap();
        let deserialized_failed: TransactionStatus = serde_json::from_str(&failed_json).unwrap();

        assert!(matches!(deserialized_pending, TransactionStatus::Pending));
        assert!(matches!(deserialized_confirmed, TransactionStatus::Confirmed));
        assert!(matches!(deserialized_failed, TransactionStatus::Failed));
    }

    #[test]
    fn test_base_url_clone() {
        let base_url = BaseUrl("https://example.com".to_string());
        let cloned = base_url.clone();
        
        assert_eq!(base_url.0, cloned.0);
    }

    #[test]
    fn test_macaroon_hex_clone() {
        let macaroon = MacaroonHex("test_macaroon_hex".to_string());
        let cloned = macaroon.clone();
        
        assert_eq!(macaroon.0, cloned.0);
    }
}