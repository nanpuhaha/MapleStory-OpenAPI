# Ticket 22: C-ABI High-Leverage Typed Summaries

## Background & Motivation
C/C++ consumers and FFI bindings currently have to retrieve raw JSON strings and deserialize them with external JSON libraries just to get basic combat stats, starforce breakdown, or coordination equipment.

## Requirements
1. Define C-compatible structs in `bindings/c/include/maplestory_openapi.h` and `bindings/c/src/lib.rs`:
   - `MapleStoryCombatSummary`: combat_power (i64), damage (f64), boss_damage (f64), ignore_monster_armor (f64), total_starforce (i32), arcane_force (i32), authentic_force (i32), dojo_best_floor (i32).
   - `maplestory_character_get_combat_summary(client, ocid, out_summary) -> i32`
2. Provide coordination C-ABI endpoint:
   - `maplestory_character_get_coordination(client, ocid, out_coordination_json) -> i32`
3. Proper memory safety, NULL checks, and error logging via existing C-ABI infrastructure.
4. Add C tests / verification.
