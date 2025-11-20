# Spelling & Staff Mapping Benchmark Baseline (T15.2)

- **Date (UTC):** 2025-11-20
- **Host:** Linux 6.6.87.2-microsoft-standard-WSL2 x86_64 (WSL2)
- **Command:** `cargo bench -p music-notation --bench spelling_mapping`
- **Criterion:** v0.7.0 (default configuration)

## Spelling strategy throughput

| Dataset | Length | Median Time (µs) | Throughput (M elems/s) | Notes |
| --- | --- | --- | --- | --- |
| spelling_strategy/128 | 128 | 2.95 | 43.45 | Rotating keys over dense chromatic phrase. |
| spelling_strategy/512 | 512 | 12.06 | 42.44 | Sustains near-linear scaling; cache-friendly. |
| spelling_strategy/2048 | 2,048 | 53.61 | 38.20 | Slight drop from branch pressure; still <0.06 ms per batch. |

## Staff mapping throughput (natural phrase, 2,048 pitches)

| Scenario | Median Time (µs) | Throughput (M elems/s) | Notes |
| --- | --- | --- | --- |
| pitch_to_offset_treble | 5.16 | 396.96 | Treble conversion stays well under 0.01 ms. |
| pitch_to_offset_bass | 4.71 | 435.17 | Bass mapping benefits from lower offsets. |
| offset_to_pitch_treble | 8.48 | 241.51 | Reverse mapping cost dominated by modulo arithmetic. |
| offset_to_pitch_bass | 8.57 | 239.01 | Similar to treble; slight overhead from negative offsets. |

## Observations

- Spelling strategy scales almost linearly through 2k-note batches with <55 µs median runtime; keeping key/context arrays small minimizes cache churn.
- Staff offset lookups are effectively free (<6 µs per 2k notes) and reverse mapping remains <9 µs, providing generous headroom for engraving passes.
- Future optimizations: explore precomputing chromatic-to-letter maps per key to claw back the ~12% throughput dip at the 2k batch size.
