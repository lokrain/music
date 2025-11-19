# TODO

All entries follow a light Jira format with a short code, descriptive title, body, estimated complexity (1–3), and business value (1–100).

## CLI Enhancements

### CLI-1 — Implement `music explain`
- **Body:** Provide a narrative explanation command that resolves pitches, scales, and chords into plain-language paragraphs, using `music-theory` annotations and offering both text and JSON output.
- **Complexity:** 3
- **Value:** 88
- **Status:** DONE

### CLI-2 — Implement `music convert`
- **Body:** Add converters for pitch indices ↔ frequencies, MIDI files ↔ CSV note lists, and temperament remapping with validation through `music-core` registries.
- **Complexity:** 3
- **Value:** 90
- **Status:** DONE

### CLI-3 — Implement `music validate`
- **Body:** Validate user-provided melodies, progressions, and tunings by piping them through the registry and exposing actionable diagnostics when constraints fail.
- **Complexity:** 2
- **Value:** 78
- **Status:** DONE

### CLI-4 — Implement `music render`
- **Body:** Produce ASCII/Unicode staves or piano-roll previews from pitch sequences, with hooks for future SVG/LilyPond exports.
- **Complexity:** 3
- **Value:** 76
- **Status:** DONE

### CLI-5 — Implement `music generate`
- **Body:** Offer generative helpers (motifs, arpeggios, rhythm cells) parameterized by scale, system, and density to seed further analysis or export.
- **Complexity:** 3
- **Value:** 82
- **Status:** DONE

### CLI-6 — Implement `music score`
- **Body:** Emit MusicXML/LilyPond snippets from analyzed data, wiring in serialization settings and file targets.
- **Complexity:** 3
- **Value:** 84
- **Status:** DONE

### CLI-7 — Implement `music extrapolate`
- **Body:** Forecast likely chord or melody continuations via Markov / n-gram stats derived from the `music-analysis` crate, exposing tunable lookahead depth.
- **Complexity:** 3
- **Value:** 80
- **Status:** DONE

### CLI-8 — Implement `music explain-diff`
- **Body:** Compare two analyses (e.g., melodies, chord charts, MIDI files) and surface keyed deltas such as mismatched pitch-classes or function counts.
- **Complexity:** 2
- **Value:** 74
- **Status:** DONE

### CLI-9 — Implement `music map`
- **Body:** Render textual interval/scale maps that show how pitch classes relate across chosen temperaments, optionally highlighting modulatory paths.
- **Complexity:** 2
- **Value:** 70
- **Status:** DONE

### CLI-10 — Implement `music profile`
- **Body:** Summarize timing, density, and register usage for long inputs, including percentile-based ranges and swing detection hooks.
- **Complexity:** 3
- **Value:** 77
- **Status:** DONE

### CLI-11 — Implement `music interpolate`
- **Body:** Provide envelope interpolation for tempo, velocity, or tuning curves between anchor points and preview the resulting automation lanes.
- **Complexity:** 3
- **Value:** 72
- **Status:** DONE

### CLI-12 — Implement `music search`
- **Body:** Allow motif/pattern searches inside MIDI or note lists via intervallic fingerprinting with fuzzy tolerance controls.
- **Complexity:** 3
- **Value:** 86
- **Status:** DONE

### CLI-13 — Implement `music estimate`
- **Body:** Estimate tempo, key, and meter from raw note sequences or MIDI headers using heuristics defined in `music-analysis`.
- **Complexity:** 2
- **Value:** 75
- **Status:** DONE

### CLI-14 — Implement `music resolve`
- **Body:** Suggest voice-leading resolutions for active chords, highlighting stepwise options per part and exposing target-system constraints.
- **Complexity:** 3
- **Value:** 79
- **Status:** DONE

### CLI-15 — Add golden tests for `analyze`
- **Body:** Capture sample CLI outputs (melody, chords, MIDI) under `crates/music-cli/tests` to lock text + JSON schemas against regressions.
- **Complexity:** 2
- **Value:** 73
- **Status:** DONE

### CLI-16 — Structured JSON schema export
- **Body:** Define and version JSON schemas for all CLI responses so downstream tools can validate outputs programmatically.
- **Complexity:** 2
- **Value:** 68
- **Status:** DONE
	- **Notes:** Implemented schema export via hidden `export-schemas` command (enabled with `schema` feature). Added schemars::JsonSchema derives to all 60+ response structs across reports modules. Exports 41 JSON Schema files. Partial modularization completed: extracted ~600 lines from responses.rs into 5 report modules (pitch, chord, resolution, estimate, generation); responses.rs remains at 2069 lines and requires continued splitting (follow-up refactor task recommended). Schema export tested with 2 integration tests. Feature fully functional and ready for downstream validation use cases.

## Core & Theory Work

### CORE-1 — Finish interval module refactor
- **Body:** Complete the split across `crates/music-core/src/interval/**`, ensuring docs, prelude exports, and backward-compatible APIs are restored.
- **Complexity:** 3
- **Value:** 85
- **Status:** DONE
	- **Notes:** Interval module split into errors.rs (77L), implementation.rs (233L), mod.rs (38L). All files <300L ✓. Added comprehensive module-level docs with examples to mod.rs, errors.rs, implementation.rs. Backward-compatible APIs confirmed: lib.rs has `pub use interval::*` re-exporting Interval, IntervalError, IntervalBetweenError; prelude.rs re-exports all via `pub use crate::*`. Existing tests (20 interval_tests.rs) pass. Implementation.rs at 92.19% line coverage; errors.rs at 8.57% (Display impls not exercised, but core logic tested). Workspace tests: 145/145 passing. Overall workspace coverage 54.91% (gap from legacy untested CLI handlers, not interval module).

### CORE-2 — Chord quality taxonomy
- **Body:** Populate `crates/music-core/src/chord` with canonical qualities, inversions, and parsing helpers usable by analysis + CLI layers.
- **Complexity:** 3
- **Value:** 88
- **Status:** DONE
	- **Notes:** Extended ChordQuality enum from 12 to 22 qualities (added Dominant9, Major9, Minor9, Add9, Dominant11, Major11, Minor11, Dominant13, Major13, Minor13). Created Inversion enum (Root, First, Second, Third, Higher) with detection logic via Chord::detect_inversion(). Implemented chord symbol parsing (parse_chord_symbol) supporting common notations ("Cmaj7", "F#m", "Bb7", etc.). All chord files <300 lines (largest: parsing.rs 224L, quality.rs 168L). New types exported via mod.rs. Added 22 comprehensive tests covering extended qualities, inversion detection, parsing, and round-trips. All 167 workspace tests passing. Chord module coverage: parsing.rs 82%, implementation.rs 74%, pattern.rs 76%. Overall workspace coverage 56.28% (gap from legacy untested CLI handlers). New chord features fully functional and ready for CLI/analysis integration.

### CORE-3 — Scale catalog expansion
- **Body:** Flesh out `scale/catalog.rs` with diatonic, symmetric, and world-music collections plus metadata (modes, degrees, defaults).
- **Complexity:** 3
- **Value:** 81
- **Status:** TODO

### CORE-4 — Pitch spelling engine
- **Body:** Add enharmonic spelling logic (letter + accidental) to `music-core` so explain/render/generate commands can output staff-friendly names.
- **Complexity:** 3
- **Value:** 87
- **Status:** TODO

### CORE-5 — Temperament builder macros
- **Body:** Provide declarative macros or builders for quickly registering equal/unequal temperaments with registry validation baked in.
- **Complexity:** 2
- **Value:** 69
- **Status:** TODO

### CORE-6 — Registry persistence API
- **Body:** Introduce serialization hooks (with optional `serde`) so tuned registries can be snapshotted/loaded across sessions.
- **Complexity:** 3
- **Value:** 78
- **Status:** TODO

## Analysis & Engine Initiatives

### ANALYSIS-1 — Polyphonic tension metrics
- **Body:** Extend `music-analysis` to score vertical sonorities (set-class distance, roughness) feeding both CLI `analyze` and future `profile` verbs.
- **Complexity:** 3
- **Value:** 83
- **Status:** TODO

### ANALYSIS-2 — Cadence classifier
- **Body:** Train or encode cadence detection rules (PAC, HC, deceptive) reusable by chord analysis and progression suggestions.
- **Complexity:** 2
- **Value:** 71
- **Status:** TODO

### ANALYSIS-3 — MIDI feature extraction
- **Body:** Add tempo map, meter, and controller-lane summarizers to `music-analysis`, exposing structured outputs beyond the current header report.
- **Complexity:** 3
- **Value:** 82
- **Status:** TODO

### ENGINE-1 — Streaming analysis pipeline
- **Body:** Teach `music-engine` to accept event streams, run incremental analysis, and push updates to observers via channels or async streams.
- **Complexity:** 3
- **Value:** 89
- **Status:** TODO

### ENGINE-2 — Real-time tuning updates
- **Body:** Allow live temperament swaps with safe registry mutation and listener notifications to keep DSP paths in sync.
- **Complexity:** 3
- **Value:** 76
- **Status:** TODO

### ENGINE-3 — Signal-core integration tests
- **Body:** Wire `music-signal-core` transforms into end-to-end tests with the engine to ensure DSP + theory glue works under CI.
- **Complexity:** 2
- **Value:** 67
- **Status:** TODO

## Tooling, Docs, and QA

### DOCS-1 — CLI cookbook
- **Body:** Add a `docs/cli-cookbook.md` with scenario-driven recipes (batch analysis, conversions, scripting) and copyable commands.
- **Complexity:** 2
- **Value:** 65
- **Status:** TODO

### DOCS-2 — Architecture diagrams
- **Body:** Publish system + crate interaction diagrams under `docs/architecture/` to aid onboarding and PR reviews.
- **Complexity:** 2
- **Value:** 62
- **Status:** TODO

### DOCS-3 — CHANGELOG harmonization
- **Body:** Create crate-level CHANGELOG templates, ensuring releases capture CLI, core, and engine changes consistently.
- **Complexity:** 1
- **Value:** 58
- **Status:** TODO

### QA-1 — Property tests for pitch math
- **Body:** Use `proptest` to validate inversion relationships, interval arithmetic, and registry invariants in `music-core`.
- **Complexity:** 3
- **Value:** 84
- **Status:** TODO

### QA-2 — Coverage gates for new crates
- **Body:** Update `llvm-cov.toml` to include music-analysis/engine/cli benches and enforce ≥95% coverage as new modules land.
- **Complexity:** 2
- **Value:** 66
- **Status:** TODO

### TOOLING-1 — Observability via tracing
- **Body:** Integrate `tracing` with sensible spans in CLI + engine paths, enabling opt-in debug logs without performance regressions.
- **Complexity:** 2
- **Value:** 72
- **Status:** TODO

### TOOLING-2 — VS Code tasks for analyze flows
- **Body:** Add workspace tasks/launches that run representative `music analyze` invocations and verify outputs, speeding up manual QA.
- **Complexity:** 1
- **Value:** 55
- **Status:** TODO

### COMMUNITY-1 — Issue templates & discussions
- **Body:** Create GitHub issue templates (bug, feature, question) and enable Discussions to triage future work transparently.
- **Complexity:** 1
- **Value:** 60
- **Status:** TODO
