# 08: Auto-paginating Streams for History Endpoints

**What to build:** Add stream-based pagination helpers (using `futures_core::Stream` or `tokio_stream`) on `HistoryClient` that automatically follow `next_cursor` until exhaustively completed or a limit is reached, eliminating manual while-loop cursor management for callers.

**Blocked by:** None

**Status:** todo

- [ ] Add `get_starforce_history_stream(&self, count: u32, date: impl Into<DateParam>) -> impl Stream<Item = Result<StarforceHistoryEntry>>` on `HistoryClient`.
- [ ] Add equivalent auto-paginating stream methods for Potential, Cube, and Soul Potential history (`get_potential_history_stream`, `get_cube_history_stream`, `get_soul_potential_history_stream`).
- [ ] Support graceful early termination and bounded page fetching with cancellation.
- [ ] Add unit/mock tests verifying pagination termination when `next_cursor` is null or exhausted.
