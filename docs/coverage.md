# Coverage Gates

We keep harmonic theory/time/notation crates at **≥95% line coverage** and the planner/score stack at **≥90%**. These targets are enforced in CI via `cargo llvm-cov` and should be exercised locally before opening a pull request.

## Targets per crate group

| Group | Crates | Minimum line coverage |
| --- | --- | --- |
| Theory & time primitives | `music-theory`, `music-time`, `music-acoustic` | 95% |
| Notation stack | `music-articulation`, `music-notation`, `music-engraving` | 95% |
| Planner & score | `music-score` | 90% |

> **Note:** `music-acoustic` is counted with theory/time because it provides the shared tuning model used across those crates.

## Running locally

1. Install `cargo-llvm-cov` (once):

   ```bash
   cargo install cargo-llvm-cov
   ```

2. From the repo root, run the high-threshold group:

   ```bash
   cargo llvm-cov --locked \
     -p music-acoustic \
     -p music-theory \
     -p music-time \
     -p music-articulation \
     -p music-notation \
     -p music-engraving \
     --fail-under-lines 95
   ```

3. Re-run without cleaning for the planner/score crate (90% gate):

   ```bash
   cargo llvm-cov --locked --no-clean -p music-score --fail-under-lines 90
   ```

Adding `--no-clean` on the second invocation reuses the instrumented artifacts from the first run so coverage completes quickly. Both commands will fail if any crate dips below its respective threshold.

## CI integration

The `coverage` job in `.github/workflows/ci.yml` runs the same two commands, ensuring every pull request meets the documented gates. If the job fails, run the commands above locally, add tests to raise coverage, and re-run until the thresholds are satisfied.
