# 19: Pure Character Dashboard Assembler (`CharacterDashboard::assemble`)

**What to build:** Extract pure synchronous assembler `CharacterDashboard::assemble(raw: RawDashboardSections, options: &DashboardOptions) -> Result<CharacterDashboard>` from `CharacterClient`, isolating domain error degradation and partial failure synthesis from concurrent network I/O.

**Blocked by:** None

**Status:** done

- [x] Defined `RawDashboardSections` domain input struct holding all 20+ endpoint section results.
- [x] Implemented pure synchronous `CharacterDashboard::assemble(raw, options)` propagating `basic` errors and degrading non-essential failures into `self.errors` via `unwrap_sec!` macro.
- [x] Refactored `CharacterClient::get_character_dashboard_with_options` to delegate to `assemble`, reducing line count by 300+ lines while preserving 100% backward compatibility.
- [x] Added microsecond-level unit tests (`test_pure_character_dashboard_assemble_and_partial_failures`) verifying partial failures without mock HTTP servers.
