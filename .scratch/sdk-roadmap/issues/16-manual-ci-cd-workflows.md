# 16: Manual CI/CD Workflows (GitHub Actions with workflow_dispatch)

**What to build:** Create manual-trigger-only GitHub Actions workflows (`.github/workflows/ci.yml` and `.github/workflows/release.yml`) using `workflow_dispatch` without automatic push/PR triggers, covering `cargo test --all`, `cargo test --doc`, `cargo clippy`, examples check, and multi-platform build verification.

**Blocked by:** 15: Documentation, Architecture Guides & Examples Synchronization

**Status:** done

- [x] Create `.github/workflows/ci.yml` configured exclusively with `on: workflow_dispatch` (no automatic push/pr trigger).
- [x] Implement Rust validation jobs: `cargo test --all`, `cargo test --doc`, `cargo clippy --all-targets --all-features -- -D warnings`, and `cargo check --examples --all-features`.
- [x] Add matrix build job verifying compilation across Windows, Ubuntu, and macOS runners.
- [x] Ensure local validation passes before committing.
