# Copilot instructions for the `music` workspace

## Architecture snapshot
- This is a multi-crate Rust workspace; most shared logic lives in `crates/music-core`. Other crates (analysis, engine, CLI, FFI, etc.) consume the `music-core` APIs, so prefer adding reusable primitives there first.
- `music-core` models pitches in four layers: `system.rs` defines `PitchSystemId` + `PitchSystem`; `registry.rs` hosts `TuningRegistry` for lookups; `pitch.rs` wraps literal/abstract pitches plus labels & errors; `interval.rs` stores ratios/step deltas for cross-system transposition.
- Temperaments (12/24 TET, etc.) are in `crates/music-core/src/systems/**` and usually compose `EqualTemperament`. New systems should implement `PitchSystem` + re-export through `systems/mod.rs` and, if widely used, the prelude.

## Key patterns & expectations
- Always validate external identifiers with `PitchSystemId::try_new` or `FromStr`; the infallible `new`/`from` helpers are for trusted literals.
- The registry owns `Arc<dyn PitchSystem>` values. Prefer the helper APIs (`register_system`, `try_register_system`, `with_system`, `get_or_insert_with`, `resolve_*_str`) instead of accessing the map directly.
- Intervals capture both frequency ratios and optional step deltas. When applying an interval, the result stays abstract only if the stored system matches the target pitch; otherwise it produces a literal frequency.
- `Pitch` APIs must preserve the literal/abstract distinction. Every method resolving frequencies takes a `&TuningRegistry` and returns `PitchError`. Keep literal validation centralized via `validate_literal_frequency`.
- Exposed symbols are re-exported from `lib.rs` and `prelude.rs`. Whenever you add new core types (errors, helpers, systems) confirm they’re exported consistently so downstream crates compile without explicit module paths.
- Feature flags: default build enables `std`; `serde` is optional—gate derives/uses with `#[cfg(feature = "serde")]` just like existing modules.

## Testing & workflows
- Fast unit/integration loop for the core crate:
  ```bash
  cargo test -p music-core
  ```
- Workspace-wide QA (preferred before PRs) piggybacks on the documented tools:
  ```bash
  cargo nextest run --workspace --all-features
  cargo llvm-cov --workspace --all-features --fail-under-lines 95
  ```
- Keep files formatted via `cargo fmt`. Clippy is enforced at CI with pedantic + nursery + `unwrap/expect` denies, so avoid those patterns or gate them with clear justifications.

## Examples & references
- See `crates/music-core/tests/registry_tests.rs` for registry ergonomics, `tests/pitch_tests.rs` for pitch/serde behaviors, and `tests/interval_tests.rs` for interval round-trips.
- The workspace README documents tooling expectations, required Rust toolchain (`rust-toolchain.toml`), and VS Code tasks—review it when adding new developer flows.

Please ping the maintainers if any part of this guide feels incomplete so we can refine it for future agents.

## Task Execution Protocol (Agent Augmentation)

These workspace-specific execution rules refine how agents should pick, start, and finish tasks, ensuring consistency with the project's QA and maintainability standards.

### Starting a Task
1. Always select the earliest (top-most) item in `TODO.md` whose **Status** is `TODO`.
2. Immediately change its status to `IN_PROGRESS` in `TODO.md` before writing code.
3. Create or update an internal action checklist (using the todo list management tool) covering implementation, tests, coverage, refactors, and documentation updates.

### Definition of Done (DoD)
To mark a task `DONE`, ALL conditions must hold:
1. Feature/functionality works end-to-end (manual spot check + automated tests).
2. Test coverage for the affected crate/module ≥95% line coverage (run `cargo llvm-cov --workspace --all-features --fail-under-lines 95`). If new code lowers coverage, add tests until threshold passes.
3. Any new sub-work discovered is appended to `TODO.md` (new tasks with clear status).
4. Notes/remarks (design decisions, trade-offs, follow-ups) appended to the relevant task body in `TODO.md`.
5. No source file exceeds 300 lines after changes. If exceeded:
  - Split into logical submodules under the same crate.
  - Mirror the split with matching test files (see Test Layout below).
  - Add a refactor task to `TODO.md` if split cannot be completed immediately.
6. Tests reside in a `src/tests/` (or crate-local) structure mirroring source hierarchy: each `foo.rs` has a `foo_test.rs` sibling (or module-level `mod tests` inside `foo.rs` only when trivial). Integration tests may still live under `tests/` for CLI end-to-end flows, but unit/business logic MUST have colocated mirrors.
7. Single source of truth: duplicated business logic must be refactored. If duplication is detected but cannot be resolved instantly, add a refactor task to `TODO.md`.

### Test Layout Rules
* For a module `src/bar/baz.rs`, create `src/bar/baz_test.rs` containing focused unit tests.
* Avoid large omnibus test files covering multiple modules.
* End-to-end / CLI golden tests remain under `crates/music-cli/tests/`.

### Aborting / Cancelling Work
If a task is abandoned (blocked, superseded, invalid):
1. Change its status to `BLOCKED` or `CANCELLED` in `TODO.md` with a brief rationale appended under the task body.
2. Add any derivative tasks if partial progress created new artifacts needing cleanup.

### Coverage & Quality Commands
* Full workspace: `cargo nextest run --workspace --all-features`
* Coverage gate: `cargo llvm-cov --workspace --all-features --fail-under-lines 95`

### File Size Monitoring
Use ad-hoc scanning or tooling scripts to flag files >300 lines. Prioritize splitting high-churn or multi-responsibility files first (handlers, large response renderers, composite logic).

### Refactor Guidance
When splitting large files:
* Preserve public API re-exports in `lib.rs` / `prelude.rs`.
* Prefer moving pure helper functions to a private `mod util;` or into `music-core` for reuse.
* Update tests to mirror the new file boundaries immediately.

### Task Flow Cycle
1. Pick first `TODO` → mark `IN_PROGRESS`.
2. Implement incrementally with tests.
3. Ensure coverage ≥95%.
4. Enforce file-size rule.
5. Update task with NOTES if needed → mark `DONE`.
6. Automatically begin next `TODO` task; do not wait for user prompt.

### Failure Handling
If a test or coverage gate fails:
* Iterate with focused fixes.
* On persistent structural blocker, create a new refactor or investigation task.

---
This protocol ensures maintainability, traceability, and consistent quality across rapid CLI feature additions.
