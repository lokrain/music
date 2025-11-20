# Performance Playbook (Epic 15)

A central reference for running our key Criterion benches, understanding the
current targets, and interpreting regressions locally or in CI.

## Reference hardware

- **Host:** Linux 6.6.87.2-microsoft-standard-WSL2 #1 SMP PREEMPT_DYNAMIC Thu Jun  5 18:30:46 UTC 2025 x86_64
- **Environment:** WSL2 on Windows 11 (8 performance cores, SMT on). Cargo 1.81+
  with `criterion 0.7.0` and default settings.
- All baselines were gathered on 2025-11-20; newer measurements should note the
  host + date in a fresh baseline doc under `docs/`.

## Bench suites & targets

| Suite | Command | Target Metric (median) | Notes | Baseline Doc |
| --- | --- | --- | --- | --- |
| Planner (music-score) | `cargo bench -p music-score --bench planner` | Ballad 4 bars ≈ **1.8 µs**; Gospel 8 bars ≈ **35 µs**; Pop hook 16 bars ≈ **215 µs** | Latency should scale roughly linearly with template bars; spikes >25% usually mean planner heuristics regressed. | `docs/perf_planner_baseline.md` |
| Spelling & staff mapping (music-notation) | `cargo bench -p music-notation --bench spelling_mapping` | Spelling 128 notes ≈ **3 µs**; 2048 notes ≈ **54 µs**; Staff pitch→offset ≈ **5 µs**, offset→pitch ≈ **8.5 µs** (per 2048-note batch) | Throughput should stay >35 M elems/s for spelling and >230 M elems/s for mapping. Deviations often indicate accidental heap allocations. | `docs/perf_spelling_baseline.md` |
| Time grid & alignment (music-time) | `cargo bench -p music-time --bench timegrid_alignment` | Grid build 2048 bars ≈ **141 µs**; Alignment 16k events ≈ **684 µs** | Grid construction scales with bar count; alignment should remain <1 ms for 16k events due to cache-friendly subdivision arrays. | `docs/perf_timegrid_baseline.md` |

## Running the benches locally

1. Install Criterion (already in workspace dependencies). Ensure `cargo +stable` is
   on the pinned toolchain (`rust-toolchain.toml`).
2. From repo root, run the desired suite (examples below). Use `--bench` to avoid
   running unrelated benches and keep noise low.
3. Criterion writes reports into `target/criterion/<bench>/`. Capture medians and
   copy them into the relevant baseline doc if they represent a meaningful change.

### Commands

- Planner: `cargo bench -p music-score --bench planner`
- Spelling/mapping: `cargo bench -p music-notation --bench spelling_mapping`
- Time grid/alignment: `cargo bench -p music-time --bench timegrid_alignment`

> Tip: Use `CARGO_TARGET_DIR=target/perf` if you want to isolate performance runs
> from normal builds.

## Regression interpretation

- **Tolerance:** Treat a >15% regression in any median as actionable. For planner
  latency, even 10% increases are suspicious because workloads are tiny.
- **First check noise:** Re-run the affected bench 2–3 times. Criterion reports
  outliers—if they are mostly “high mild/severe,” noise may be the culprit.
- **Bisecting:** Use `cargo bench planner -- --sample-size 200` to tighten error
  bars when bisecting commits.
- **Profiling hooks:** Each suite isolates a single subsystem. Use `perf record`
  or `cargo flamegraph` inside the same bench command to locate hotspots.

## CI expectations

- The `coverage` workflow already installs `cargo-llvm-cov`; extend it with
  targeted Criterion invocations when we add automated perf gating.
- Until automated gating lands, reviewers should request a rerun of the relevant
  bench whenever planner, notation spelling, or time-grid code changes.
- New performance-sensitive modules must:
  1. Add a Criterion bench under the corresponding crate’s `benches/` directory.
  2. Record a baseline doc under `docs/perf_<area>_baseline.md`.
  3. Update this playbook table with the command and targets.

## Adding new scenarios

When a new subsystem needs coverage:

1. Scope the scenario (inputs, expected complexity, invariants).
2. Add the bench and run it locally until noise is <5%.
3. Check the result into a new `perf_*_baseline.md` with host info, command, and
   the key table(s).
4. Amend the table above with the new suite and notify reviewers.
