# Copilot instructions for the `music` workspace (v1)

These instructions reflect the v1 repository plan from the discovery, specification, and technical implementation documents (01, 02, 03). Treat this as the authoritative guide for how to design, structure, test, and land changes.

## Repository shape (monorepo)
- Rust workspace (edition 2024, resolver = 3)
- Top-level structure:
  - Cargo.toml (workspace + lint settings)
  - rust-toolchain.toml (pinned toolchain)
  - .cargo/config.toml (resolver=3, shared flags)
  - crates/
    - music-core (theory + timing)
    - music-structure (phrases/sections/graph, templates)
    - music-analysis (harmonic + melodic engine, planner)
    - music-api (HTTP/WS/SSE API, models, persistence)
    - music-cli (local tool for dev/bench/testing)
  - tests/ (workspace-level integration as needed)

## Crate responsibilities
- music-core: pitch/interval/scale/chord/key/meter/tuning, raw/grid timing, shared errors.
- music-structure: phrase/section modeling, StructureGraph, template DSL + compile + registry.
- music-analysis: feature extraction, style rule-graph, beam-search planner, melody engine, explainability capture/render.
- music-api: REST + streaming endpoints, models, simple run persistence.
- music-cli: entrypoints to exercise the stack locally; golden tests and benches.

## Module layout expectations (per crate)
- lib.rs as entrypoint; clear feature flags; re-export public API via crate prelude.
- Split modules to keep files around 300–400 LOC. Prefer submodules to avoid god-files.
- Colocate unit tests next to source or under a mirrored tests tree (see Test Layout).

## Quality guardrails (strict)
- Deny on warnings, unused imports, and dead code in application crates.
- No unwrap/expect in non-test code; introduce typed error enums and propagate.
- Public API must be documented with rustdoc comments and examples where helpful.
- Maintain single source of truth: no duplicated logic; refactor shared helpers.

## Testing & coverage
- Fast loop:
  - cargo nextest run --workspace --all-features
- Coverage gate:
  - cargo llvm-cov --workspace --all-features --fail-under-lines 95
- Golden tests: CLI and API text/JSON locked outputs for regressions.
- Fuzz/property tests for math-heavy core (intervals, pitch math, conversions).

## Performance & correctness
- Favor iterator-based tight loops, avoid unnecessary allocation; consider Arc for shared state.
- Keep algorithms deterministic for identical inputs/config.
- Benchmarks for hot paths (core math, planner inner loops, end-to-end where feasible).

## Explainability
- All major decision points in analysis/planner expose light-weight capture hooks.
- Rendering supports modes: none / brief / detailed / debug (debug may be feature-gated).

## Task Execution Protocol (Agent)
Follow this mini-Jira flow strictly to ensure consistent delivery.

### Start a task
1. Pick the earliest TODO with Status=TODO in `TODO.md`.
2. Immediately flip it to IN_PROGRESS.
3. Create/update an internal checklist (tests, coverage, refactors, docs).

### Definition of Done (DoD)
1. Functionality works (manual spot check + automated tests).
2. Coverage ≥ 95% lines (workspace) via cargo llvm-cov gate.
3. All discovered sub-work appended to TODO.md with clear status.
4. Design notes/trade-offs captured in the task body under TODO.md.
5. No file exceeds ~300–400 LOC; split into logical submodules if necessary.
6. Tests colocated and mirror source hierarchy; integration tests for CLI/API remain under crate-level tests/ when appropriate.
7. Remove duplication; if blocked, add a refactor task and link it.

### Test layout rules
- For `src/foo/bar.rs`, create `tests/foo/bar_test.rs` for focused unit tests; avoid omnibus.
- End-to-end: CLI/API under `crates/music-cli/tests/` or `crates/music-api/tests/`.

### Failure handling
- If a test or coverage gate fails: iterate with focused fixes.
- If blocked by structure, create a follow-up investigation/refactor task.

## Implementation phasing snapshot (v1)
Phases are sequenced as in 03. TECHNICAL_IMPLEMENTATION_PLAN.md — Core → Structure → Templates → Analysis → Style Graph → Planner → Melody → Explainability → Orchestrator → API → CLI.

## Practical tips
- Keep public re-exports tidy (`lib.rs`/prelude.rs) to avoid deep paths across crates.
- Use feature flags consistently for optional serde and debug/trace hooks.
- Keep file sizes in check; split early rather than late.

If anything is unclear or diverges from the three canonical docs (01, 02, 03), align with them first and add TODO entries to reconcile code with plan.
