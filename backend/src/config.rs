use crate::error::AppError;
use serde::Deserialize;
use std::path::Path;

#[derive(Clone, Deserialize, Debug)]
pub struct Config {
    pub taproot_assets_host: String,
    pub macaroon_path: String,
    pub lnd_macaroon_path: String,
    pub tls_verify: bool,
    pub cors_origins: Vec<String>,
    pub server_address: String,
    pub request_timeout_secs: u64,
    pub rate_limit_per_minute: usize,
    pub rfq_poll_interval_secs: u64,
}

impl Config {
    #[allow(dead_code)]
    pub fn load() -> Result<Self, AppError> {
        // Load host configuration
        let taproot_assets_host =
            std::env::var("TAPROOT_ASSETS_HOST").unwrap_or_else(|_| "127.0.0.1:8289".to_string());

        // Load authentication paths
        let macaroon_path = std::env::var("TAPD_MACAROON_PATH")?;
        let lnd_macaroon_path = std::env::var("LND_MACAROON_PATH")?;

        // Security settings - TLS verification defaults to true for production safety
        let tls_verify = std::env::var("TLS_VERIFY")
            .unwrap_or_else(|_| "true".to_string())
            .parse::<bool>()
            .unwrap_or(true);

        // CORS configuration
        let cors_origins = std::env::var("CORS_ORIGINS")
            .unwrap_or_else(|_| "http://localhost:5173,http://127.0.0.1:5173".to_string())
            .split(',')
            .map(|s| s.trim().to_string())
            .collect();

        // Server configuration
        let server_address =
            std::env::var("SERVER_ADDRESS").unwrap_or_else(|_| "127.0.0.1:8080".to_string());

        // Request timeout configuration
        let request_timeout_secs = std::env::var("REQUEST_TIMEOUT_SECS")
            .unwrap_or_else(|_| "30".to_string())
            .parse::<u64>()
            .unwrap_or(30);

        // Rate limiting configuration
        let rate_limit_per_minute = std::env::var("RATE_LIMIT_PER_MINUTE")
            .unwrap_or_else(|_| "100".to_string())
            .parse::<usize>()
            .unwrap_or(100);

        // RFQ polling interval configuration
        let rfq_poll_interval_secs = std::env::var("RFQ_POLL_INTERVAL_SECS")
            .unwrap_or_else(|_| "5".to_string())
            .parse::<u64>()
            .unwrap_or(5);

        // Validate paths exist
        if !Path::new(&macaroon_path).exists() {
            return Err(AppError::ValidationError(format!(
                "Tapd macaroon file does not exist at path: {macaroon_path}. Please check TAPD_MACAROON_PATH in your .env file."
            )));
        }
        if !Path::new(&lnd_macaroon_path).exists() {
            return Err(AppError::ValidationError(format!(
                "LND macaroon file does not exist at path: {lnd_macaroon_path}. Please check LND_MACAROON_PATH in your .env file."
            )));
        }

        let config = Config {
            taproot_assets_host,
            macaroon_path,
            lnd_macaroon_path,
            tls_verify,
            cors_origins,
            server_address,
            request_timeout_secs,
            rate_limit_per_minute,
            rfq_poll_interval_secs,
        };

        // Validate configuration
        config.validate()?;

        Ok(config)
    }

    #[allow(dead_code)]
    pub fn validate(&self) -> Result<(), AppError> {
        // Validate host configuration
        if self.taproot_assets_host.is_empty() {
            return Err(AppError::ValidationError(
                "TAPROOT_ASSETS_HOST cannot be empty".to_string(),
            ));
        }

        // Validate authentication paths
        if self.macaroon_path.is_empty() {
            return Err(AppError::ValidationError(
                "TAPD_MACAROON_PATH cannot be empty".to_string(),
            ));
        }
        if self.lnd_macaroon_path.is_empty() {
            return Err(AppError::ValidationError(
                "LND_MACAROON_PATH cannot be empty".to_string(),
            ));
        }

        // Validate server configuration
        if self.server_address.is_empty() {
            return Err(AppError::ValidationError(
                "SERVER_ADDRESS cannot be empty".to_string(),
            ));
        }

        // Validate timeout configuration
        if self.request_timeout_secs == 0 {
            return Err(AppError::ValidationError(
                "REQUEST_TIMEOUT_SECS must be greater than 0".to_string(),
            ));
        }

        // Validate rate limiting configuration
        if self.rate_limit_per_minute == 0 {
            return Err(AppError::ValidationError(
                "RATE_LIMIT_PER_MINUTE must be greater than 0".to_string(),
            ));
        }

        // Validate RFQ polling interval
        if self.rfq_poll_interval_secs == 0 {
            return Err(AppError::ValidationError(
                "RFQ_POLL_INTERVAL_SECS must be greater than 0".to_string(),
            ));
        }

        Ok(())
    }

    /// Create a test configuration for unit testing
    #[cfg(test)]
    pub fn test_config() -> Self {
        Config {
            taproot_assets_host: "127.0.0.1:8289".to_string(),
            macaroon_path: "/tmp/test_macaroon".to_string(),
            lnd_macaroon_path: "/tmp/test_lnd_macaroon".to_string(),
            tls_verify: true,
            cors_origins: vec!["http://localhost:5173".to_string()],
            server_address: "127.0.0.1:8080".to_string(),
            request_timeout_secs: 30,
            rate_limit_per_minute: 100,
            rfq_poll_interval_secs: 5,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use tempfile::NamedTempFile;

    #[test]
    fn test_config_validation_success() {
        let config = Config::test_config();
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_config_validation_empty_host() {
        let mut config = Config::test_config();
        config.taproot_assets_host = "".to_string();
        let result = config.validate();
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), AppError::ValidationError(_)));
    }

    #[test]
    fn test_config_validation_empty_macaroon_path() {
        let mut config = Config::test_config();
        config.macaroon_path = "".to_string();
        let result = config.validate();
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), AppError::ValidationError(_)));
    }

    #[test]
    fn test_config_validation_empty_lnd_macaroon_path() {
        let mut config = Config::test_config();
        config.lnd_macaroon_path = "".to_string();
        let result = config.validate();
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), AppError::ValidationError(_)));
    }

    #[test]
    fn test_config_validation_empty_server_address() {
        let mut config = Config::test_config();
        config.server_address = "".to_string();
        let result = config.validate();
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), AppError::ValidationError(_)));
    }

    #[test]
    fn test_config_validation_zero_timeout() {
        let mut config = Config::test_config();
        config.request_timeout_secs = 0;
        let result = config.validate();
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), AppError::ValidationError(_)));
    }

    #[test]
    fn test_config_validation_zero_rate_limit() {
        let mut config = Config::test_config();
        config.rate_limit_per_minute = 0;
        let result = config.validate();
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), AppError::ValidationError(_)));
    }

    #[test]
    fn test_config_validation_zero_rfq_interval() {
        let mut config = Config::test_config();
        config.rfq_poll_interval_secs = 0;
        let result = config.validate();
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), AppError::ValidationError(_)));
    }

    #[test]
    fn test_config_load_with_valid_env_vars() {
        // Create temporary files for macaroons
        let tapd_macaroon = NamedTempFile::new().unwrap();
        let lnd_macaroon = NamedTempFile::new().unwrap();
        
        // Set environment variables
        env::set_var("TAPD_MACAROON_PATH", tapd_macaroon.path().to_str().unwrap());
        env::set_var("LND_MACAROON_PATH", lnd_macaroon.path().to_str().unwrap());
        env::set_var("TAPROOT_ASSETS_HOST", "test.host:8289");
        env::set_var("SERVER_ADDRESS", "test.server:8080");
        env::set_var("REQUEST_TIMEOUT_SECS", "60");
        env::set_var("RATE_LIMIT_PER_MINUTE", "200");
        env::set_var("RFQ_POLL_INTERVAL_SECS", "10");
        env::set_var("TLS_VERIFY", "false");
        env::set_var("CORS_ORIGINS", "http://test.com,https://test.com");

        let result = Config::load();
        assert!(result.is_ok());

        let config = result.unwrap();
        assert_eq!(config.taproot_assets_host, "test.host:8289");
        assert_eq!(config.server_address, "test.server:8080");
        assert_eq!(config.request_timeout_secs, 60);
        assert_eq!(config.rate_limit_per_minute, 200);
        assert_eq!(config.rfq_poll_interval_secs, 10);
        assert_eq!(config.tls_verify, false);
        assert_eq!(config.cors_origins, vec!["http://test.com", "https://test.com"]);

        // Clean up
        env::remove_var("TAPD_MACAROON_PATH");
        env::remove_var("LND_MACAROON_PATH");
        env::remove_var("TAPROOT_ASSETS_HOST");
        env::remove_var("SERVER_ADDRESS");
        env::remove_var("REQUEST_TIMEOUT_SECS");
        env::remove_var("RATE_LIMIT_PER_MINUTE");
        env::remove_var("RFQ_POLL_INTERVAL_SECS");
        env::remove_var("TLS_VERIFY");
        env::remove_var("CORS_ORIGINS");
    }

    #[test]
    fn test_config_load_with_missing_macaroon_files() {
        env::set_var("TAPD_MACAROON_PATH", "/nonexistent/path");
        env::set_var("LND_MACAROON_PATH", "/nonexistent/path");

        let result = Config::load();
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), AppError::ValidationError(_)));

        // Clean up
        env::remove_var("TAPD_MACAROON_PATH");
        env::remove_var("LND_MACAROON_PATH");
    }

    #[test]
    fn test_config_load_with_missing_env_vars() {
        // Don't set any environment variables
        let result = Config::load();
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), AppError::EnvVarError(_)));
    }

    #[test]
    fn test_config_default_values() {
        // Create temporary files for macaroons
        let tapd_macaroon = NamedTempFile::new().unwrap();
        let lnd_macaroon = NamedTempFile::new().unwrap();
        
        // Set only required environment variables
        env::set_var("TAPD_MACAROON_PATH", tapd_macaroon.path().to_str().unwrap());
        env::set_var("LND_MACAROON_PATH", lnd_macaroon.path().to_str().unwrap());

        let result = Config::load();
        assert!(result.is_ok());

        let config = result.unwrap();
        assert_eq!(config.taproot_assets_host, "127.0.0.1:8289");
        assert_eq!(config.server_address, "127.0.0.1:8080");
        assert_eq!(config.request_timeout_secs, 30);
        assert_eq!(config.rate_limit_per_minute, 100);
        assert_eq!(config.rfq_poll_interval_secs, 5);
        assert_eq!(config.tls_verify, true);
        assert_eq!(config.cors_origins, vec!["http://localhost:5173", "http://127.0.0.1:5173"]);

        // Clean up
        env::remove_var("TAPD_MACAROON_PATH");
        env::remove_var("LND_MACAROON_PATH");
    }
}
