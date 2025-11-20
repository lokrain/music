# Contributing to `music`

Thanks for helping grow the `music` workspace. This document summarizes the
expectations for coding style, testing, and local workflows so every crate stays
consistent.

## Prerequisites

- Rust 1.91+ (pinned via `rust-toolchain.toml`).
- `cargo fmt`, `cargo clippy`, and `cargo test` available in your toolchain.
- Optional tooling such as `cargo-nextest` or `just` can speed up local loops,
  but the standard Cargo commands are sufficient for contributions.

## Coding Style & Lints

- Edition 2024, resolver 3.
- `unsafe` is forbidden; do not add `unsafe` blocks without a dedicated RFC.
- Module/file size target: keep individual files ≲400 LOC. Split modules when
  types or implementations grow beyond that limit.
- Follow `cargo fmt` output exactly. CI runs `cargo fmt --check` on every PR.
- Treat all Clippy warnings as errors. The workspace enables pedantic lints plus
  `unwrap_used`/`expect_used`; prefer explicit error handling.

## Testing Expectations

- Add targeted unit tests for every new type and behavior. Prefer doctests when
  small examples explain the API.
- Run `cargo test` at the workspace root before opening a PR. If a change only
  touches a single crate, still smoke the root test suite once to keep cross-
  crate assumptions honest.
- When adding serialization/deserialization or math-heavy code, include edge
  cases (e.g., octave wrapping, negative spans, large tuplets).

## Local Workflow

1. `cargo fmt`
2. `cargo clippy --all-targets --all-features`
3. `cargo test`

Repeat until all three pass. If Clippy flags a new lint, prefer fixing the code
instead of silencing the lint—file-level allows should be the exception.

## Commit & PR Guidelines

- Keep commits focused; one logical change per commit makes reviews easier.
- Reference the relevant JIRA task ID in the commit message when available.
- Update `todo.md` statuses as you start (`IN PROGRESS`) or finish (`DONE`) a
  task so planning stays in sync with the repo.

Welcome aboard! EOF
