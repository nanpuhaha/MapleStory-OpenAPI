# Ticket 21: Decoupled Combat Analytics & Job Force Scaling

## Background & Motivation
`CharacterCombatAnalytics` could previously only be constructed from a 20-section `CharacterDashboard`, even though only basic stats, force, and dojo are needed. Furthermore, job-specific force scaling (Xenon 10 AF -> +48 main stats, Demon Avenger 10 AF -> +2100 HP) was incomplete or required manual caller adjustment.

## Requirements
1. Decouple `CharacterCombatAnalytics` constructor:
   - Provide `CharacterCombatAnalytics::from_stats(basic, stat, item_equipment, dojo, arcane_force, authentic_force, hyper_stat)` so callers don't need a full dashboard.
   - Maintain `CharacterCombatAnalytics::from_dashboard(dashboard)` as a high-leverage convenience constructor.
2. Internalize job-specific Force stat scaling formulas:
   - Xenon: 10 Arcane Force / Authentic Force gives +48 to STR/DEX/LUK (instead of standard +100).
   - Demon Avenger: 10 Arcane Force gives +2100 Max HP, Authentic Force gives +4200 Max HP (instead of standard +100).
   - Standard classes: 10 Arcane Force gives +100 main stat, Authentic Force gives +200 main stat.
3. Tests verifying job-specific stat bonuses and decoupled constructor.
