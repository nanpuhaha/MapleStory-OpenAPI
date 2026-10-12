# 14: Character Progression Deep Domain Helpers (HEXA Matrix, V-Matrix, Ability, Hyper Stat)

**What to build:** Deepen character progression models (`CharacterHexaMatrix`, `CharacterHexaMatrixStat`, `CharacterVMatrix`, `CharacterAbility`, `CharacterHyperStat`) in `src/models/character.rs` with domain query and aggregation methods.

**Blocked by:** 04: Equipment Potential Option Parser & Stat Aggregator

**Status:** todo

- [ ] Add `CharacterHexaMatrix` helpers: `core_level(name)`, `cores_by_type(core_type)`, `active_cores()`, and `CharacterHexaMatrixStat` main/additional stat level accessors.
- [ ] Add `CharacterVMatrix` helpers: `total_slot_enhancement_level()`, `core_by_name(name)`, `max_level_cores()`.
- [ ] Add `CharacterAbility` and `CharacterHyperStat` preset-aware helpers (`active_preset_abilities()`, `active_preset_hyper_stats()`, `find_hyper_stat(stat_type)`).
- [ ] Expose progression models in `src/lib.rs` root re-exports and add unit tests in `tests/helpers_tests.rs`.
