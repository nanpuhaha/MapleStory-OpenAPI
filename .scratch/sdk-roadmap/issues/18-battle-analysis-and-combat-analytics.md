# 18: Character Combat Analytics & Dojo Spec Aggregator

**What to build:** Introduce `CharacterCombatAnalytics` domain aggregator and estimation methods in `maplestory-openapi` (`src/models/character.rs` / `analytics.rs` and `src/client/character.rs`), computing effective boss damage multipliers against typical boss defense rates (300%/380%), Starforce/Symbol stat contribution breakdown, and Mu Lung Dojo floor scaling insights.

**Blocked by:** 17: Packaging & Release Readiness

**Status:** todo

- [ ] Design and implement `CharacterCombatAnalytics` model capturing effective damage multiplier estimates, IED penetration ratios, and stat force breakdowns.
- [ ] Add client methods `client.character().get_combat_analytics(ocid, date)` and `client.get_character_combat_analytics_by_name(name)`.
- [ ] Re-export types from `maplestory-openapi` root and add comprehensive unit tests in `tests/helpers_tests.rs`.
- [ ] Update documentation, examples, and verify clean compilation with `cargo test --all` and zero clippy warnings.
