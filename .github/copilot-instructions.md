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
