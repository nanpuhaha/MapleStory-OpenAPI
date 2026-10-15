# Ticket 19: Pure Character Dashboard Assembler (`CharacterDashboard::assemble`)

## Background & Motivation
Currently `CharacterClient::get_character_dashboard_with_options` contains 500+ lines coupling concurrent network dispatch (`tokio::join!`) with 20 match arms handling error degradation and fallback synthesis. Testing degradation behavior requires a full HTTP mock server and Tokio runtime.

## Requirements
1. Define a domain input container: `RawDashboardSections` (or `DashboardSectionResults`) containing raw `Result<T, NexonApiError>` for each of the 20 character endpoints.
2. Implement `CharacterDashboard::assemble(raw: RawDashboardSections, options: &DashboardOptions) -> CharacterDashboard`:
   - Pure synchronous domain function (no async, no Tokio, no reqwest).
   - Handles partial failures: if non-essential sections return 400/404, record section name in `degraded_sections` and synthesize fallback.
   - If essential section (`basic`) fails, propagate error.
3. Refactor `CharacterClient::get_character_dashboard_with_options`:
   - Keep existing public method signature identical.
   - Only orchestrates the concurrent network fetching, producing `RawDashboardSections`, then delegates to `CharacterDashboard::assemble`.
4. Tests:
   - Add pure unit tests in `tests/` verifying fallback and degraded section tracking without any mock HTTP server.
