# AI Coding Standards & Architecture Guidelines

This document serves as the central source of truth for architectural principles, coding conventions, and development workflows for this project.

**Agent Instructions:**
- **Implementer Agents:** Use this document as a *Pull* reference. Do not load this entire file into every task context. Query this file only when you need to resolve architectural decisions, establish test boundaries, or verify project-specific conventions.
- **Review Agent / Code Quality Agent:** This document is *Push* context for you. You must evaluate all implemented code against the core principles outlined here. Any deviation without documented justification should result in a failed review or a required refactor.

---

## 1. Core Architecture Principles

### 1.1 Deep Modules over Shallow Modules
Following John Ousterhout's philosophy of software design, this project strictly adheres to a **Deep Module** architecture.

*   **Definition:** A deep module provides a simple, minimal public interface while hiding substantial complexity and implementation details within.
*   **Agent Directive (Implementation):** Do not create "shallow" wrappers or pass-through functions that expose internal state or require the caller to orchestrate complex logic. Encapsulate related state and behavior. 
*   **Agent Directive (Review):** Flag modules that have broad API surfaces but trivial implementations. Ensure that callers are not burdened with implementation details (information hiding).

### 1.2 Vertical Slices (Tracer Bullets)
Development must proceed in **Vertical Slices**, completely avoiding horizontal layer-by-layer development (e.g., building all DB tables, then all APIs, then all UI).

*   **Definition:** A vertical slice implements a single, focused feature from the user interface all the way down to the database/infrastructure layer as an end-to-end unit.
*   **Agent Directive (Implementation):** When assigned a feature, build the *thinnest possible* end-to-end slice first (a "tracer bullet") to prove the integration works, then expand the functionality.
*   **Agent Directive (Review):** Reject PRs or implementations that only introduce horizontal layers without a connected, testable user-facing or API-facing feature.

---

## 2. Development & Testing Workflow

### 2.1 Red-Green-Refactor (TDD)
Test-Driven Development is mandatory to prevent AI agents from "hallucinating" implementations and writing tests after the fact to artificially pass.

*   **The Cycle:**
    1.  **Red:** Write *one* failing test that defines a specific desired behavior. Run the test to confirm it fails.
    2.  **Green:** Write the *minimum* amount of code necessary to make that specific test pass.
    3.  **Refactor:** Clean up the implementation (applying Deep Module principles) while keeping the test suite green.
*   **Agent Directive (Implementation):** You must prove a test fails before implementing the solution. Do not write the entire test suite upfront (horizontal testing); iterate one behavior at a time.
*   **Agent Directive (Review):** Verify that tests cover the required behavior and are not simply mocking out the entire implementation just to achieve coverage.

### 2.2 Automated Feedback Loops
Agents must rely on immediate, automated feedback rather than waiting for human intervention or CI/CD pipelines to catch basic errors.

*   **Mandatory Verification Steps:** After any code modification, the implementing agent MUST execute the following commands before considering the task complete:
    *   **Type Checking:** `cargo check --workspace --all-targets --all-features`. Resolve *all* compile errors immediately.
    *   **Linting/Clippy:** `cargo clippy --workspace --all-targets --all-features -- -D warnings`. Ensure 0 warnings.
    *   **Unit & Integration Tests:** `cargo test --workspace --all-features` to ensure all tests pass.
*   **Agent Directive (Implementation):** If a command fails, you must analyze the terminal output, correct the code, and re-run the command until it passes. Do not leave broken builds for the Reviewer agent.

### 2.3 Mandatory Git Commit & Remote Push
Development artifacts and task deliverables must never remain unpushed on local machines.

*   **Continuous Synchronization:** Every completed unit of work, feature slice, bug fix, or review remediation must be committed with a descriptive Conventional Commit.
*   **Immediate Remote Push:** Following each commit or completed task, the agent MUST immediately push to the remote repository (`git push origin <branch>`). Local branches must not lag behind or accumulate unpushed commits.

---

## 3. Usage in the AI Workflow

To maintain context limits (the "smart zone") and prevent token bloat:

1.  **Progressive Disclosure:** Do not duplicate large swaths of documentation here. If a specific tool (e.g., Tailwind, Prisma) has its own complex conventions, link to a separate `.md` file (e.g., `docs/PRISMA_STANDARDS.md`).
2.  **Context Loading:** 
    *   *Humans/Planners:* Reference this file when generating specs or breaking down issues.
    *   *Coders:* Fetch specific sections using tool calls when ambiguous situations arise.
    *   *Reviewers:* Inject the "Core Architecture Principles" and "Development & Testing Workflow" sections into your system prompt for every PR review.
