// src/error.rs
//! Error types for the MapleStory OpenAPI client.

use thiserror::Error;

/// Alias for `std::result::Result<T, MapleStoryError>`
pub type Result<T> = std::result::Result<T, MapleStoryError>;

/// All errors that can occur when using the MapleStory OpenAPI client.
#[derive(Debug, Error)]
pub enum MapleStoryError {
    /// HTTP request error from reqwest
    #[error("HTTP request failed: {0}")]
    Request(#[from] reqwest::Error),

    /// API returned an error response
    #[error("API error [{code}]: {message}")]
    ApiError {
        /// Error name/code from the API (e.g. "OPENAPI00004")
        code: String,
        /// Human-readable error message
        message: String,
        /// HTTP status code
        status: u16,
    },

    /// Response body could not be parsed
    #[error("Failed to parse response: {0}")]
    ParseError(String),

    /// Character not found (OCID lookup returned empty)
    #[error("Character not found: {name}")]
    CharacterNotFound { name: String },

    /// Invalid date format (must be YYYY-MM-DD)
    #[error("Invalid date format '{date}': expected YYYY-MM-DD")]
    InvalidDate { date: String },

    /// Invalid parameter
    #[error("Invalid parameter: {0}")]
    InvalidParameter(String),

    /// Rate limit exceeded
    #[error("Rate limit exceeded; retry after {retry_after} seconds")]
    RateLimited { retry_after: u64 },

    /// Authentication failed (invalid API key)
    #[error("Authentication failed: invalid API key")]
    AuthenticationFailed,

    /// Access forbidden
    #[error("Access forbidden")]
    Forbidden,

    /// Server-side internal error
    #[error("Internal server error")]
    InternalServerError,

    /// Service temporarily unavailable
    #[error("Service temporarily unavailable")]
    ServiceUnavailable,

    /// Requested date is out of the allowed range
    #[error("Date {date} is out of range; minimum allowed date is {min_date}")]
    DateOutOfRange { date: String, min_date: String },

    /// URL construction error
    #[error("URL error: {0}")]
    UrlError(#[from] url::ParseError),

    /// Other uncategorized error
    #[error("Unexpected error: {0}")]
    Other(String),
}

/// Deserialized API error response body
#[derive(Debug, serde::Deserialize)]
pub(crate) struct ApiErrorResponse {
    pub error: ApiErrorDetail,
}

/// Detail of an API error
#[derive(Debug, serde::Deserialize)]
pub(crate) struct ApiErrorDetail {
    /// Error code name (e.g. "OPENAPI00004")
    pub name: String,
    /// Error message
    pub message: String,
}