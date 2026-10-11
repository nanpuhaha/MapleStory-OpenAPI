# 04: Add strongly-typed potential and additional option domain parsers

**What to build:** Equipment item models provide strongly-typed parsing methods for raw potential and additional potential Korean strings (e.g. `"STR : +12%"`, `"올스탯 : +9%"`, `"공격력 : +30"`), parsing them into typed domain structures with stat category, numeric magnitude, and percentage indicator without forcing SDK callers to maintain custom regex parsers.

**Blocked by:** None (can start immediately)

**Status:** done

- [x] Define `StatType` (STR, DEX, INT, LUK, AllStat, AttackPower, MagicPower, BossDamage, IgnoreMonsterArmor, CriticalDamage, ItemDropRate, MesoDropRate, CooldownReduction, etc.).
- [x] Define `ParsedPotentialOption` containing `stat_type: StatType`, `value: f64`, and `is_percentage: bool`.
- [x] Add parsing helpers on `ItemEquipmentDetail` (e.g. `potential_1_parsed() -> Option<ParsedPotentialOption>`) for all 3 main potential lines and all 3 additional potential lines.
- [x] Add an aggregate accessor `total_potential_stat(&self, stat: StatType) -> (f64, f64)` summarizing flat and percentage boosts per item.
- [x] Unit tests cover all common potential string patterns, non-stat lines (e.g. "피격 시 일정 확률로 데미지 무시"), and malformed or unrecognized strings.
