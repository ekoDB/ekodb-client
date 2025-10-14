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
