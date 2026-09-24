// src/client/mod.rs
//! HTTP client implementation for MapleStory OpenAPI.

mod character;
mod guild;
mod union;
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
    custom_client: Option<Client>,
    #[cfg(feature = "cache")]
    cache_enabled: bool,
    #[cfg(feature = "cache")]
    cache_ttl: Option<std::time::Duration>,
    #[cfg(feature = "cache")]
    cache_capacity: Option<u64>,
}

impl MapleStoryClientBuilder {
    /// Create a new builder with default settings.
    pub fn new() -> Self {
        Self {
            api_key: None,
            base_url: None,
            timeout_secs: None,
            custom_client: None,
            #[cfg(feature = "cache")]
            cache_enabled: true,
            #[cfg(feature = "cache")]
            cache_ttl: None,
            #[cfg(feature = "cache")]
            cache_capacity: None,
        }
    }

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

    /// Provide a custom configured [`reqwest::Client`].
    ///
    /// If provided, this client is used directly, but the API key header
    /// will still be appended to requests.
    pub fn http_client(mut self, client: Client) -> Self {
        self.custom_client = Some(client);
        self
    }

    /// Enable or disable in-memory OCID caching (requires `cache` feature).
    #[cfg(feature = "cache")]
    pub fn enable_cache(mut self, enabled: bool) -> Self {
        self.cache_enabled = enabled;
        self
    }

    /// Set in-memory OCID cache TTL (default: 24 hours).
    #[cfg(feature = "cache")]
    pub fn cache_ttl(mut self, ttl: std::time::Duration) -> Self {
        self.cache_ttl = Some(ttl);
        self
    }

    /// Set in-memory OCID cache maximum capacity (default: 10,000 entries).
    #[cfg(feature = "cache")]
    pub fn cache_capacity(mut self, capacity: u64) -> Self {
        self.cache_capacity = Some(capacity);
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

        let http_client = if let Some(client) = self.custom_client {
            client
        } else {
            let timeout = std::time::Duration::from_secs(self.timeout_secs.unwrap_or(30));
            let mut default_headers = HeaderMap::new();
            default_headers.insert(
                API_KEY_HEADER,
                HeaderValue::from_str(&api_key).map_err(|_| {
                    MapleStoryError::InvalidParameter("Invalid API key format".into())
                })?,
            );

            Client::builder()
                .default_headers(default_headers)
                .timeout(timeout)
                .build()
                .map_err(MapleStoryError::Request)?
        };

        let base_url = self
            .base_url
            .unwrap_or_else(|| DEFAULT_BASE_URL.to_string());
        let base_url = url::Url::parse(&base_url).map_err(MapleStoryError::Url)?;
        if !matches!(base_url.scheme(), "http" | "https")
            || base_url.host_str().is_none()
            || base_url.query().is_some()
            || base_url.fragment().is_some()
        {
            return Err(MapleStoryError::InvalidParameter(
                "Base URL must be an absolute HTTP(S) URL without a query or fragment".into(),
            ));
        }
        let base_url = base_url.as_str().trim_end_matches('/').to_string();

        #[cfg(feature = "cache")]
        let ocid_cache = if self.cache_enabled {
            let ttl = self
                .cache_ttl
                .unwrap_or_else(|| std::time::Duration::from_secs(24 * 3600));
            let capacity = self.cache_capacity.unwrap_or(10_000);
            Some(
                moka::future::Cache::builder()
                    .max_capacity(capacity)
                    .time_to_live(ttl)
                    .build(),
            )
        } else {
            None
        };

        Ok(MapleStoryClient {
            http: http_client,
            base_url,
            api_key,
            #[cfg(feature = "cache")]
            ocid_cache,
        })
    }
}

// ─── Client ──────────────────────────────────────────────────────────────────

/// Async HTTP client for the MapleStory OpenAPI.
///
/// Create via [`MapleStoryClient::builder()`], [`MapleStoryClient::new()`], or
/// [`MapleStoryClient::from_env()`].
///
/// # Thread Safety
/// `MapleStoryClient` is `Clone` and can be shared across threads safely — it
/// wraps a [`reqwest::Client`] which is internally `Arc`-backed.
#[derive(Debug, Clone)]
pub struct MapleStoryClient {
    http: Client,
    base_url: String,
    api_key: String,
    #[cfg(feature = "cache")]
    pub(crate) ocid_cache: Option<moka::future::Cache<String, String>>,
}

impl MapleStoryClient {
    /// Create a builder for configuring the client.
    pub fn builder() -> MapleStoryClientBuilder {
        MapleStoryClientBuilder::new()
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

    /// Create a client using the API key from the `NEXON_API_KEY` (or `MAPLESTORY_API_KEY`) environment variable.
    ///
    /// # Example
    /// ```rust,no_run
    /// use maplestory_openapi::MapleStoryClient;
    ///
    /// # tokio_test::block_on(async {
    /// let client = MapleStoryClient::from_env().unwrap();
    /// # });
    /// ```
    pub fn from_env() -> Result<Self> {
        let key = std::env::var("NEXON_API_KEY")
            .or_else(|_| std::env::var("MAPLESTORY_API_KEY"))
            .map_err(|_| {
                MapleStoryError::InvalidParameter(
                    "Environment variable 'NEXON_API_KEY' (or 'MAPLESTORY_API_KEY') not set".into(),
                )
            })?;
        Self::new(key)
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
            .header(API_KEY_HEADER, &self.api_key)
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
            let body = response.text().await.unwrap_or_else(|_| "{}".to_string());

            let api_error = serde_json::from_str::<ApiErrorResponse>(&body).ok();
            let code = api_error
                .as_ref()
                .map(|response| response.error.name.clone())
                .unwrap_or_else(|| status.to_string());
            let message = api_error
                .as_ref()
                .map(|response| response.error.message.clone())
                .unwrap_or(body);

            Err(match status_code {
                401 => MapleStoryError::AuthenticationFailed { code, message },
                403 => MapleStoryError::Forbidden { code, message },
                429 => MapleStoryError::RateLimited {
                    retry_after: 60,
                    code,
                    message,
                },
                500 => MapleStoryError::InternalServerError { code, message },
                503 => MapleStoryError::ServiceUnavailable { code, message },
                _ => MapleStoryError::ApiError {
                    code,
                    message,
                    status: status_code,
                },
            })
        }
    }

    /// Validate a real calendar date and return it as `YYYY-MM-DD`.
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
            let year = date[..4].parse::<i32>().ok();
            let month = date[5..7].parse::<u32>().ok();
            let day = date[8..10].parse::<u32>().ok();
            if let (Some(year), Some(month), Some(day)) = (year, month, day) {
                if year > 0 && chrono::NaiveDate::from_ymd_opt(year, month, day).is_some() {
                    return Ok(date.to_string());
                }
            }
        }
        Err(MapleStoryError::InvalidDate {
            date: date.to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validates_calendar_dates() {
        assert_eq!(
            MapleStoryClient::validate_date("2024-02-29").unwrap(),
            "2024-02-29"
        );
        for invalid in [
            "2023-02-29",
            "2024-13-01",
            "2024-04-31",
            "0000-01-01",
            "2024-1-01",
        ] {
            assert!(
                matches!(
                    MapleStoryClient::validate_date(invalid),
                    Err(MapleStoryError::InvalidDate { .. })
                ),
                "accepted invalid date {invalid}"
            );
        }
    }

    #[test]
    fn validates_and_normalizes_base_url() {
        let client = MapleStoryClient::builder()
            .api_key("key")
            .base_url("https://example.com/")
            .build()
            .unwrap();
        assert_eq!(client.base_url, "https://example.com");
        for invalid in [
            "",
            "ftp://example.com",
            "https://example.com?x=1",
            "https://example.com/#frag",
        ] {
            assert!(
                MapleStoryClient::builder()
                    .api_key("key")
                    .base_url(invalid)
                    .build()
                    .is_err(),
                "accepted invalid base URL {invalid}"
            );
        }
    }
}
