# 15: Documentation, Architecture Guides & Examples Synchronization

**What to build:** Update `README.md`, `docs/` (`architecture.md`, `multi-language-bindings.md`, `dashboard-guide.md`, `recipes-and-examples.md`), and `maplestory-openapi/examples/` to document hierarchical domain sub-clients, concurrency/retry configuration, `MapleDate`, `IdentityCache`, potential/progression helpers, `GuildProfile`, and `AccountSummary`.

**Blocked by:** 12: C-ABI & Native Wrappers Expansion, 13: Union Raider Board & Union Champion Deep Domain Helpers, 14: Character Progression Deep Domain Helpers

**Status:** done

- [x] Update `README.md` and `docs/architecture.md` to document the hierarchical domain namespaces (`client.character()`, `client.guild()`, `client.union()`, `client.ranking()`, `client.history()`, `client.notice()`, `client.user()`, `client.battle_practice()`, `client.scheduler()`).
- [x] Update `docs/multi-language-bindings.md` with the expanded Python, Node.js, C, .NET, Go, and Mobile APIs.
- [x] Add or update runnable example in `maplestory-openapi/examples/` demonstrating domain sub-clients, potential stat aggregation, union/guild aggregators, and history streams.
- [x] Verify all doc-tests and examples compile cleanly with `cargo test --doc` and `cargo check --examples`.
