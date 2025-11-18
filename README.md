# music

![Rust 2024](https://img.shields.io/badge/rust-2025-orange)
![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20%7C%20Apache--2.0-blue)
![Coverage ≥95%](https://img.shields.io/badge/coverage-%E2%89%A595%25-success)

High-performance Rust workspace for symbolic music theory, polyphonic analysis, DSP pipelines, engines, CLIs, and FFI bindings. Built for large-scale creative tooling with strict quality gates and predictable performance.

## Table of Contents

- [Highlights](#highlights)
- [Architecture at a Glance](#architecture-at-a-glance)
- [Getting Started](#getting-started)
- [Developer Workflow](#developer-workflow)
- [Performance & Benchmarks](#performance--benchmarks)
- [Coverage & Quality Gates](#coverage--quality-gates)
- [VS Code Productivity](#vs-code-productivity)
- [Contributing](#contributing)
- [License](#license)

## Highlights

- **Composable theory + DSP stacks**: `music-core` hosts tunings, pitch math, and registries used across analysis, engines, and CLI tooling.
- **Extensible tuning systems**: Built-in 12-TET, 24-TET, just intonation, cent scales, and registry helpers for custom systems.
- **Cross-crate rigor**: Workspace-level Clippy (pedantic + nursery) and coverage gates keep APIs production-ready.
- **Batteries included tooling**: `cargo-nextest`, Criterion benches, and `cargo-llvm-cov` wired into VS Code tasks/launchers.

## Architecture at a Glance

| Crate | Purpose |
| --- | --- |
| `music-core` | Core pitch/tuning primitives, registries, and shared infrastructure. |
| `music-theory` | Higher-level symbolic theory helpers. |
| `music-signal-core` | DSP primitives and transforms. |
| `music-analysis` | Analytical pipelines built atop core/theory modules. |
| `music-engine` | Runtime engine orchestrating registries and streaming subsystems. |
| `music-cli` | Command-line entry point for scripting and demos. |
| `music-ffi` | Bindings for host/plugin integrations. |
| `music-playground` | Sandbox binary for quick experiments. |

All crates share workspace lint/test configuration to guarantee consistent behavior.

## Getting Started

1. **Install prerequisites**
	 - Rust toolchain ≥ 1.91 (`rustup default stable`)
	 - `cargo-nextest` and `cargo-llvm-cov` for QA (see below)
2. **Clone & enter the workspace**

	 ```bash
	 git clone https://github.com/lokrain/music.git
	 cd music
	 ```

3. **Install auxiliary tooling once**

	 ```bash
	 cargo install cargo-nextest
	 cargo install cargo-llvm-cov
	 ```

4. **Run the full test suite**

	 ```bash
	 cargo nextest run --workspace --all-features
	 ```

## music-cli capabilities

The `music-cli` crate ships with a handful of production-ready commands that mirror the specifications in `CLI.md`/`HELP.md`. Everything routes through `cargo run -p music-cli -- …` in development, or the installed `music` binary in production.

### Enumerations (`music list …`)

**List tuning systems**

```bash
cargo run -p music-cli -- list systems
```

```
4 tuning systems registered (reference index 69).
	• 12tet: 440.000 Hz at index 69 — 12-TET(69)
	• …
```

**List diatonic chords**

```bash
cargo run -p music-cli -- list chords --root 60 --system 12tet --scale major --voicing triads
```

```
7 Triads derived from the Major scale in 12tet (root 60 …)
I    (degree 1): MajorTriad
	• tone 0: 12-TET(60) (261.626 Hz)
	• …
```

**List modal rotations**

```bash
cargo run -p music-cli -- list modes --scale dorian
```

```
7 mode(s) derived from the Dorian scale (root 60) in 12tet.
 1. Dorian (rotation 1, root 12-TET(60))
		• degree 1: 12-TET(60) (261.626 Hz)
		• …
```

`music expose tunings` and `music expose modes` reuse the same logic as the `list` verbs, so they surface identical data under the “expose” namespace.

### Inspecting objects

Describe a single pitch index inside a tuning system:

```bash
cargo run -p music-cli -- inspect pitch --index 69 --system 12tet
```

```
Pitch 69 in 12tet: 12-TET(69) (440.000 Hz)
```

### Explaining theory objects (`music explain …`)

**Explain a pitch**

```bash
cargo run -p music-cli -- explain pitch --index 69 --system 12tet --in Cmaj
```

```
Pitch 69 in 12tet: 12-TET(69) (440.000 Hz)
Octave 4, pitch class 9 (A).
Relative to A4: +0 semitone(s), +0.0 cents.
In 12-TET(60) Major, this is degree 6 (submediant), functioning as tonic.

12-TET(69) sits in octave 4 and resonates at 440.000 Hz inside 12tet.

It matches the concert A4 reference exactly.
```

**Explain a scale**

```bash
cargo run -p music-cli -- explain scale --root 60 --system 12tet --scale major --degrees 7
```

```
Scale explanation: Major rooted at 12-TET(60) (60) in 12tet.
Alias/rotation: Ionian.
Step pattern: step 1: 200.0 cents, …, step 7: 100.0 cents.
	• Degree 1 (12-TET(60)) — tonic, 261.626 Hz, 0.0 cents.
	• Degree 2 (12-TET(62)) — supertonic, 293.665 Hz, 200.0 cents +2 st.
	• Degree 3 (12-TET(64)) — mediant, 329.628 Hz, 400.0 cents +4 st.
	• Degree 4 (12-TET(65)) — subdominant, 349.228 Hz, 500.0 cents +5 st.
	• Degree 5 (12-TET(67)) — dominant, 391.995 Hz, 700.0 cents +7 st.
	• Degree 6 (12-TET(69)) — submediant, 440.000 Hz, 900.0 cents +9 st.
	• Degree 7 (12-TET(71)) — leading tone, 493.883 Hz, 1100.0 cents +11 st.

The Major template covers roughly 1.00 octave(s) in 12tet.

Degree 7 (12-TET(71)) stretches to 493.883 Hz and acts as leading tone tension.
```

**Explain a chord**

```bash
cargo run -p music-cli -- explain chord --root 60 --system 12tet --scale major --degree 5 --voicing triads
```

```
Chord explanation in 12tet: Major rooted at 12-TET(60) 60 (Triads).
Degree 5 (V) — MajorTriad, function: dominant.
	• tone 0: 12-TET(67) (391.995 Hz)
	• tone 1: 12-TET(71) (493.883 Hz)
	• tone 2: 12-TET(74) (587.330 Hz)

Stacked thirds produce a MajorTriad with 3 tone(s), emphasizing its dominant pull.

Root tone 12-TET(67) anchors the chord before the remaining voices outline the interval profile.
```

### Analyzing inputs (`music analyze …`)

**Melody statistics**

```bash
cargo run -p music-cli -- analyze melody --notes 60,62,64,65,67,69
```

```
Melody analysis: 6 notes (6 unique pitch classes).
Ambitus: 9 semitones (lowest 60, highest 69).
Best key: 12-TET(60) Major — match 100.0%
Tension: 0 notes (0.0%) outside the implied scale.
Pitch-class histogram:
	pc  0: 1
	pc  2: 1
	pc  4: 1
	pc  5: 1
	pc  7: 1
	pc  9: 1
```

**Chord function balance**

```bash
cargo run -p music-cli -- analyze chords --progression I,vi,ii,V --in Cmaj
```

```
Chord progression (4 chords, 4 unique):
	I → vi → ii → V
Functional counts: tonic 2, predominant 1, dominant 1, other 0.
Cadence: none detected.
Context key hint: Cmaj.
```

**MIDI file summary**

```bash
cargo run -p music-cli -- analyze midi --file /tmp/example.mid
```

```
MIDI file: /tmp/example.mid (25 bytes).
Standard MIDI header detected.
Declared format: 0, tracks: Some(1), detected chunks: 1.
Ticks per quarter note: 480.
```

### Suggesting reharmonizations

`Suggest::Reharm` enumerates parallel modes and surfaces chords that match an optional degree target:

```bash
cargo run -p music-cli -- suggest reharm --root 60 --system 12tet --scale major --voicing triads --degree 1
```

```
Parallel-mode reharmonization for the Major scale (root 60) in 12tet — Triads.
Target degree 1: 12-TET(60) (261.626 Hz).

 1. Ionian (rotation 1, root 12-TET(60))
	 * I    (degree 1) — MajorTriad, root 12-TET(60)
 2. Dorian (rotation 2, root 12-TET(62))
		(no matching chords)
	…
```

### Placeholder verbs

The remaining top-level verbs (`convert`, `validate`, `render`, `generate`, `score`, `extrapolate`, `explain-diff`, `map`, `profile`, `interpolate`, `search`, `estimate`, `resolve`) currently emit a friendly placeholder via `handle_placeholder`:

```
`music <verb>` is not implemented yet. Use `music <verb> --help` to preview its planned behavior.
```

This keeps the UX honest while the specs mature.

## Developer Workflow

- **Nextest** (default) — high-parallel tests with deterministic reporting:

	```bash
	cargo nextest run --workspace --all-features
	```

- **Standard benches** — Criterion harness (HTML output in `target/criterion`):

	```bash
	cargo bench -p music-core --bench pitch_bench
	```

- **Coverage** — LLVM-based with 95% minimum across the workspace:

	```bash
	cargo llvm-cov --workspace --all-features --fail-under-lines 95 --html
	```

- **Conventional cargo** — still available for targeted workflows (e.g., `cargo test -p music-core`).

## Performance & Benchmarks

- `music-core/benches/pitch_bench.rs` exercises abstract vs. literal pitch resolution.
- Criterion auto-generates statistical summaries and flamecharts under `target/criterion`.
- Add new benches by creating files under `crates/<crate>/benches` and wiring them into the crate’s `Cargo.toml`.

## Coverage & Quality Gates

- `llvm-cov.toml` enforces ≥95% line and region coverage.
- CI (and local developers) fail fast when coverage slips below threshold or when Clippy (pedantic + nursery) reports violations.
- Coverage output (LCOV + HTML) lands in `target/coverage/` for quick inspection.

## VS Code Productivity

- `.vscode/tasks.json` provides tasks for nextest, benches, and coverage.
- `.vscode/launch.json` adds one-click launchers:
	- `App: music-cli (cargo run)`
	- `QA: cargo nextest (workspace)`
	- `Perf: cargo bench (music-core)`
	- `Coverage: cargo llvm-cov (workspace)`
- Ensure the Rust Analyzer extension is enabled for inline diagnostics and code actions.

## Contributing

1. Fork + branch from `main`.
2. Keep changes focused and covered by tests/benches as appropriate.
3. Run `cargo nextest`, `cargo bench` (if performance-impacting), and `cargo llvm-cov` before opening a PR.
4. Document user-facing changes in crate-level `CHANGELOG`s (if applicable) and keep APIs `#[deny(clippy::pedantic, clippy::nursery)]` clean.

## License

Dual-licensed under [MIT](LICENSE) or [Apache-2.0](LICENSE). Choose the license that best fits your use case.
