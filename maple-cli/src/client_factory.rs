use maplestory_openapi::client::cache::DiskIdentityCache;
use maplestory_openapi::client::MapleStoryClient;
use std::path::{Path, PathBuf};
use std::time::Duration;

pub fn default_cache_dir() -> PathBuf {
    if let Ok(local_app_data) = std::env::var("LOCALAPPDATA") {
        PathBuf::from(local_app_data).join("maple-cli").join("cache")
    } else if let Ok(home) = std::env::var("HOME") {
        PathBuf::from(home).join(".cache").join("maple-cli")
    } else {
        std::env::temp_dir().join("maple-cli-cache")
    }
}

pub fn create_client(
    api_key: Option<&str>,
    cache_dir: Option<&Path>,
    no_cache: bool,
) -> Result<MapleStoryClient, Box<dyn std::error::Error + Send + Sync>> {
    let key = match api_key {
        Some(k) if !k.trim().is_empty() => k.to_string(),
        _ => match std::env::var("NEXON_API_KEY") {
            Ok(k) if !k.trim().is_empty() => k,
            _ => {
                return Err(
                    "Missing Nexon API Key. Please provide --api-key or set the NEXON_API_KEY environment variable.\nGet your free key at: https://openapi.nexon.com".into()
                );
            }
        },
    };

    let builder = MapleStoryClient::builder().api_key(key);

    let builder = if !no_cache {
        let dir = cache_dir.map(PathBuf::from).unwrap_or_else(default_cache_dir);
        let disk_cache = DiskIdentityCache::new(dir, Duration::from_secs(86400 * 7));
        builder.identity_cache(std::sync::Arc::new(disk_cache))
    } else {
        builder
    };

    Ok(builder.build()?)
}
