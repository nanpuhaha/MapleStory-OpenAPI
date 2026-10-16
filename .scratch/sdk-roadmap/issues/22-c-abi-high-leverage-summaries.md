# 22: C-ABI High-Leverage Typed Summaries

**What to build:** Expose high-leverage typed combat summary (`MapleStoryCombatSummary`) and coordination lookups in C-ABI, eliminating the need for C/C++ callers to parse JSON for combat calculations and appearance synthesis.

**Blocked by:** None

**Status:** done

- [x] Defined `MapleStoryCombatSummary` in `bindings/c/include/maplestory_openapi.h` and `bindings/c/src/lib.rs` with `int32_t` force/starforce/dojang fields and double multiplier fields.
- [x] Implemented `maplestory_character_get_combat_summary` (by OCID) and `maplestory_get_character_combat_summary` (by character name).
- [x] Implemented `maplestory_character_get_coordination` (by OCID) and `maplestory_get_character_coordination` (by character name).
- [x] Verified memory safety, null-pointer diagnostics, and end-to-end mock execution in `test_c_abi_notice_union_ranking_guild_and_account_summary`.
