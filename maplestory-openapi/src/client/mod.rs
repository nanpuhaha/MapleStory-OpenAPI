// src/client/mod.rs
//! HTTP client implementation for MapleStory OpenAPI.

pub mod cache;
#[cfg(feature = "cache")]
pub use cache::MokaIdentityCache;
pub use cache::{BoxFuture, IdentityCache, NoopIdentityCache};

mod avatar;
mod battle_practice;
mod character;
mod guild;
mod history;
mod notice;
mod ranking;
mod scheduler;
mod union;
mod user;

pub use avatar::AvatarClient;
pub use battle_practice::BattlePracticeClient;
pub use character::CharacterClient;
pub use guild::GuildClient;
pub use history::HistoryClient;
pub use notice::NoticeClient;
pub use ranking::RankingClient;
pub use scheduler::SchedulerClient;
pub use union::UnionClient;
pub use user::UserClient;

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

/// The name of the legacy API key header.
const LEGACY_API_KEY_HEADER: &str = "x-nxopen-legacy-api-key";

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
#[derive(Default)]
pub struct MapleStoryClientBuilder {
    api_key: Option<String>,
    base_url: Option<String>,
    timeout_secs: Option<u64>,
    custom_client: Option<Client>,
    max_concurrency: Option<usize>,
    max_retries: Option<u32>,
    custom_identity_cache: Option<std::sync::Arc<dyn IdentityCache>>,
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
            max_concurrency: None,
            max_retries: None,
            custom_identity_cache: None,
            #[cfg(feature = "cache")]
            cache_enabled: true,
            #[cfg(feature = "cache")]
            cache_ttl: None,
            #[cfg(feature = "cache")]
            cache_capacity: None,
        }
    }

    /// Supply a custom [`IdentityCache`] implementation (e.g. distributed Redis cache).
    pub fn identity_cache(mut self, cache: std::sync::Arc<dyn IdentityCache>) -> Self {
        self.custom_identity_cache = Some(cache);
        self
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

    /// Set the maximum number of concurrent in-flight HTTP requests (default: 10).
    pub fn max_concurrency(mut self, limit: usize) -> Self {
        self.max_concurrency = Some(limit);
        self
    }

    /// Set the maximum number of retry attempts for 429 and transient 5xx errors (default: 3).
    pub fn max_retries(mut self, retries: u32) -> Self {
        self.max_retries = Some(retries);
        self
    }

    /// Enable or disable retries for 429 and transient 5xx errors.
    ///
    /// When disabled (`false`), `max_retries` is set to 0. When enabled (`true`),
    /// defaults to 3 retries unless previously configured.
    pub fn enable_retries(mut self, enabled: bool) -> Self {
        if enabled {
            if self.max_retries.is_none() || self.max_retries == Some(0) {
                self.max_retries = Some(3);
            }
        } else {
            self.max_retries = Some(0);
        }
        self
    }

    /// Alias for [`enable_retries`](Self::enable_retries).
    pub fn retry(self, enabled: bool) -> Self {
        self.enable_retries(enabled)
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
            #[cfg(not(target_arch = "wasm32"))]
            let timeout = std::time::Duration::from_secs(self.timeout_secs.unwrap_or(30));
            let mut default_headers = HeaderMap::new();
            default_headers.insert(
                API_KEY_HEADER,
                HeaderValue::from_str(&api_key).map_err(|_| {
                    MapleStoryError::InvalidParameter("Invalid API key format".into())
                })?,
            );

            #[allow(unused_mut)]
            let mut builder = Client::builder().default_headers(default_headers);

            #[cfg(not(target_arch = "wasm32"))]
            {
                builder = builder.timeout(timeout);
            }

            builder.build().map_err(MapleStoryError::Request)?
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

        let identity_cache: std::sync::Arc<dyn IdentityCache> =
            if let Some(custom) = self.custom_identity_cache {
                custom
            } else {
                #[cfg(feature = "cache")]
                {
                    if self.cache_enabled {
                        let ttl = self
                            .cache_ttl
                            .unwrap_or_else(|| std::time::Duration::from_secs(24 * 3600));
                        let capacity = self.cache_capacity.unwrap_or(10_000);
                        std::sync::Arc::new(MokaIdentityCache::new(capacity, ttl))
                    } else {
                        std::sync::Arc::new(NoopIdentityCache)
                    }
                }
                #[cfg(not(feature = "cache"))]
                {
                    std::sync::Arc::new(NoopIdentityCache)
                }
            };

        let max_concurrency = self.max_concurrency.unwrap_or(10).max(1);
        let max_retries = self.max_retries.unwrap_or(3);
        let concurrency_limiter = std::sync::Arc::new(tokio::sync::Semaphore::new(max_concurrency));

        Ok(MapleStoryClient {
            http: http_client,
            base_url,
            api_key,
            max_concurrency,
            max_retries,
            concurrency_limiter,
            identity_cache,
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
#[derive(Clone)]
pub struct MapleStoryClient {
    http: Client,
    base_url: String,
    api_key: String,
    max_concurrency: usize,
    max_retries: u32,
    concurrency_limiter: std::sync::Arc<tokio::sync::Semaphore>,
    pub(crate) identity_cache: std::sync::Arc<dyn IdentityCache>,
}

impl MapleStoryClient {
    /// Returns the active identity cache instance.
    pub fn identity_cache(&self) -> &std::sync::Arc<dyn IdentityCache> {
        &self.identity_cache
    }
    /// Returns the maximum number of concurrent in-flight HTTP requests allowed.
    pub fn max_concurrency(&self) -> usize {
        self.max_concurrency
    }

    /// Returns the maximum number of retry attempts on rate limit or transient server errors.
    pub fn max_retries(&self) -> u32 {
        self.max_retries
    }

    /// Returns the current number of available concurrency permits.
    pub fn available_concurrency_permits(&self) -> usize {
        self.concurrency_limiter.available_permits()
    }

    /// Access character domain methods via [`CharacterClient`].
    pub fn character(&self) -> CharacterClient<'_> {
        CharacterClient::new(self)
    }

    /// Access avatar look and animation domain methods via [`AvatarClient`].
    pub fn avatar(&self) -> AvatarClient<'_> {
        AvatarClient::new(self)
    }

    /// Access guild domain methods via [`GuildClient`].
    pub fn guild(&self) -> GuildClient<'_> {
        GuildClient::new(self)
    }

    /// Access union domain methods via [`UnionClient`].
    pub fn union(&self) -> UnionClient<'_> {
        UnionClient::new(self)
    }

    /// Access ranking domain methods via [`RankingClient`].
    pub fn ranking(&self) -> RankingClient<'_> {
        RankingClient::new(self)
    }

    /// Access history domain methods via [`HistoryClient`].
    pub fn history(&self) -> HistoryClient<'_> {
        HistoryClient::new(self)
    }

    /// Access notice domain methods via [`NoticeClient`].
    pub fn notice(&self) -> NoticeClient<'_> {
        NoticeClient::new(self)
    }

    /// Access user and account domain methods via [`UserClient`].
    pub fn user(&self) -> UserClient<'_> {
        UserClient::new(self)
    }

    /// Access battle practice domain methods via [`BattlePracticeClient`].
    pub fn battle_practice(&self) -> BattlePracticeClient<'_> {
        BattlePracticeClient::new(self)
    }

    /// Access scheduler domain methods via [`SchedulerClient`].
    pub fn scheduler(&self) -> SchedulerClient<'_> {
        SchedulerClient::new(self)
    }

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

    /// Internal helper — performs a GET request with custom header and deserializes the response.
    pub(crate) async fn get_with_header<T: DeserializeOwned>(
        &self,
        path: &str,
        params: &[(&str, String)],
        header_name: &str,
        header_val: &str,
    ) -> Result<T> {
        let url = format!("{}{}", self.base_url, path);
        let mut attempts = 0;

        loop {
            attempts += 1;

            let _permit =
                self.concurrency_limiter.acquire().await.map_err(|e| {
                    MapleStoryError::Other(format!("Concurrency limiter error: {}", e))
                })?;

            tracing::debug!("GET {} {:?} (attempt {})", url, params, attempts);

            let response_result = self
                .http
                .get(&url)
                .header(header_name, header_val)
                .query(params)
                .send()
                .await;

            // Release permit before any retry wait
            drop(_permit);

            let response = match response_result {
                Ok(resp) => resp,
                Err(err)
                    if attempts <= self.max_retries
                        && (err.is_timeout() || {
                            #[cfg(not(target_arch = "wasm32"))]
                            {
                                err.is_connect()
                            }
                            #[cfg(target_arch = "wasm32")]
                            {
                                false
                            }
                        }) =>
                {
                    let delay = calculate_backoff_delay(attempts, None);
                    tokio::time::sleep(delay).await;
                    continue;
                }
                Err(err) => return Err(MapleStoryError::Request(err)),
            };

            let status = response.status();

            if status.is_success() {
                return response
                    .json::<T>()
                    .await
                    .map_err(|e| MapleStoryError::ParseError(e.to_string()));
            }

            let status_code = status.as_u16();

            // Retry on 429 (rate-limited) or 503/500 if attempts remain
            if attempts <= self.max_retries && matches!(status_code, 429 | 500 | 503) {
                let retry_after_secs = response
                    .headers()
                    .get("retry-after")
                    .and_then(|h| h.to_str().ok())
                    .and_then(|s| s.parse::<u64>().ok());

                let delay = calculate_backoff_delay(attempts, retry_after_secs);
                tokio::time::sleep(delay).await;
                continue;
            }

            // Parse final API error body
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

            return Err(match status_code {
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
            });
        }
    }

    /// Internal helper — performs a GET request with standard API key header and deserializes the response.
    pub(crate) async fn get<T: DeserializeOwned>(
        &self,
        path: &str,
        params: &[(&str, String)],
    ) -> Result<T> {
        self.get_with_header(path, params, API_KEY_HEADER, &self.api_key)
            .await
    }

    /// Validate a calendar date and return it as `YYYY-MM-DD`.
    ///
    /// Validates:
    /// - Format matches `YYYY-MM-DD` and is a valid calendar date.
    /// - Date >= [`MapleDate::MIN_DATE`] (`2023-12-21`).
    /// - Date <= latest available date in KST (rejects future dates).
    ///
    /// # Errors
    /// Returns [`MapleStoryError::InvalidDate`] if the date format or calendar date is invalid.
    /// Returns [`MapleStoryError::DateOutOfRange`] if the date is earlier than `2023-12-21`
    /// or later than the latest available date in KST.
    pub fn validate_date(date: &str) -> Result<String> {
        let bytes = date.as_bytes();
        if bytes.len() != 10
            || bytes[4] != b'-'
            || bytes[7] != b'-'
            || !bytes[..4].iter().all(|b| b.is_ascii_digit())
            || !bytes[5..7].iter().all(|b| b.is_ascii_digit())
            || !bytes[8..].iter().all(|b| b.is_ascii_digit())
        {
            return Err(MapleStoryError::InvalidDate {
                date: date.to_string(),
            });
        }

        let year = date[..4]
            .parse::<i32>()
            .map_err(|_| MapleStoryError::InvalidDate {
                date: date.to_string(),
            })?;
        let month = date[5..7]
            .parse::<u32>()
            .map_err(|_| MapleStoryError::InvalidDate {
                date: date.to_string(),
            })?;
        let day = date[8..10]
            .parse::<u32>()
            .map_err(|_| MapleStoryError::InvalidDate {
                date: date.to_string(),
            })?;

        if year <= 0 {
            return Err(MapleStoryError::InvalidDate {
                date: date.to_string(),
            });
        }

        let parsed = match chrono::NaiveDate::from_ymd_opt(year, month, day) {
            Some(d) => d,
            None => {
                return Err(MapleStoryError::InvalidDate {
                    date: date.to_string(),
                });
            }
        };

        let min_date = chrono::NaiveDate::from_ymd_opt(2023, 12, 21).unwrap();
        if parsed < min_date {
            return Err(MapleStoryError::DateOutOfRange {
                date: date.to_string(),
                min_date: crate::models::common::MapleDate::MIN_DATE.to_string(),
            });
        }

        let latest_available = crate::models::common::MapleDate::latest_available()
            .to_naive_date()
            .unwrap();
        if parsed > latest_available {
            return Err(MapleStoryError::DateOutOfRange {
                date: date.to_string(),
                min_date: crate::models::common::MapleDate::MIN_DATE.to_string(),
            });
        }

        Ok(date.to_string())
    }
}

/// Calculate randomized exponential backoff delay with full jitter.
fn calculate_backoff_delay(attempts: u32, retry_after_secs: Option<u64>) -> std::time::Duration {
    if let Some(secs) = retry_after_secs {
        return std::time::Duration::from_millis(secs.saturating_mul(1000).max(50));
    }

    let base_ms = (100u64).saturating_mul(1 << (attempts.saturating_sub(1)).min(6));
    let entropy = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.subsec_nanos() as u64)
        .unwrap_or(0);
    let max_jitter = (base_ms / 2).max(50);
    let jitter = entropy % max_jitter;

    std::time::Duration::from_millis(base_ms + jitter)
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
