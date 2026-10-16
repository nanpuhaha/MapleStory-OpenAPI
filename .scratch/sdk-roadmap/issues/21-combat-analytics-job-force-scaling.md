# 21: Decoupled Combat Analytics & Job Force Scaling

**What to build:** Decouple `CharacterCombatAnalytics` construction from the 20-section `CharacterDashboard` by providing `from_stats(basic, stat, item_equipment, dojo, arcane_force, authentic_force, hyper_stat)`, and internalize job-specific Force stat scaling for Xenon and Demon Avenger.

**Blocked by:** None

**Status:** done

- [x] Implemented decoupled constructor `CharacterCombatAnalytics::from_stats` accepting individual components and optional hyper stat.
- [x] Added `CharacterCombatAnalytics::from_dashboard(dashboard)` convenience constructor.
- [x] Internalized job-specific Force formulas in `force_stat_multiplier`:
  - Xenon: +48 STR/DEX/LUK per 10 Arcane Force / Authentic Force.
  - Demon Avenger: +2100 Max HP per 10 Arcane Force, +4200 Max HP per 10 Authentic Force.
  - Standard classes: +100 main stat per 10 Arcane Force, +200 main stat per 10 Authentic Force.
- [x] Added unit tests verifying Xenon, Demon Avenger, and Hero stat bonus calculations in `test_combat_analytics_decoupled_and_job_specific_force_scaling`.
