# 06: Abstract IdentityCache trait and add Guild OGuildID caching

**What to build:** Decouple in-memory caching behind an asynchronous `IdentityCache` trait to enable external shared backends (e.g. Redis in multi-pod Discord bots), and integrate guild identifier caching (`oguild_id`) so repeat guild queries incur 0ms latency rather than making two live HTTP requests per lookup.

**Blocked by:** None (can start immediately)

**Status:** done

- [x] Define an `IdentityCache` async trait supporting character OCID (`character_name -> ocid`) and guild OGuildID (`(guild_name, world_name) -> oguild_id`) lookups and insertions.
- [x] Provide default in-memory `MokaIdentityCache` implementation (under the `cache` feature) and a no-op passthrough when caching is disabled.
- [x] Allow callers to supply custom cache implementations via `MapleStoryClientBuilder::identity_cache(Arc<dyn IdentityCache>)`.
- [x] Update `get_guild_by_name` and `get_oguild_id` to consult and populate guild cache entries.
- [x] Integration tests verify cache hits for both characters and guilds return in 0ms without hitting mock or live endpoints.
