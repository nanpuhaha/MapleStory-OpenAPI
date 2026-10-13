# 12: C-ABI & Native Wrappers Expansion (C, C# .NET, Go, Mobile) with Error Diagnostics

**What to build:** Expand `bindings/c` (`maplestory_openapi.h` and `src/lib.rs`) beyond character dashboard/GIF to expose guild profile, union, overall ranking, notices, and account summary, along with structured last-error message retrieval (`maplestory_last_error_message`), and synchronize C# (`.NET`), Go (`cgo`), Swift, and Kotlin wrappers.

**Blocked by:** 07: Introduce hierarchical domain namespaces on MapleStoryClient, 10: Rich Domain Aggregators

**Status:** done

- [x] Add thread-local or handle-scoped error diagnostic string retrieval (`maplestory_last_error_message`) in `bindings/c` so FFI callers get actionable error messages instead of opaque `-3` codes.
- [x] Expose C-ABI functions for `maplestory_get_guild_profile`, `maplestory_get_user_union`, `maplestory_get_overall_ranking`, `maplestory_get_notice_list`, and `maplestory_get_account_summary` in `bindings/c/src/lib.rs` and `bindings/c/include/maplestory_openapi.h`.
- [x] Synchronize `bindings/dotnet/MapleStoryClient.cs`, `bindings/go/maplestory.go`, `bindings/mobile/swift/MapleStoryClient.swift`, and `bindings/mobile/kotlin/MapleStoryClient.kt` with the new C-ABI endpoints.
- [x] Add unit tests in `bindings/c` verifying FFI lifecycle, JSON output, and error diagnostics against a mock server.
