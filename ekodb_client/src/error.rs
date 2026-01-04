//! Error types for the ekoDB client

use thiserror::Error;

/// Result type alias for ekoDB client operations
pub type Result<T> = std::result::Result<T, Error>;

/// Errors that can occur when using the ekoDB client
#[derive(Error, Debug)]
pub enum Error {
    /// HTTP request failed
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),

    /// API returned an error response
    #[error("API error ({code}): {message}")]
    Api {
        /// HTTP status code
        code: u16,
        /// Error message from the server
        message: String,
    },

    /// Authentication failed
    #[error("Authentication failed: {0}")]
    Auth(String),

    /// Token expired - can be retried with token refresh
    #[error("Token expired, please refresh")]
    TokenExpired,

    /// Invalid URL
    #[error("Invalid URL: {0}")]
    InvalidUrl(#[from] url::ParseError),

    /// Serialization error
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    /// WebSocket error
    #[error("WebSocket error: {0}")]
    WebSocket(String),

    /// Connection error
    #[error("Connection error: {0}")]
    Connection(String),

    /// Timeout error
    #[error("Operation timed out")]
    Timeout,

    /// Rate limit exceeded
    #[error("Rate limit exceeded. Retry after {retry_after_secs} seconds")]
    RateLimit {
        /// Seconds to wait before retrying
        retry_after_secs: u64,
    },

    /// Service unavailable
    #[error("Service unavailable: {0}")]
    ServiceUnavailable(String),

    /// Invalid configuration
    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),

    /// Record not found
    #[error("Record not found")]
    NotFound,

    /// Validation error
    #[error("Validation error: {0}")]
    Validation(String),

    /// Authentication error
    #[error("Authentication error: {0}")]
    Authentication(String),
}

impl Error {
    /// Create an API error from status code and message
    pub fn api(code: u16, message: impl Into<String>) -> Self {
        Error::Api {
            code,
            message: message.into(),
        }
    }

    /// Check if the error is retryable
    pub fn is_retryable(&self) -> bool {
        matches!(
            self,
            Error::RateLimit { .. }
                | Error::ServiceUnavailable(_)
                | Error::Timeout
                | Error::Connection(_)
        )
    }

    /// Get retry delay in seconds if applicable
    pub fn retry_delay_secs(&self) -> Option<u64> {
        match self {
            Error::RateLimit { retry_after_secs } => Some(*retry_after_secs),
            Error::ServiceUnavailable(_) => Some(10),
            Error::Timeout => Some(5),
            Error::Connection(_) => Some(3),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ========================================================================
    // Error Construction Tests
    // ========================================================================

    #[test]
    fn test_error_api_construction() {
        let err = Error::api(404, "Not found");
        match err {
            Error::Api { code, message } => {
                assert_eq!(code, 404);
                assert_eq!(message, "Not found");
            }
            _ => panic!("Expected Api error"),
        }
    }

    #[test]
    fn test_error_api_with_string_message() {
        let err = Error::api(500, String::from("Internal server error"));
        match err {
            Error::Api { code, message } => {
                assert_eq!(code, 500);
                assert_eq!(message, "Internal server error");
            }
            _ => panic!("Expected Api error"),
        }
    }

    // ========================================================================
    // Error Display Tests
    // ========================================================================

    #[test]
    fn test_error_display_api() {
        let err = Error::api(404, "Resource not found");
        assert_eq!(format!("{}", err), "API error (404): Resource not found");
    }

    #[test]
    fn test_error_display_auth() {
        let err = Error::Auth("Invalid credentials".to_string());
        assert_eq!(
            format!("{}", err),
            "Authentication failed: Invalid credentials"
        );
    }

    #[test]
    fn test_error_display_token_expired() {
        let err = Error::TokenExpired;
        assert_eq!(format!("{}", err), "Token expired, please refresh");
    }

    #[test]
    fn test_error_display_timeout() {
        let err = Error::Timeout;
        assert_eq!(format!("{}", err), "Operation timed out");
    }

    #[test]
    fn test_error_display_rate_limit() {
        let err = Error::RateLimit {
            retry_after_secs: 30,
        };
        assert_eq!(
            format!("{}", err),
            "Rate limit exceeded. Retry after 30 seconds"
        );
    }

    #[test]
    fn test_error_display_service_unavailable() {
        let err = Error::ServiceUnavailable("Maintenance".to_string());
        assert_eq!(format!("{}", err), "Service unavailable: Maintenance");
    }

    #[test]
    fn test_error_display_not_found() {
        let err = Error::NotFound;
        assert_eq!(format!("{}", err), "Record not found");
    }

    #[test]
    fn test_error_display_validation() {
        let err = Error::Validation("Invalid email format".to_string());
        assert_eq!(format!("{}", err), "Validation error: Invalid email format");
    }

    #[test]
    fn test_error_display_connection() {
        let err = Error::Connection("Connection refused".to_string());
        assert_eq!(format!("{}", err), "Connection error: Connection refused");
    }

    #[test]
    fn test_error_display_websocket() {
        let err = Error::WebSocket("Connection closed".to_string());
        assert_eq!(format!("{}", err), "WebSocket error: Connection closed");
    }

    #[test]
    fn test_error_display_invalid_config() {
        let err = Error::InvalidConfig("Missing API key".to_string());
        assert_eq!(format!("{}", err), "Invalid configuration: Missing API key");
    }

    #[test]
    fn test_error_display_authentication() {
        let err = Error::Authentication("Token expired".to_string());
        assert_eq!(format!("{}", err), "Authentication error: Token expired");
    }

    // ========================================================================
    // Retryable Error Tests
    // ========================================================================

    #[test]
    fn test_is_retryable_rate_limit() {
        let err = Error::RateLimit {
            retry_after_secs: 30,
        };
        assert!(err.is_retryable());
    }

    #[test]
    fn test_is_retryable_service_unavailable() {
        let err = Error::ServiceUnavailable("Maintenance".to_string());
        assert!(err.is_retryable());
    }

    #[test]
    fn test_is_retryable_timeout() {
        let err = Error::Timeout;
        assert!(err.is_retryable());
    }

    #[test]
    fn test_is_retryable_connection() {
        let err = Error::Connection("Connection reset".to_string());
        assert!(err.is_retryable());
    }

    #[test]
    fn test_is_not_retryable_api_error() {
        let err = Error::api(404, "Not found");
        assert!(!err.is_retryable());
    }

    #[test]
    fn test_is_not_retryable_auth() {
        let err = Error::Auth("Invalid credentials".to_string());
        assert!(!err.is_retryable());
    }

    #[test]
    fn test_is_not_retryable_token_expired() {
        let err = Error::TokenExpired;
        assert!(!err.is_retryable());
    }

    #[test]
    fn test_is_not_retryable_validation() {
        let err = Error::Validation("Bad input".to_string());
        assert!(!err.is_retryable());
    }

    #[test]
    fn test_is_not_retryable_not_found() {
        let err = Error::NotFound;
        assert!(!err.is_retryable());
    }

    #[test]
    fn test_is_not_retryable_invalid_config() {
        let err = Error::InvalidConfig("Missing field".to_string());
        assert!(!err.is_retryable());
    }

    #[test]
    fn test_is_not_retryable_websocket() {
        let err = Error::WebSocket("Connection closed".to_string());
        assert!(!err.is_retryable());
    }

    #[test]
    fn test_is_not_retryable_authentication() {
        let err = Error::Authentication("Invalid token".to_string());
        assert!(!err.is_retryable());
    }

    // ========================================================================
    // Retry Delay Tests
    // ========================================================================

    #[test]
    fn test_retry_delay_rate_limit() {
        let err = Error::RateLimit {
            retry_after_secs: 60,
        };
        assert_eq!(err.retry_delay_secs(), Some(60));
    }

    #[test]
    fn test_retry_delay_rate_limit_custom() {
        let err = Error::RateLimit {
            retry_after_secs: 120,
        };
        assert_eq!(err.retry_delay_secs(), Some(120));
    }

    #[test]
    fn test_retry_delay_service_unavailable() {
        let err = Error::ServiceUnavailable("Maintenance".to_string());
        assert_eq!(err.retry_delay_secs(), Some(10));
    }

    #[test]
    fn test_retry_delay_timeout() {
        let err = Error::Timeout;
        assert_eq!(err.retry_delay_secs(), Some(5));
    }

    #[test]
    fn test_retry_delay_connection() {
        let err = Error::Connection("Failed".to_string());
        assert_eq!(err.retry_delay_secs(), Some(3));
    }

    #[test]
    fn test_retry_delay_none_for_api_error() {
        let err = Error::api(404, "Not found");
        assert_eq!(err.retry_delay_secs(), None);
    }

    #[test]
    fn test_retry_delay_none_for_auth() {
        let err = Error::Auth("Failed".to_string());
        assert_eq!(err.retry_delay_secs(), None);
    }

    #[test]
    fn test_retry_delay_none_for_not_found() {
        let err = Error::NotFound;
        assert_eq!(err.retry_delay_secs(), None);
    }

    #[test]
    fn test_retry_delay_none_for_validation() {
        let err = Error::Validation("Invalid".to_string());
        assert_eq!(err.retry_delay_secs(), None);
    }

    #[test]
    fn test_retry_delay_none_for_token_expired() {
        let err = Error::TokenExpired;
        assert_eq!(err.retry_delay_secs(), None);
    }

    // ========================================================================
    // Debug Trait Tests
    // ========================================================================

    #[test]
    fn test_error_debug_format() {
        let err = Error::api(500, "Server error");
        let debug_str = format!("{:?}", err);
        assert!(debug_str.contains("Api"));
        assert!(debug_str.contains("500"));
        assert!(debug_str.contains("Server error"));
    }

    // ========================================================================
    // Edge Cases
    // ========================================================================

    #[test]
    fn test_rate_limit_zero_seconds() {
        let err = Error::RateLimit {
            retry_after_secs: 0,
        };
        assert!(err.is_retryable());
        assert_eq!(err.retry_delay_secs(), Some(0));
    }

    #[test]
    fn test_empty_message_errors() {
        let err = Error::Auth(String::new());
        assert_eq!(format!("{}", err), "Authentication failed: ");

        let err = Error::Connection(String::new());
        assert_eq!(format!("{}", err), "Connection error: ");
    }

    #[test]
    fn test_api_error_various_codes() {
        for code in [400, 401, 403, 404, 500, 502, 503] {
            let err = Error::api(code, "test");
            match err {
                Error::Api { code: c, .. } => assert_eq!(c, code),
                _ => panic!("Expected Api error"),
            }
        }
    }
}
