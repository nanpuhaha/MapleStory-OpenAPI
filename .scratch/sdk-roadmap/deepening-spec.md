# Architecture Deepening Specification: MapleStory SDK Module Deepening

This specification covers the implementation of the 5 architectural deepening candidates identified in the codebase architecture review, transforming shallow coordination points into robust deep modules with simple public interfaces, high leverage, and clear test seams.

## Candidate Summary & Seams

1. **Ticket 19: Pure Dashboard Assembler (`CharacterDashboard::assemble`)**
   - **Seam**: `CharacterDashboard::assemble(raw_sections: RawDashboardSections) -> CharacterDashboard`
   - **Complexity Hidden**: 20-endpoint error degradation, fallbacks, partial failure recovery, and synthesis without network I/O.
   - **Public Client Interface**: Preserves `client.character().get_character_dashboard_with_options(...)` unchanged.

2. **Ticket 20: Dedicated Avatar Client (`AvatarClient`)**
   - **Seam**: `client.avatar()` returning `AvatarClient<'a>` with `render_gif(...)`, `render_custom_gif(...)`, `fetch_raw_frame(...)`.
   - **Complexity Hidden**: Multi-frame HTTP orchestration, animated GIF frame encoding, delay & disposal method defaults.

3. **Ticket 21: Decoupled Combat Analytics & Job Force Scaling**
   - **Seam**: `CharacterCombatAnalytics::from_stats(basic, stat, item_equipment, dojo, arcane_force, authentic_force, hyper_stat)`
   - **Complexity Hidden**: Xenon (+48 main stat / 10 AF) and Demon Avenger (+2100 HP / 10 AF) ARC/AUT scaling formulas, IED multiplicative composition, defense penetration simulation.

4. **Ticket 22: C-ABI High-Leverage Typed Summaries**
   - **Seam**: `maplestory_character_get_combat_summary` & `maplestory_character_get_coordination` in `maplestory_openapi.h`
   - **Complexity Hidden**: Direct Rust struct to C struct mapping without requiring C/C++ clients to parse 100KB JSON payloads.

5. **Ticket 23: Equip Layering Engine (`EquipLayerResolver`)**
   - **Seam**: `EquipLayerResolver::resolve(item_equipment, cash_item_equipment, beauty_equipment, symbol_equipment) -> CharacterCoordination`
   - **Complexity Hidden**: Transparent item filtering, anvil item appearance override precedence, cash vs normal equipment slot mapping priority rules.
