use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
};
use crate::types::{ApiResponse, TaprootAsset, AssetTransfer, Transaction, AppState};

pub async fn list_assets(
    State(app_state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<TaprootAsset>>>, StatusCode> {
    match app_state.tapd_client.list_assets().await {
        Ok(assets) => Ok(Json(ApiResponse {
            success: true,
            data: Some(assets),
            error: None,
            message: Some("Assets retrieved successfully".to_string()),
        })),
        Err(e) => Ok(Json(ApiResponse {
            success: false,
            data: None,
            error: Some(e.to_string()),
            message: Some("Failed to retrieve assets".to_string()),
        }))
    }
}

pub async fn get_asset_balance(
    State(app_state): State<AppState>,
) -> Result<Json<ApiResponse<serde_json::Value>>, StatusCode> {
    match app_state.tapd_client.get_balance().await {
        Ok(balance) => Ok(Json(ApiResponse {
            success: true,
            data: Some(balance),
            error: None,
            message: Some("Balance retrieved successfully".to_string()),
        })),
        Err(e) => Ok(Json(ApiResponse {
            success: false,
            data: None,
            error: Some(e.to_string()),
            message: Some("Failed to retrieve balance".to_string()),
        }))
    }
}

pub async fn send_asset(
    State(app_state): State<AppState>,
    Json(transfer): Json<AssetTransfer>,
) -> Result<Json<ApiResponse<String>>, StatusCode> {
    match app_state.tapd_client.send_asset(&transfer).await {
        Ok(tx_id) => Ok(Json(ApiResponse {
            success: true,
            data: Some(tx_id),
            error: None,
            message: Some("Asset transfer initiated".to_string()),
        })),
        Err(e) => Ok(Json(ApiResponse {
            success: false,
            data: None,
            error: Some(e.to_string()),
            message: Some("Failed to send asset".to_string()),
        }))
    }
}

pub async fn create_asset_address(
    State(app_state): State<AppState>,
    Json(request): Json<serde_json::Value>,
) -> Result<Json<ApiResponse<String>>, StatusCode> {
    let asset_id = request["asset_id"].as_str().unwrap_or("");
    let amount = request["amount"].as_u64().unwrap_or(0);
    
    match app_state.tapd_client.create_address(asset_id, amount).await {
        Ok(address) => Ok(Json(ApiResponse {
            success: true,
            data: Some(address),
            error: None,
            message: Some("Asset address created".to_string()),
        })),
        Err(e) => Ok(Json(ApiResponse {
            success: false,
            data: None,
            error: Some(e.to_string()),
            message: Some("Failed to create address".to_string()),
        }))
    }
}

pub async fn mint_asset(
    State(app_state): State<AppState>,
    Json(request): Json<serde_json::Value>,
) -> Result<Json<ApiResponse<String>>, StatusCode> {
    let name = request["name"].as_str().unwrap_or("");
    let amount = request["amount"].as_u64().unwrap_or(0);
    let asset_type = request["asset_type"].as_str().unwrap_or("NORMAL");
    
    match app_state.tapd_client.mint_asset(name, amount, asset_type).await {
        Ok(batch_key) => Ok(Json(ApiResponse {
            success: true,
            data: Some(batch_key),
            error: None,
            message: Some("Asset minting initiated".to_string()),
        })),
        Err(e) => Ok(Json(ApiResponse {
            success: false,
            data: None,
            error: Some(e.to_string()),
            message: Some("Failed to mint asset".to_string()),
        }))
    }
}

pub async fn get_transactions() -> Result<Json<ApiResponse<Vec<Transaction>>>, StatusCode> {
    // TODO: Implement actual transaction history from database
    let transactions = vec![];
    
    Ok(Json(ApiResponse {
        success: true,
        data: Some(transactions),
        error: None,
        message: Some("Transactions retrieved successfully".to_string()),
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_transactions() {
        // Simple test that doesn't require async or complex mocking
        let result = tokio::runtime::Runtime::new().unwrap().block_on(get_transactions());
        assert!(result.is_ok());

        let response = result.unwrap();
        let response_data = response.0;
        
        assert!(response_data.success);
        assert!(response_data.data.is_some());
        assert!(response_data.error.is_none());
        assert_eq!(response_data.message, Some("Transactions retrieved successfully".to_string()));

        let transactions = response_data.data.unwrap();
        assert_eq!(transactions.len(), 0); // Currently returns empty vector
    }
}