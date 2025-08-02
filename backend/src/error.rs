use thiserror::Error;
use axum::http::StatusCode;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Environment variable error: {0}")]
    EnvVarError(String),
    
    #[error("Validation error: {0}")]
    ValidationError(String),
    
    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Request error: {0}")]
    RequestError(String),
}

impl AppError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            AppError::EnvVarError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::ValidationError(_) => StatusCode::BAD_REQUEST,
            AppError::InvalidInput(_) => StatusCode::BAD_REQUEST,
            AppError::RequestError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl From<std::env::VarError> for AppError {
    fn from(err: std::env::VarError) -> Self {
        AppError::EnvVarError(err.to_string())
    }
}

impl From<reqwest::Error> for AppError {
    fn from(err: reqwest::Error) -> Self {
        AppError::RequestError(err.to_string())
    }
}

impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        AppError::RequestError(err.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_env_var_error_creation() {
        let error = AppError::EnvVarError("TEST_VAR not found".to_string());
        assert!(matches!(error, AppError::EnvVarError(_)));
        assert_eq!(error.to_string(), "Environment variable error: TEST_VAR not found");
    }

    #[test]
    fn test_validation_error_creation() {
        let error = AppError::ValidationError("Invalid configuration".to_string());
        assert!(matches!(error, AppError::ValidationError(_)));
        assert_eq!(error.to_string(), "Validation error: Invalid configuration");
    }

    #[test]
    fn test_invalid_input_error_creation() {
        let error = AppError::InvalidInput("Invalid public key format".to_string());
        assert!(matches!(error, AppError::InvalidInput(_)));
        assert_eq!(error.to_string(), "Invalid input: Invalid public key format");
    }

    #[test]
    fn test_from_var_error() {
        // Create a VarError by trying to get a non-existent environment variable
        let var_error = env::var("NON_EXISTENT_VAR").unwrap_err();
        let app_error: AppError = var_error.into();
        
        assert!(matches!(app_error, AppError::EnvVarError(_)));
        assert!(app_error.to_string().contains("Environment variable error"));
    }

    #[test]
    fn test_error_debug_formatting() {
        let error = AppError::ValidationError("Test error".to_string());
        let debug_output = format!("{:?}", error);
        assert!(debug_output.contains("ValidationError"));
        assert!(debug_output.contains("Test error"));
    }

    #[test]
    fn test_error_display_formatting() {
        let error = AppError::InvalidInput("Test input error".to_string());
        let display_output = error.to_string();
        assert_eq!(display_output, "Invalid input: Test input error");
    }

    #[test]
    fn test_error_pattern_matching() {
        let env_error = AppError::EnvVarError("test".to_string());
        let validation_error = AppError::ValidationError("test".to_string());
        let input_error = AppError::InvalidInput("test".to_string());

        match env_error {
            AppError::EnvVarError(_) => assert!(true),
            _ => assert!(false, "Should match EnvVarError"),
        }

        match validation_error {
            AppError::ValidationError(_) => assert!(true),
            _ => assert!(false, "Should match ValidationError"),
        }

        match input_error {
            AppError::InvalidInput(_) => assert!(true),
            _ => assert!(false, "Should match InvalidInput"),
        }
    }

    #[test]
    fn test_error_with_empty_strings() {
        let env_error = AppError::EnvVarError("".to_string());
        let validation_error = AppError::ValidationError("".to_string());
        let input_error = AppError::InvalidInput("".to_string());

        assert_eq!(env_error.to_string(), "Environment variable error: ");
        assert_eq!(validation_error.to_string(), "Validation error: ");
        assert_eq!(input_error.to_string(), "Invalid input: ");
    }

    #[test]
    fn test_error_with_special_characters() {
        let error = AppError::InvalidInput("Error with special chars: !@#$%^&*()".to_string());
        assert_eq!(error.to_string(), "Invalid input: Error with special chars: !@#$%^&*()");
    }

    #[test]
    fn test_error_with_unicode() {
        let error = AppError::ValidationError("Unicode test: 🚀 测试".to_string());
        assert_eq!(error.to_string(), "Validation error: Unicode test: 🚀 测试");
    }
}