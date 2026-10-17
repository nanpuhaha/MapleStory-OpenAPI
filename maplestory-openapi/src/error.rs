// src/error.rs
//! Error types for the MapleStory OpenAPI client.

use std::time::Duration;
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
    #[error(
        "Rate limit exceeded; retry after {retry_after} seconds: {message} (API error {code})"
    )]
    RateLimited {
        retry_after: u64,
        code: String,
        message: String,
    },

    /// Authentication failed (invalid API key)
    #[error("Authentication failed: {message} (API error {code})")]
    AuthenticationFailed { code: String, message: String },

    /// Access forbidden
    #[error("Access forbidden: {message} (API error {code})")]
    Forbidden { code: String, message: String },

    /// Server-side internal error
    #[error("Internal server error: {message} (API error {code})")]
    InternalServerError { code: String, message: String },

    /// Service temporarily unavailable
    #[error("Service temporarily unavailable: {message} (API error {code})")]
    ServiceUnavailable { code: String, message: String },

    /// Requested date is out of the allowed range
    #[error("Date {date} is out of range; allowed range is {min_date} ~ {max_date}")]
    DateOutOfRange {
        date: String,
        min_date: String,
        max_date: String,
    },

    /// URL construction error
    #[error("URL error: {0}")]
    Url(#[from] url::ParseError),

    /// Other uncategorized error
    #[error("Unexpected error: {0}")]
    Other(String),
}

impl MapleStoryError {
    /// Returns `true` if this error indicates the requested character was not found.
    pub fn is_not_found(&self) -> bool {
        matches!(self, MapleStoryError::CharacterNotFound { .. })
    }

    /// Returns `true` if the request was rate-limited (HTTP 429).
    pub fn is_rate_limited(&self) -> bool {
        matches!(self, MapleStoryError::RateLimited { .. })
    }

    /// Returns `true` if the error was caused by invalid or missing authentication (HTTP 401).
    pub fn is_auth_error(&self) -> bool {
        matches!(self, MapleStoryError::AuthenticationFailed { .. })
    }

    /// Returns `true` if access was forbidden (HTTP 403).
    pub fn is_forbidden(&self) -> bool {
        matches!(self, MapleStoryError::Forbidden { .. })
    }

    /// Returns `true` if the server encountered an internal error or is unavailable (HTTP 5xx).
    pub fn is_server_error(&self) -> bool {
        matches!(
            self,
            MapleStoryError::InternalServerError { .. }
                | MapleStoryError::ServiceUnavailable { .. }
        )
    }

    /// Returns the recommended duration to wait before retrying, if rate limited.
    pub fn retry_after(&self) -> Option<Duration> {
        match self {
            MapleStoryError::RateLimited { retry_after, .. } => {
                Some(Duration::from_secs(*retry_after))
            }
            _ => None,
        }
    }

    /// Returns the HTTP status code if available.
    pub fn status_code(&self) -> Option<u16> {
        match self {
            MapleStoryError::ApiError { status, .. } => Some(*status),
            MapleStoryError::AuthenticationFailed { .. } => Some(401),
            MapleStoryError::Forbidden { .. } => Some(403),
            MapleStoryError::RateLimited { .. } => Some(429),
            MapleStoryError::InternalServerError { .. } => Some(500),
            MapleStoryError::ServiceUnavailable { .. } => Some(503),
            _ => None,
        }
    }

    /// Returns the API error code string (e.g. "OPENAPI00004") if available.
    pub fn api_code(&self) -> Option<&str> {
        match self {
            MapleStoryError::ApiError { code, .. }
            | MapleStoryError::RateLimited { code, .. }
            | MapleStoryError::AuthenticationFailed { code, .. }
            | MapleStoryError::Forbidden { code, .. }
            | MapleStoryError::InternalServerError { code, .. }
            | MapleStoryError::ServiceUnavailable { code, .. } => Some(code),
            _ => None,
        }
    }

    /// Returns the API error message if available.
    pub fn api_message(&self) -> Option<&str> {
        match self {
            MapleStoryError::ApiError { message, .. }
            | MapleStoryError::RateLimited { message, .. }
            | MapleStoryError::AuthenticationFailed { message, .. }
            | MapleStoryError::Forbidden { message, .. }
            | MapleStoryError::InternalServerError { message, .. }
            | MapleStoryError::ServiceUnavailable { message, .. } => Some(message),
            _ => None,
        }
    }
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
