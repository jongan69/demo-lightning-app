# Testing Guide for Taproot Backend

This document provides a comprehensive guide to testing the Taproot Backend application.

## Table of Contents

1. [Overview](#overview)
2. [Test Structure](#test-structure)
3. [Running Tests](#running-tests)
4. [Test Types](#test-types)
5. [Writing Tests](#writing-tests)
6. [Test Utilities](#test-utilities)
7. [Best Practices](#best-practices)
8. [Troubleshooting](#troubleshooting)

## Overview

The test suite is designed to ensure the reliability, correctness, and performance of the Taproot Backend. It includes:

- **Unit Tests**: Test individual functions and modules in isolation
- **Integration Tests**: Test the interaction between different components
- **API Tests**: Test HTTP endpoints and request/response handling
- **Error Handling Tests**: Test various error scenarios
- **Configuration Tests**: Test configuration loading and validation

## Test Structure

```
backend/
├── src/
│   ├── lib.rs              # Library entry point for testing
│   ├── config.rs           # Configuration module with tests
│   ├── crypto.rs           # Crypto module with tests
│   ├── error.rs            # Error handling with tests
│   ├── types.rs            # Type definitions with tests
│   └── api/
│       └── handlers.rs     # API handlers with tests
├── tests/
│   ├── integration_tests.rs # Integration tests
│   └── common/
│       └── mod.rs          # Common test utilities
├── scripts/
│   └── run_tests.sh       # Test runner script
└── TESTING.md             # This file
```

## Running Tests

### Quick Start

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_name

# Run tests in a specific module
cargo test config::tests
```

### Using the Test Runner Script

```bash
# Make script executable (first time only)
chmod +x scripts/run_tests.sh

# Run all tests
./scripts/run_tests.sh

# Run specific test types
./scripts/run_tests.sh unit
./scripts/run_tests.sh integration
./scripts/run_tests.sh coverage

# Get help
./scripts/run_tests.sh help
```

### Test Runner Options

- `unit` - Run only unit tests
- `integration` - Run only integration tests
- `coverage` - Generate coverage report
- `clippy` - Run clippy checks
- `format` - Run format checks
- `audit` - Run security audit
- `clean` - Clean the project
- `help` - Show help message

## Test Types

### 1. Unit Tests

Unit tests are located within each module using the `#[cfg(test)]` attribute.

**Example:**
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function_success() {
        let result = my_function("test");
        assert_eq!(result, "expected");
    }

    #[test]
    fn test_function_failure() {
        let result = my_function("");
        assert!(result.is_err());
    }
}
```

### 2. Integration Tests

Integration tests are in the `tests/` directory and test the application as a whole.

**Example:**
```rust
use axum::{
    body::Body,
    http::{Request, StatusCode},
    Router,
};
use tower::ServiceExt;

#[tokio::test]
async fn test_api_endpoint() {
    let app = create_test_app();
    
    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/test")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}
```

### 3. Mock Tests

The test suite includes comprehensive mocking capabilities for external dependencies.

**Example:**
```rust
use crate::tests::common::{MockTapdClient, TapdClientTrait};

#[tokio::test]
async fn test_with_mock() {
    let mock_client = MockTapdClient::new(true);
    let app_state = TestAppState::new(Box::new(mock_client));
    
    // Test with mock client
    let result = handler(State(app_state)).await;
    assert!(result.is_ok());
}
```

## Writing Tests

### Test Naming Convention

- Use descriptive names that explain what is being tested
- Follow the pattern: `test_[function_name]_[scenario]`
- Examples:
  - `test_verify_signature_valid`
  - `test_config_load_with_valid_env_vars`
  - `test_api_response_success`

### Test Structure

1. **Arrange**: Set up test data and conditions
2. **Act**: Execute the function being tested
3. **Assert**: Verify the expected outcome

**Example:**
```rust
#[test]
fn test_config_validation() {
    // Arrange
    let mut config = Config::test_config();
    config.server_address = "".to_string();
    
    // Act
    let result = config.validate();
    
    // Assert
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), AppError::ValidationError(_)));
}
```

### Async Tests

For async functions, use `#[tokio::test]`:

```rust
#[tokio::test]
async fn test_async_function() {
    let result = async_function().await;
    assert!(result.is_ok());
}
```

### Error Testing

Test both success and failure scenarios:

```rust
#[test]
fn test_function_success() {
    let result = function_with_result("valid_input");
    assert!(result.is_ok());
}

#[test]
fn test_function_error() {
    let result = function_with_result("invalid_input");
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), AppError::InvalidInput(_)));
}
```

## Test Utilities

### Common Test Utilities

The `tests/common/mod.rs` module provides:

- `MockTapdClient`: Mock implementation of the TapdClient
- `TapdClientTrait`: Trait for abstracting client operations
- `TestAppState`: Test application state
- `create_test_asset()`: Helper to create test assets
- `create_test_transfer()`: Helper to create test transfers

### Environment Setup

```rust
use crate::tests::common::utils::{setup_test_env, cleanup_test_env};

#[test]
fn test_with_env() {
    let (tapd_macaroon, lnd_macaroon) = setup_test_env();
    
    // Your test code here
    
    cleanup_test_env();
}
```

### API Response Assertions

```rust
use crate::tests::common::utils::{assert_api_response_success, assert_api_response_error};

#[test]
fn test_api_response() {
    let response = ApiResponse {
        success: true,
        data: Some("test"),
        error: None,
        message: Some("Success"),
    };
    
    assert_api_response_success(&response);
}
```

## Best Practices

### 1. Test Isolation

- Each test should be independent
- Clean up resources after tests
- Use unique test data for each test

### 2. Comprehensive Coverage

- Test both success and failure paths
- Test edge cases and boundary conditions
- Test error handling and recovery

### 3. Mock External Dependencies

- Mock external services and APIs
- Use trait abstractions for testability
- Avoid real network calls in tests

### 4. Descriptive Test Names

- Use clear, descriptive test names
- Include the scenario being tested
- Make it easy to understand what failed

### 5. Test Data Management

- Use helper functions to create test data
- Keep test data realistic but minimal
- Avoid hardcoded values in assertions

### 6. Error Testing

- Test all error conditions
- Verify error types and messages
- Test error recovery mechanisms

## Troubleshooting

### Common Issues

1. **Test Compilation Errors**
   ```bash
   # Check for missing dependencies
   cargo check
   
   # Check for unused imports
   cargo clippy
   ```

2. **Async Test Issues**
   ```rust
   // Make sure to use tokio::test for async functions
   #[tokio::test]
   async fn test_async() {
       // Your async test
   }
   ```

3. **Mock Issues**
   ```rust
   // Ensure mock implements the correct trait
   impl TapdClientTrait for MockTapdClient {
       // Implement all required methods
   }
   ```

4. **Environment Variable Issues**
   ```rust
   // Use test utilities for environment setup
   let (tapd_macaroon, lnd_macaroon) = setup_test_env();
   // ... test code ...
   cleanup_test_env();
   ```

### Debugging Tests

```bash
# Run specific test with output
cargo test test_name -- --nocapture

# Run tests with backtrace
RUST_BACKTRACE=1 cargo test

# Run tests with specific log level
RUST_LOG=debug cargo test
```

### Performance Testing

For performance-critical code:

```rust
#[test]
fn test_performance() {
    let start = std::time::Instant::now();
    
    // Your performance test
    
    let duration = start.elapsed();
    assert!(duration < std::time::Duration::from_millis(100));
}
```

## Continuous Integration

The test suite is designed to work with CI/CD pipelines:

```yaml
# Example GitHub Actions workflow
- name: Run tests
  run: |
    cd backend
    ./scripts/run_tests.sh

- name: Run clippy
  run: |
    cd backend
    cargo clippy -- -D warnings

- name: Check formatting
  run: |
    cd backend
    cargo fmt -- --check
```

## Coverage Reports

Generate coverage reports:

```bash
# Install grcov
cargo install grcov

# Generate coverage
./scripts/run_tests.sh coverage

# View coverage report
open coverage/index.html
```

## Contributing

When adding new features:

1. Write tests first (TDD approach)
2. Ensure all tests pass
3. Add integration tests for new endpoints
4. Update this documentation if needed
5. Run the full test suite before submitting

## Support

For test-related issues:

1. Check this documentation
2. Review existing test examples
3. Check the troubleshooting section
4. Open an issue with detailed information 