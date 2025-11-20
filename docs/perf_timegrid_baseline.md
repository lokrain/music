# Time Grid & Alignment Benchmark Baseline (T15.3)

- **Date (UTC):** 2025-11-20
- **Host:** Linux 6.6.87.2-microsoft-standard-WSL2 x86_64 (WSL2)
- **Command:** `cargo bench -p music-time --bench timegrid_alignment`
- **Criterion:** v0.7.0 (default settings)

## Grid construction throughput

| Scenario | Bars | Subdivisions/Beat | Median Time (µs) | Throughput (M elems/s) | Notes |
| --- | --- | --- | --- | --- | --- |
| timegrid_build/32 | 32 | 4 | 2.04 | 15.67 | Baseline for short cues. |
| timegrid_build/128 | 128 | 4 | 8.33 | 15.36 | Stays linear vs. 32-bar case. |
| timegrid_build/512 | 512 | 4 | 34.43 | 14.87 | Minor cache-pressure dip (<5%). |
| timegrid_build/2048 | 2,048 | 4 | 141.26 | 14.50 | 2k-bar grids build in <0.15 ms. |

## Event alignment throughput

| Events | Median Time (µs) | Throughput (M elems/s) | Notes |
| --- | --- | --- | --- |
| 1,024 | 35.12 | 29.16 | Binary search against subdivision array; cache-hot. |
| 4,096 | 155.25 | 26.38 | ~4× work with modest throughput drop. |
| 16,384 | 683.41 | 23.97 | 0.68 ms for ~16k events; plenty of headroom. |

## Observations

- Grid construction scales linearly with bar count and sustains ~15 M elems/s even for 2k-bar timelines thanks to pre-reserved vectors.
- Alignment cost is dominated by binary searches over subdivision arrays; throughput gently tapers from 29 M/s to 24 M/s as datasets grow, still <1 ms for 16k events.
- Future optimization idea: cache subdivision step size per meter to skip binary search when events are near-quantized; track regressions by re-running this bench when adjusting quantization heuristics.
