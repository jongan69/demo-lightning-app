use anyhow::Result;
use reqwest::Client;
use serde_json::json;
use tracing::{error, info};

pub struct TapdClient {
    gateway_url: String,
    client: Client,
}

impl TapdClient {
    pub fn new(gateway_url: String) -> Self {
        Self {
            gateway_url,
            client: Client::new(),
        }
    }

    pub async fn list_assets(&self) -> Result<Vec<crate::types::TaprootAsset>> {
        info!("Listing assets from gateway at {}", self.gateway_url);
        
        let url = format!("{}/v1/taproot-assets/assets", self.gateway_url);
        let response = self.client.get(&url).send().await?;
        
        if !response.status().is_success() {
            let error_text = response.text().await?;
            error!("Failed to list assets: {}", error_text);
            return Err(anyhow::anyhow!("Failed to list assets: {}", error_text));
        }
        
        let json: serde_json::Value = response.json().await?;
        let empty_vec = vec![];
        let assets = json["assets"].as_array().unwrap_or(&empty_vec);
        
        let mut result = Vec::new();
        for asset in assets {
            if let Ok(taproot_asset) = serde_json::from_value::<crate::types::TaprootAsset>(asset.clone()) {
                result.push(taproot_asset);
            }
        }
        
        Ok(result)
    }

    pub async fn send_asset(&self, transfer: &crate::types::AssetTransfer) -> Result<String> {
        info!("Sending asset {} to {} via gateway", transfer.asset_id, transfer.destination);
        
        let url = format!("{}/v1/taproot-assets/send", self.gateway_url);
        let payload = json!({
            "tap_addrs": [transfer.destination],
            "fee_rate": transfer.fee_rate.unwrap_or(5)
        });
        
        let response = self.client
            .post(&url)
            .json(&payload)
            .send()
            .await?;
        
        if !response.status().is_success() {
            let error_text = response.text().await?;
            error!("Failed to send asset: {}", error_text);
            return Err(anyhow::anyhow!("Failed to send asset: {}", error_text));
        }
        
        let json: serde_json::Value = response.json().await?;
        let tx_id = json["transfer"]["anchor_tx_hash"]
            .as_str()
            .unwrap_or("unknown")
            .to_string();
        
        Ok(tx_id)
    }

    pub async fn create_address(&self, asset_id: &str, amount: u64) -> Result<String> {
        info!("Creating address for asset {} amount {}", asset_id, amount);
        
        let url = format!("{}/v1/taproot-assets/addrs", self.gateway_url);
        let payload = json!({
            "asset_id": asset_id,
            "amt": amount.to_string()
        });
        
        let response = self.client
            .post(&url)
            .json(&payload)
            .send()
            .await?;
        
        if !response.status().is_success() {
            let error_text = response.text().await?;
            error!("Failed to create address: {}", error_text);
            return Err(anyhow::anyhow!("Failed to create address: {}", error_text));
        }
        
        let json: serde_json::Value = response.json().await?;
        let address = json["encoded"]
            .as_str()
            .unwrap_or("unknown")
            .to_string();
        
        Ok(address)
    }

    pub async fn mint_asset(&self, name: &str, amount: u64, asset_type: &str) -> Result<String> {
        info!("Minting asset {} with amount {}", name, amount);
        
        let url = format!("{}/v1/taproot-assets/assets", self.gateway_url);
        let payload = json!({
            "asset": {
                "asset_type": asset_type,
                "name": name,
                "amount": amount.to_string()
            },
            "short_response": true
        });
        
        let response = self.client
            .post(&url)
            .json(&payload)
            .send()
            .await?;
        
        if !response.status().is_success() {
            let error_text = response.text().await?;
            error!("Failed to mint asset: {}", error_text);
            return Err(anyhow::anyhow!("Failed to mint asset: {}", error_text));
        }
        
        let json: serde_json::Value = response.json().await?;
        let batch_key = json["pending_batch"]["batch_key"]
            .as_str()
            .unwrap_or("unknown")
            .to_string();
        
        Ok(batch_key)
    }

    pub async fn get_balance(&self) -> Result<serde_json::Value> {
        info!("Getting asset balance from gateway");
        
        let url = format!("{}/v1/taproot-assets/assets/balance", self.gateway_url);
        let response = self.client.get(&url).send().await?;
        
        if !response.status().is_success() {
            let error_text = response.text().await?;
            error!("Failed to get balance: {}", error_text);
            return Err(anyhow::anyhow!("Failed to get balance: {}", error_text));
        }
        
        let json: serde_json::Value = response.json().await?;
        Ok(json)
    }

    pub async fn get_info(&self) -> Result<serde_json::Value> {
        info!("Getting taproot assets info from gateway");
        
        let url = format!("{}/v1/taproot-assets/info", self.gateway_url);
        let response = self.client.get(&url).send().await?;
        
        if !response.status().is_success() {
            let error_text = response.text().await?;
            error!("Failed to get info: {}", error_text);
            return Err(anyhow::anyhow!("Failed to get info: {}", error_text));
        }
        
        let json: serde_json::Value = response.json().await?;
        Ok(json)
    }

    pub async fn list_addresses(&self) -> Result<serde_json::Value> {
        info!("Listing addresses from gateway");
        
        let url = format!("{}/v1/taproot-assets/addrs", self.gateway_url);
        let response = self.client.get(&url).send().await?;
        
        if !response.status().is_success() {
            let error_text = response.text().await?;
            error!("Failed to list addresses: {}", error_text);
            return Err(anyhow::anyhow!("Failed to list addresses: {}", error_text));
        }
        
        let json: serde_json::Value = response.json().await?;
        Ok(json)
    }

    pub async fn new_address(&self, payload: serde_json::Value) -> Result<serde_json::Value> {
        info!("Creating new address via gateway");
        
        let url = format!("{}/v1/taproot-assets/addrs", self.gateway_url);
        let response = self.client
            .post(&url)
            .json(&payload)
            .send()
            .await?;
        
        if !response.status().is_success() {
            let error_text = response.text().await?;
            error!("Failed to create new address: {}", error_text);
            return Err(anyhow::anyhow!("Failed to create new address: {}", error_text));
        }
        
        let json: serde_json::Value = response.json().await?;
        Ok(json)
    }

    pub async fn mint_asset_raw(&self, payload: serde_json::Value) -> Result<serde_json::Value> {
        info!("Minting asset via gateway with raw payload");
        
        let url = format!("{}/v1/taproot-assets/assets", self.gateway_url);
        let response = self.client
            .post(&url)
            .json(&payload)
            .send()
            .await?;
        
        if !response.status().is_success() {
            let error_text = response.text().await?;
            error!("Failed to mint asset: {}", error_text);
            return Err(anyhow::anyhow!("Failed to mint asset: {}", error_text));
        }
        
        let json: serde_json::Value = response.json().await?;
        Ok(json)
    }
}