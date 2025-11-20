## v1 Stabilization Checklist

### Compatibility & Semantics
- [x] Confirm public APIs in each crate have concise docs and `#[must_use]` annotations where appropriate (pass through music-score + documentation updates).
- [x] Audit `music-score` diff events to ensure attribute/edge diffs cover required flows (added `NoteUpdate`, edge-aware inserts/removals, and regression tests).

### CI & Tooling
- [x] Ensure `cargo fmt`, `cargo clippy --all-targets --all-features`, `cargo test --workspace`, and `cargo doc --workspace` pass locally (`2025-02-14` runs attached in release summary).

### Coverage & Testing
- [x] Produce coverage report via `cargo llvm-cov --workspace --summary-only` and document gaps (overall 84% region / 81% line coverage; see command output in release notes appendix).
- [x] Verify property tests in `music-acoustic` and regression suites in other crates run under CI (covered by `cargo test --workspace` / coverage run).

### Performance
- [x] Document performance expectations (current implementation validated on small/medium scores; large-score benchmarks flagged as future work in release notes).

### Release Notes
- [x] Draft release notes (see `docs/release_notes.md`) summarizing features per layer and known limitations/out-of-scope plans.
