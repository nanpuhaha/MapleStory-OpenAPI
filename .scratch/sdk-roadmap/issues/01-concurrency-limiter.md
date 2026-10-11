# 01: Add client-side in-flight request concurrency limiter

**What to build:** The `MapleStoryClient` throttles simultaneous in-flight HTTP requests using an internal concurrency limiter (`max_concurrency`), ensuring that mass fan-outs (such as the 20-endpoint character dashboard) and batch character iterations never exceed safe connection limits or burst-trigger NEXON API rate limits.

**Blocked by:** None (can start immediately)

**Status:** done

- [x] `MapleStoryClientBuilder` supports `.max_concurrency(usize)` with a sensible default (e.g. 10 or 15).
- [x] All outgoing HTTP requests in `get_with_header` acquire a concurrency permit before sending and release it on completion.
- [x] Concurrent character lookups and dashboard fan-outs respect the concurrency boundary under load.
- [x] An automated integration test proves that dispatching 30 concurrent requests respects the configured in-flight concurrency threshold.
