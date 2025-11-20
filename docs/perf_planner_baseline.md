# Planner Benchmark Baseline (T15.1)

- **Date (UTC):** 2025-11-20
- **Host:** Linux 6.6.87.2-microsoft-standard-WSL2 x86_64 (WSL2)
- **Command:** `cargo bench -p music-score --bench planner`
- **Criterion:** v0.7.0 (default settings)

| Scenario | Template | Bars | Style Profile | Median Time (µs) | Notes |
| --- | --- | --- | --- | --- | --- |
| planner/ballad/4 | custom four-bar ballad | 4 | `StyleProfile::smooth_ballad()` | 1.8256 | 3% high-severe outliers, still well under 2 µs |
| planner/gospel/8 | `builtin_template("gospel_vamp_v1")` | 8 | smooth ballad | 34.556 | No anomalous samples |
| planner/pop_hook/16 | `builtin_template("pop16_hook_v1")` | 16 | `StyleProfile::pop_radio()` | 214.95 | 4 mild outliers; deterministic output |

## Observations

- Planner latency scales roughly linearly with template length; doubling from 8 to 16 bars increases runtime ~6× due to denser harmonic search on the pop template.
- Gospel template remains within tens of microseconds, leaving ample headroom for real-time preview.
- Next steps for T15.1: monitor regressions via `cargo bench` in CI and capture deltas whenever adding planner heuristics.
