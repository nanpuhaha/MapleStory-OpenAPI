// src/client/cache.rs
//! Pluggable identity caching abstraction for character OCID and guild OGuildID.

use std::future::Future;
use std::pin::Pin;

/// A boxed future suitable for dynamic dispatch traits.
pub type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

/// Asynchronous cache for character OCID and guild OGuildID mappings.
///
/// Implement this trait to provide custom shared cache storage (such as Redis or Memcached)
/// across multiple application instances.
pub trait IdentityCache: Send + Sync {
    /// Retrieve a cached OCID by character name.
    fn get_ocid<'a>(&'a self, character_name: &'a str) -> BoxFuture<'a, Option<String>>;

    /// Store a character OCID into the cache.
    fn set_ocid<'a>(&'a self, character_name: &'a str, ocid: &'a str) -> BoxFuture<'a, ()>;

    /// Retrieve a cached OGuildID by guild name and world name.
    fn get_oguild_id<'a>(
        &'a self,
        guild_name: &'a str,
        world_name: &'a str,
    ) -> BoxFuture<'a, Option<String>>;

    /// Store an OGuildID into the cache.
    fn set_oguild_id<'a>(
        &'a self,
        guild_name: &'a str,
        world_name: &'a str,
        oguild_id: &'a str,
    ) -> BoxFuture<'a, ()>;
}

/// A no-op identity cache that never caches entries.
#[derive(Debug, Default, Clone, Copy)]
pub struct NoopIdentityCache;

impl IdentityCache for NoopIdentityCache {
    fn get_ocid<'a>(&'a self, _character_name: &'a str) -> BoxFuture<'a, Option<String>> {
        Box::pin(async { None })
    }

    fn set_ocid<'a>(&'a self, _character_name: &'a str, _ocid: &'a str) -> BoxFuture<'a, ()> {
        Box::pin(async {})
    }

    fn get_oguild_id<'a>(
        &'a self,
        _guild_name: &'a str,
        _world_name: &'a str,
    ) -> BoxFuture<'a, Option<String>> {
        Box::pin(async { None })
    }

    fn set_oguild_id<'a>(
        &'a self,
        _guild_name: &'a str,
        _world_name: &'a str,
        _oguild_id: &'a str,
    ) -> BoxFuture<'a, ()> {
        Box::pin(async {})
    }
}

/// In-memory identity cache powered by [`moka`].
#[cfg(feature = "cache")]
pub struct MokaIdentityCache {
    ocid_cache: moka::future::Cache<String, String>,
    oguild_id_cache: moka::future::Cache<(String, String), String>,
}

#[cfg(feature = "cache")]
impl MokaIdentityCache {
    /// Create a new in-memory cache with specified maximum capacity and time-to-live.
    pub fn new(capacity: u64, ttl: std::time::Duration) -> Self {
        Self {
            ocid_cache: moka::future::Cache::builder()
                .max_capacity(capacity)
                .time_to_live(ttl)
                .build(),
            oguild_id_cache: moka::future::Cache::builder()
                .max_capacity(capacity)
                .time_to_live(ttl)
                .build(),
        }
    }
}

#[cfg(feature = "cache")]
impl IdentityCache for MokaIdentityCache {
    fn get_ocid<'a>(&'a self, character_name: &'a str) -> BoxFuture<'a, Option<String>> {
        Box::pin(async move { self.ocid_cache.get(character_name).await })
    }

    fn set_ocid<'a>(&'a self, character_name: &'a str, ocid: &'a str) -> BoxFuture<'a, ()> {
        let name = character_name.to_string();
        let ocid = ocid.to_string();
        Box::pin(async move {
            self.ocid_cache.insert(name, ocid).await;
        })
    }

    fn get_oguild_id<'a>(
        &'a self,
        guild_name: &'a str,
        world_name: &'a str,
    ) -> BoxFuture<'a, Option<String>> {
        let key = (guild_name.to_string(), world_name.to_string());
        Box::pin(async move { self.oguild_id_cache.get(&key).await })
    }

    fn set_oguild_id<'a>(
        &'a self,
        guild_name: &'a str,
        world_name: &'a str,
        oguild_id: &'a str,
    ) -> BoxFuture<'a, ()> {
        let key = (guild_name.to_string(), world_name.to_string());
        let oguild_id = oguild_id.to_string();
        Box::pin(async move {
            self.oguild_id_cache.insert(key, oguild_id).await;
        })
    }
}

// ─── DiskIdentityCache ────────────────────────────────────────────────────────

#[cfg(not(target_arch = "wasm32"))]
use std::collections::HashMap;
#[cfg(not(target_arch = "wasm32"))]
use std::fs;
#[cfg(not(target_arch = "wasm32"))]
use std::path::{Path, PathBuf};
#[cfg(not(target_arch = "wasm32"))]
use std::sync::atomic::{AtomicU64, Ordering};
#[cfg(not(target_arch = "wasm32"))]
use std::sync::Arc;
#[cfg(not(target_arch = "wasm32"))]
use std::time::{Duration, SystemTime, UNIX_EPOCH};
#[cfg(not(target_arch = "wasm32"))]
use serde::{Deserialize, Serialize};
#[cfg(not(target_arch = "wasm32"))]
use tokio::sync::RwLock;

#[cfg(not(target_arch = "wasm32"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
struct DiskCacheEntry {
    value: String,
    cached_at_millis: u64,
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
struct DiskCacheData {
    #[serde(default)]
    version: u32,
    #[serde(default)]
    ocid: HashMap<String, DiskCacheEntry>,
    #[serde(default)]
    oguild_id: HashMap<String, DiskCacheEntry>,
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Debug)]
struct DiskInnerState {
    data: DiskCacheData,
    last_disk_mtime: Option<SystemTime>,
}

#[cfg(not(target_arch = "wasm32"))]
fn current_time_millis() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0)
}

#[cfg(not(target_arch = "wasm32"))]
fn is_entry_expired(entry: &DiskCacheEntry, ttl: Duration) -> bool {
    let now = current_time_millis();
    now.saturating_sub(entry.cached_at_millis) > ttl.as_millis() as u64
}

#[cfg(not(target_arch = "wasm32"))]
fn format_guild_key(guild_name: &str, world_name: &str) -> String {
    format!("{}:{}", guild_name, world_name)
}

#[cfg(not(target_arch = "wasm32"))]
fn safe_atomic_write(dir: &Path, final_path: &Path, content: &[u8]) -> std::io::Result<()> {
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    let temp_name = format!(
        ".identity_cache_tmp_{}_{}_{}.tmp",
        std::process::id(),
        current_time_millis(),
        COUNTER.fetch_add(1, Ordering::Relaxed)
    );
    let temp_path = dir.join(temp_name);

    fs::write(&temp_path, content)?;

    let mut last_err = None;
    for attempt in 0..5 {
        match fs::rename(&temp_path, final_path) {
            Ok(()) => return Ok(()),
            Err(e) => {
                last_err = Some(e);
                std::thread::sleep(Duration::from_millis(5 * (attempt + 1)));
            }
        }
    }

    let _ = fs::remove_file(&temp_path);
    Err(last_err.unwrap_or_else(|| std::io::Error::other("Failed to rename temporary cache file")))
}

/// Persistent disk-backed identity cache that stores character OCIDs and guild OGuildIDs.
///
/// Saves mappings as human-readable JSON and provides thread-safe, atomic disk persistence with
/// configurable TTL expiration and automatic synchronization across multiple instances or processes.
#[cfg(not(target_arch = "wasm32"))]
#[derive(Debug, Clone)]
pub struct DiskIdentityCache {
    dir: PathBuf,
    cache_file: PathBuf,
    ttl: Duration,
    state: Arc<RwLock<DiskInnerState>>,
}

#[cfg(not(target_arch = "wasm32"))]
impl DiskIdentityCache {
    /// Create a new persistent disk cache in `dir` with the specified TTL.
    ///
    /// If the directory does not exist, it will be created. If an existing `identity_cache.json`
    /// file exists, non-expired entries are loaded immediately into memory.
    pub fn new(dir: impl AsRef<Path>, ttl: Duration) -> Self {
        let dir = dir.as_ref().to_path_buf();
        let _ = fs::create_dir_all(&dir);
        let cache_file = dir.join("identity_cache.json");

        let (data, mtime) = Self::read_disk_file(&cache_file);

        Self {
            dir,
            cache_file,
            ttl,
            state: Arc::new(RwLock::new(DiskInnerState {
                data,
                last_disk_mtime: mtime,
            })),
        }
    }

    /// Returns the directory path where the cache file is stored.
    pub fn dir(&self) -> &Path {
        &self.dir
    }

    /// Returns the configured time-to-live.
    pub fn ttl(&self) -> Duration {
        self.ttl
    }

    /// Clears all cached entries from memory and deletes the on-disk cache file.
    pub async fn clear(&self) -> std::io::Result<()> {
        let mut state = self.state.write().await;
        state.data = DiskCacheData::default();
        state.last_disk_mtime = None;
        if self.cache_file.exists() {
            fs::remove_file(&self.cache_file)?;
        }
        Ok(())
    }

    /// Returns the number of cached (non-expired) entries as `(ocid_count, oguild_count)`.
    pub async fn count(&self) -> (usize, usize) {
        self.sync_from_disk_if_modified().await;
        let state = self.state.read().await;
        let ocid_count = state
            .data
            .ocid
            .values()
            .filter(|e| !is_entry_expired(e, self.ttl))
            .count();
        let oguild_count = state
            .data
            .oguild_id
            .values()
            .filter(|e| !is_entry_expired(e, self.ttl))
            .count();
        (ocid_count, oguild_count)
    }

    fn read_disk_file(path: &Path) -> (DiskCacheData, Option<SystemTime>) {
        if !path.exists() {
            return (DiskCacheData::default(), None);
        }

        let Ok(metadata) = fs::metadata(path) else {
            return (DiskCacheData::default(), None);
        };
        let mtime = metadata.modified().ok();

        let Ok(bytes) = fs::read(path) else {
            return (DiskCacheData::default(), mtime);
        };

        if bytes.is_empty() {
            return (DiskCacheData::default(), mtime);
        }

        match serde_json::from_slice::<DiskCacheData>(&bytes) {
            Ok(data) => (data, mtime),
            Err(e) => {
                tracing::warn!("Failed to parse identity cache file {:?}: {}", path, e);
                (DiskCacheData::default(), mtime)
            }
        }
    }

    async fn sync_from_disk_if_modified(&self) {
        if !self.cache_file.exists() {
            return;
        }

        let current_mtime = fs::metadata(&self.cache_file)
            .ok()
            .and_then(|m| m.modified().ok());

        {
            let state = self.state.read().await;
            if state.last_disk_mtime == current_mtime {
                return;
            }
        }

        let mut state = self.state.write().await;
        if state.last_disk_mtime != current_mtime {
            let (disk_data, mtime) = Self::read_disk_file(&self.cache_file);
            state.data = disk_data;
            state.last_disk_mtime = mtime;
        }
    }

    async fn get_ocid_internal(&self, character_name: &str) -> Option<String> {
        self.sync_from_disk_if_modified().await;
        let state = self.state.read().await;
        let entry = state.data.ocid.get(character_name)?;
        if is_entry_expired(entry, self.ttl) {
            None
        } else {
            Some(entry.value.clone())
        }
    }

    async fn set_ocid_internal(&self, character_name: &str, ocid: &str) {
        let mut state = self.state.write().await;

        let disk_mtime = fs::metadata(&self.cache_file)
            .ok()
            .and_then(|m| m.modified().ok());
        if disk_mtime != state.last_disk_mtime && self.cache_file.exists() {
            let (disk_data, mtime) = Self::read_disk_file(&self.cache_file);
            for (k, v) in disk_data.ocid {
                if !is_entry_expired(&v, self.ttl) {
                    state.data.ocid.entry(k).or_insert(v);
                }
            }
            for (k, v) in disk_data.oguild_id {
                if !is_entry_expired(&v, self.ttl) {
                    state.data.oguild_id.entry(k).or_insert(v);
                }
            }
            state.last_disk_mtime = mtime;
        }

        let now = current_time_millis();
        state.data.version = 1;
        state.data.ocid.insert(
            character_name.to_string(),
            DiskCacheEntry {
                value: ocid.to_string(),
                cached_at_millis: now,
            },
        );

        let ttl = self.ttl;
        state.data.ocid.retain(|_, v| !is_entry_expired(v, ttl));
        state.data.oguild_id.retain(|_, v| !is_entry_expired(v, ttl));

        if let Ok(bytes) = serde_json::to_vec_pretty(&state.data) {
            let _ = fs::create_dir_all(&self.dir);
            if safe_atomic_write(&self.dir, &self.cache_file, &bytes).is_ok() {
                state.last_disk_mtime = fs::metadata(&self.cache_file)
                    .ok()
                    .and_then(|m| m.modified().ok());
            }
        }
    }

    async fn get_oguild_id_internal(&self, key: &str) -> Option<String> {
        self.sync_from_disk_if_modified().await;
        let state = self.state.read().await;
        let entry = state.data.oguild_id.get(key)?;
        if is_entry_expired(entry, self.ttl) {
            None
        } else {
            Some(entry.value.clone())
        }
    }

    async fn set_oguild_id_internal(&self, key: &str, oguild_id: &str) {
        let mut state = self.state.write().await;

        let disk_mtime = fs::metadata(&self.cache_file)
            .ok()
            .and_then(|m| m.modified().ok());
        if disk_mtime != state.last_disk_mtime && self.cache_file.exists() {
            let (disk_data, mtime) = Self::read_disk_file(&self.cache_file);
            for (k, v) in disk_data.ocid {
                if !is_entry_expired(&v, self.ttl) {
                    state.data.ocid.entry(k).or_insert(v);
                }
            }
            for (k, v) in disk_data.oguild_id {
                if !is_entry_expired(&v, self.ttl) {
                    state.data.oguild_id.entry(k).or_insert(v);
                }
            }
            state.last_disk_mtime = mtime;
        }

        let now = current_time_millis();
        state.data.version = 1;
        state.data.oguild_id.insert(
            key.to_string(),
            DiskCacheEntry {
                value: oguild_id.to_string(),
                cached_at_millis: now,
            },
        );

        let ttl = self.ttl;
        state.data.ocid.retain(|_, v| !is_entry_expired(v, ttl));
        state.data.oguild_id.retain(|_, v| !is_entry_expired(v, ttl));

        if let Ok(bytes) = serde_json::to_vec_pretty(&state.data) {
            let _ = fs::create_dir_all(&self.dir);
            if safe_atomic_write(&self.dir, &self.cache_file, &bytes).is_ok() {
                state.last_disk_mtime = fs::metadata(&self.cache_file)
                    .ok()
                    .and_then(|m| m.modified().ok());
            }
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl IdentityCache for DiskIdentityCache {
    fn get_ocid<'a>(&'a self, character_name: &'a str) -> BoxFuture<'a, Option<String>> {
        let name = character_name.to_string();
        Box::pin(async move { self.get_ocid_internal(&name).await })
    }

    fn set_ocid<'a>(&'a self, character_name: &'a str, ocid: &'a str) -> BoxFuture<'a, ()> {
        let name = character_name.to_string();
        let ocid = ocid.to_string();
        Box::pin(async move { self.set_ocid_internal(&name, &ocid).await })
    }

    fn get_oguild_id<'a>(
        &'a self,
        guild_name: &'a str,
        world_name: &'a str,
    ) -> BoxFuture<'a, Option<String>> {
        let key = format_guild_key(guild_name, world_name);
        Box::pin(async move { self.get_oguild_id_internal(&key).await })
    }

    fn set_oguild_id<'a>(
        &'a self,
        guild_name: &'a str,
        world_name: &'a str,
        oguild_id: &'a str,
    ) -> BoxFuture<'a, ()> {
        let key = format_guild_key(guild_name, world_name);
        let oguild_id = oguild_id.to_string();
        Box::pin(async move { self.set_oguild_id_internal(&key, &oguild_id).await })
    }
}
