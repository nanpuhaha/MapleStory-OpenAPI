// src/client/mod.rs
//! HTTP client implementation for MapleStory OpenAPI.

mod character;
mod user;

use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client,
};
use serde::de::DeserializeOwned;

use crate::error::{ApiErrorResponse, MapleStoryError, Result};

// Character and user method impls are added via `impl MapleStoryClient` in submodules

/// The base URL of the MapleStory OpenAPI.
pub const DEFAULT_BASE_URL: &str = "https://open.api.nexon.com";

/// The name of the API key header.
const API_KEY_HEADER: &str = "x-nxopen-api-key";

/// The minimum data availability date.
pub const MIN_DATE: &str = "2023-12-21";

// ─── Builder ─────────────────────────────────────────────────────────────────

/// Builder for [`MapleStoryClient`].
///
/// # Example
/// ```rust,no_run
/// use maplestory_openapi::MapleStoryClient;
///
/// let client = MapleStoryClient::builder()
///     .api_key("YOUR_API_KEY")
///     .build()
///     .unwrap();
/// ```
#[derive(Debug, Default)]
pub struct MapleStoryClientBuilder {
    api_key: Option<String>,
    base_url: Option<String>,
    timeout_secs: Option<u64>,
}

impl MapleStoryClientBuilder {
    /// Set the API key (required).
    pub fn api_key(mut self, key: impl Into<String>) -> Self {
        self.api_key = Some(key.into());
        self
    }

    /// Override the base URL (defaults to `https://open.api.nexon.com`).
    pub fn base_url(mut self, url: impl Into<String>) -> Self {
        self.base_url = Some(url.into());
        self
    }

    /// Set request timeout in seconds (default: 30).
    pub fn timeout_secs(mut self, secs: u64) -> Self {
        self.timeout_secs = Some(secs);
        self
    }

    /// Build the [`MapleStoryClient`].
    ///
    /// # Errors
    /// Returns `MapleStoryError::InvalidParameter` if the API key is missing or
    /// `MapleStoryError::Request` if the underlying HTTP client cannot be built.
    pub fn build(self) -> Result<MapleStoryClient> {
        let api_key = self
            .api_key
            .ok_or_else(|| MapleStoryError::InvalidParameter("API key is required".into()))?;

        if api_key.is_empty() {
            return Err(MapleStoryError::InvalidParameter(
                "API key must not be empty".into(),
            ));
        }

        let timeout = std::time::Duration::from_secs(self.timeout_secs.unwrap_or(30));

        let mut default_headers = HeaderMap::new();
        default_headers.insert(
            API_KEY_HEADER,
            HeaderValue::from_str(&api_key)
                .map_err(|_| MapleStoryError::InvalidParameter("Invalid API key format".into()))?,
        );

        let http_client = Client::builder()
            .default_headers(default_headers)
            .timeout(timeout)
            .build()
            .map_err(MapleStoryError::Request)?;

        let base_url = self
            .base_url
            .unwrap_or_else(|| DEFAULT_BASE_URL.to_string());

        Ok(MapleStoryClient {
            http: http_client,
            base_url,
        })
    }
}

// ─── Client ──────────────────────────────────────────────────────────────────

/// Async HTTP client for the MapleStory OpenAPI.
///
/// Create via [`MapleStoryClient::builder()`] or [`MapleStoryClient::new()`].
///
/// # Thread Safety
/// `MapleStoryClient` is `Clone` and can be shared across threads safely — it
/// wraps a [`reqwest::Client`] which is internally `Arc`-backed.
#[derive(Debug, Clone)]
pub struct MapleStoryClient {
    http: Client,
    base_url: String,
}

impl MapleStoryClient {
    /// Create a builder for configuring the client.
    pub fn builder() -> MapleStoryClientBuilder {
        MapleStoryClientBuilder::default()
    }

    /// Convenience constructor — shorthand for `.builder().api_key(key).build()`.
    ///
    /// # Example
    /// ```rust,no_run
    /// use maplestory_openapi::MapleStoryClient;
    ///
    /// # tokio_test::block_on(async {
    /// let client = MapleStoryClient::new("YOUR_API_KEY").unwrap();
    /// # });
    /// ```
    pub fn new(api_key: impl Into<String>) -> Result<Self> {
        Self::builder().api_key(api_key).build()
    }

    /// Internal helper — performs a GET request and deserializes the response.
    pub(crate) async fn get<T: DeserializeOwned>(
        &self,
        path: &str,
        params: &[(&str, String)],
    ) -> Result<T> {
        let url = format!("{}{}", self.base_url, path);

        tracing::debug!("GET {} {:?}", url, params);

        let response = self
            .http
            .get(&url)
            .query(params)
            .send()
            .await
            .map_err(MapleStoryError::Request)?;

        let status = response.status();

        if status.is_success() {
            response
                .json::<T>()
                .await
                .map_err(|e| MapleStoryError::ParseError(e.to_string()))
        } else {
            // Try to parse the API error body
            let status_code = status.as_u16();
            let body = response
                .text()
                .await
                .unwrap_or_else(|_| "{}".to_string());

            if let Ok(err_resp) = serde_json::from_str::<ApiErrorResponse>(&body) {
                let code = err_resp.error.name.clone();
                let message = err_resp.error.message.clone();
                match status_code {
                    401 => return Err(MapleStoryError::AuthenticationFailed),
                    403 => return Err(MapleStoryError::Forbidden),
                    429 => return Err(MapleStoryError::RateLimited { retry_after: 60 }),
                    500 => return Err(MapleStoryError::InternalServerError),
                    503 => return Err(MapleStoryError::ServiceUnavailable),
                    _ => {
                        return Err(MapleStoryError::ApiError {
                            code,
                            message,
                            status: status_code,
                        })
                    }
                }
            }

            Err(MapleStoryError::ApiError {
                code: status.to_string(),
                message: body,
                status: status_code,
            })
        }
    }

    /// Validate a date string and return it as a `String`.
    /// Returns an error if the format is not `YYYY-MM-DD`.
    pub(crate) fn validate_date(date: &str) -> Result<String> {
        // Simple length + dash position check
        let bytes = date.as_bytes();
        if bytes.len() == 10
            && bytes[4] == b'-'
            && bytes[7] == b'-'
            && bytes[..4].iter().all(|b| b.is_ascii_digit())
            && bytes[5..7].iter().all(|b| b.is_ascii_digit())
            && bytes[8..].iter().all(|b| b.is_ascii_digit())
        {
            Ok(date.to_string())
        } else {
            Err(MapleStoryError::InvalidDate {
                date: date.to_string(),
            })
        }
    }
}
