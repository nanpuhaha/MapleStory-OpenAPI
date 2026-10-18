# Project Development & Synchronization Rules

This project enforces strict standards for AI-assisted development, continuous integration, and git synchronization.

## 1. Mandatory Git Commit & Push Workflow
- **Commit on completion**: Every completed task, feature slice, bug fix, or review remediation MUST be committed with a descriptive Conventional Commit message.
- **Always push immediately**: Every commit (or batch of related commits in a completed task) MUST be pushed to the remote repository (`git push origin <current-branch>`) before finishing the turn or reporting task completion.
- **No stale local commits**: The working tree must be clean and local branches must remain fully synchronized with the remote (`up to date with origin/<branch>`).

## 2. Quality Gates Before Commit & Push
Before committing and pushing, all the following feedback loops must pass with 0 errors and 0 warnings:
1. `cargo check --workspace --all-targets --all-features`
2. `cargo clippy --workspace --all-targets --all-features -- -D warnings`
3. `cargo test --workspace --all-features`
