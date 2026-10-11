# 02: Implement automatic exponential backoff with jitter on 429 and 5xx

**What to build:** The `MapleStoryClient` automatically intercepts HTTP 429 (Rate Limited), 503 (Service Unavailable), and transient connection dropouts, performing jittered exponential backoff retries up to a configured maximum attempt count before surfacing errors to callers.

**Blocked by:** 01: Add client-side in-flight request concurrency limiter

**Status:** done

- [x] `MapleStoryClientBuilder` exposes `.max_retries(u32)` (defaulting to e.g. 3) and an optional retry toggle.
- [x] When an API response returns HTTP 429, the client parses the `Retry-After` header (falling back to exponential backoff) and pauses execution before retrying.
- [x] Exponential backoff applies randomized jitter to prevent thundering herd problems during server recoveries.
- [x] Non-retryable errors (400 Bad Request, 401 Unauthorized, 403 Forbidden, 404 Not Found) fail immediately without wasteful retries.
- [x] An automated test simulates transient 429 and 503 responses and verifies that the client recovers and succeeds transparently.
