# 11: Real-time Live API Integration & Automated E2E Verification Workflow

**What to build:** Implement a robust integration test suite utilizing live or recording-based sandbox responses against official NEXON Open API endpoints with real credentials (when `NEXON_API_KEY` is present), complemented by mock-recorded golden fixtures for offline CI.

**Blocked by:** None

**Status:** done

- [x] Enrich `tests/live_api_tests.rs` with comprehensive test coverage across character dashboard, union artifact, set effects, guild, ranking, and history endpoints when `NEXON_API_KEY` is set.
- [x] Add VCR / cassette recording or offline fixture replay fallback for GitHub Actions CI runs without live secrets.
- [x] Ensure all 2026 NEXON OpenAPI spec schema updates match deserialization models without missing fields or unexpected deserialization errors.
