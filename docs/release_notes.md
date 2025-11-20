# `music` v1 Release Notes (Draft)

## Highlights
- **Acoustic layer**: Temperament-aware frequency, ratio, and cents primitives with registries for runtime tuning.
- **Theory layer**: Temperament-generic pitch, interval, scale, chord, key, and harmonic function APIs with exhaustive tests.
- **Time layer**: Beat/time span arithmetic, meter/tempo conversions, and configurable time grids.
- **Notation layer**: Note spellings, durations, staff positions, and key-aware spelling strategies, plus documentation/examples.
- **Articulation & Dynamics**: Stable taxonomies for articulations/ornaments and ordered dynamics with profile helpers.
- **Engraving**: Layout positions/boxes, glyph/stem helpers, and geometry regression tests.
- **Score**: Typed events, relationship graph, diff/patch model with attribute updates, and cross-crate example (`examples/full_flow.rs`).

## Known Limitations
- Score diff edges do not yet auto-sync beams/voice relationships when reordering events.
- Performance characteristics are validated on small/medium scores; large-score benchmarks planned post-v1.
- Crate APIs are intentionally conservative; async/real-time features are out of scope for v1.

## Upgrade Notes
- All crates follow semantic versioning; breaking changes will require a major version bump.
- Consumers should rely on `ScoreDiff` for mutations to remain compatible with future graph-aware behaviors.
