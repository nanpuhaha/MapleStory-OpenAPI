# 07: Introduce hierarchical domain namespaces on MapleStoryClient

**What to build:** Group the 50+ flat methods across `MapleStoryClient` into clean domain namespaces (`client.character()`, `client.guild()`, `client.union()`, `client.ranking()`, `client.history()`, `client.notice()`, `client.user()`, `client.battle_practice()`, `client.scheduler()`), while preserving key 1-call convenience methods on the root client, streamlining IDE autocomplete discovery without breaking existing call sites.

**Blocked by:** 01: Add client-side in-flight request concurrency limiter, 03: Support flexible date inputs (Into<MapleDate>) and enforce MIN_DATE / KST boundaries

**Status:** done

- [x] Create domain sub-clients borrowing or holding an `Arc` reference to the core HTTP client: `CharacterClient`, `GuildClient`, `UnionClient`, `RankingClient`, `HistoryClient`, `NoticeClient`, `UserClient`, `BattlePracticeClient`, `SchedulerClient`.
- [x] Expose accessor methods on `MapleStoryClient`: `client.character()`, `client.guild()`, etc., which expose domain-specific methods without pollution from other tags.
- [x] Retain top-level 1-call shortcuts (`client.get_character_by_name(...)`, `client.get_character_dashboard_by_name(...)`) as facades delegating to the sub-clients.
- [x] Ensure all sub-client endpoints take advantage of concurrency limiting (Ticket 01) and flexible `Into<MapleDate>` (Ticket 03).
- [x] End-to-end integration and doc tests demonstrate calling APIs through both the new namespaced interface and top-level convenience facades.
