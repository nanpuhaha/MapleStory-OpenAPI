# 09: Multi-language Binding Expansion (Python & Node.js Domain Sub-Clients)

**What to build:** Synchronize the new Rust hierarchical domain sub-clients (`guild()`, `union()`, `ranking()`, `history()`, `notice()`, `scheduler()`) and new data models across Python (`pyo3`) and Node.js (`napi-rs`) bindings to achieve feature parity with the core Rust SDK.

**Blocked by:** 07: Introduce hierarchical domain namespaces on MapleStoryClient

**Status:** done

- [x] Expose domain namespaces or full endpoint methods on `PyMapleStoryClient` in `bindings/python/src/lib.rs` (Guild, Union, Ranking, History, Notice).
- [x] Expose domain endpoints on `JsMapleStoryClient` in `bindings/nodejs/src/lib.rs`.
- [x] Ensure `pythonize` and `serde_json` serialization works without GIL bottlenecks or async panic.
- [x] Add integration smoke tests in Python and Node.js verifying domain call invocations.
