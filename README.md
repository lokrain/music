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
