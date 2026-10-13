# 13: Union Raider Board & Union Champion Deep Domain Helpers

**What to build:** Enrich `UserUnionRaider` and `UserUnionChampion` in `src/models/union.rs` with preset-aware domain helpers, parsed stat aggregation (`HashMap<StatType, f64>`), Raider member job/level lookups, and Champion badge/grade summaries.

**Blocked by:** 04: Equipment Potential Option Parser & Stat Aggregator, 05: Union Artifact & Set Effect Domain Helpers

**Status:** done

- [x] Add preset selection helper `active_preset(&self, preset_no: Option<u8>)` and occupied/inner stat aggregation on `UserUnionRaider`.
- [x] Add `total_occupied_stats(&self) -> HashMap<String, f64>` and `total_raider_stats(&self) -> HashMap<String, f64>` on `UserUnionRaider`.
- [x] Add `UserUnionChampion` helpers (`champion_count()`, `find_champion_by_slot(slot)`, `total_badge_stats()`) in `src/models/union.rs`.
- [x] Add unit tests in `tests/helpers_tests.rs` covering Raider presets, occupied/raider stat aggregation, and Union Champion helpers.
