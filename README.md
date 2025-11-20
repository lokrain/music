# `music` — Theory, Notation, Engraving, and Score Engine
[![CI](https://github.com/lokrain/music/actions/workflows/ci.yml/badge.svg)](https://github.com/lokrain/music/actions/workflows/ci.yml)

A layered Rust workspace for music theory, notation, engraving, and score modeling.

## Crate Layout

- `core/`
  - `music-acoustic` — physical layer: frequencies, cents, ratios, temperaments.
  - `music-theory` — abstract theory: pitch, intervals, scales, chords, keys.
  - `music-time` — temporal grid: beats, meter, tempo, time spans.
- `notation/`
  - `music-notation` — symbolic notation: clefs, spellings, note values, tuplets.
  - `music-articulation` — articulations and dynamics.
  - `music-engraving` — engraving primitives: glyphs, positions, layout boxes.
- `score/`
  - `music-score` — high-level score model, typed events + graph overlay + diff API.

## Dependency Direction

Strict downward-only dependencies:

```text
music-acoustic
	↓
music-theory
	↓
music-time
	↓
music-notation ← music-articulation
	↓
music-engraving
	↓
music-score
```

* Lower layers never depend on higher layers.
* `music-score` is the integration point for building and transforming scores.

## Temperament & Pitch

* All pitch-related types are generic over a `Temperament` marker.
* 12-TET is exposed via the `T12` marker and convenient aliases like `Pitch12`.

## Notation & Spelling

* Theory (`music-theory`) has **no** notion of enharmonic spelling.
* Spelling lives entirely in `music-notation`, driven by strategies.

## Score Model

* Strongly typed events (`NoteEvent`, `RestEvent`, etc.).
* Graph overlay for relationships (ties, slurs, beams, voices).
* Diff-based API planned for efficient, undoable edits.

## Long-term Goals

- Serve as the canonical Rust foundation for advanced notation, analysis, and rendering tools.
- Keep every layer fully deterministic and explainable so engines built atop the workspace remain auditable.
- Provide language bindings (via future FFI crates) once the core APIs stabilize.
- Ship ready-to-use CLI and service front-ends after the v1 core reaches feature parity with modern commercial tooling.

## Developer Workflow

Use the workspace `Justfile` to keep routines consistent:

| Command | Description |
| --- | --- |
| `just check` | `cargo check --workspace --all-targets --all-features`. |
| `just fmt` / `just lint` | Format the repo or run fmt + pedantic clippy (same flags as CI). |
| `just test` | Executes `cargo test --workspace`. |
| `just doc` | Builds docs for all crates without pulling in dependencies. |
| `just bench` | Runs `cargo bench --workspace` (safe even if no benches exist). |
| `just release-dry-run` | Packages each publishable crate via `cargo package --locked --allow-dirty --no-verify`. |
