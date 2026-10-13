# 17: Packaging & Release Readiness (crates.io, Python, Node.js)

**What to build:** Complete crate metadata in `Cargo.toml` (`description`, `license`, `repository`, `documentation`, `keywords`, `categories`, `readme`, `exclude`), verify clean `cargo publish --dry-run -p maplestory-openapi`, and configure multi-language release scripts/metadata for Python (`maturin`) and Node.js (`napi-build`).

**Blocked by:** 16: Manual CI/CD Workflows

**Status:** done

- [x] Enrich `maplestory-openapi/Cargo.toml` with complete crates.io metadata (`description`, `repository`, `keywords`, `categories`, `readme`, `exclude`/`include`).
- [x] Verify `cargo publish --dry-run -p maplestory-openapi` executes cleanly without packaging warnings or errors.
- [x] Review and standardize package configurations across `bindings/python/pyproject.toml` and `bindings/nodejs/package.json`.
- [x] Add a release checklist / script `scripts/verify-release.sh` or PowerShell script `scripts/verify-release.ps1`.
