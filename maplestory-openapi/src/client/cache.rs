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
