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
- **Status:** TODO

### CLI-3 — Implement `music validate`
- **Body:** Validate user-provided melodies, progressions, and tunings by piping them through the registry and exposing actionable diagnostics when constraints fail.
- **Complexity:** 2
- **Value:** 78
- **Status:** TODO

### CLI-4 — Implement `music render`
- **Body:** Produce ASCII/Unicode staves or piano-roll previews from pitch sequences, with hooks for future SVG/LilyPond exports.
- **Complexity:** 3
- **Value:** 76
- **Status:** TODO

### CLI-5 — Implement `music generate`
- **Body:** Offer generative helpers (motifs, arpeggios, rhythm cells) parameterized by scale, system, and density to seed further analysis or export.
- **Complexity:** 3
- **Value:** 82
- **Status:** TODO

### CLI-6 — Implement `music score`
- **Body:** Emit MusicXML/LilyPond snippets from analyzed data, wiring in serialization settings and file targets.
- **Complexity:** 3
- **Value:** 84
- **Status:** TODO

### CLI-7 — Implement `music extrapolate`
- **Body:** Forecast likely chord or melody continuations via Markov / n-gram stats derived from the `music-analysis` crate, exposing tunable lookahead depth.
- **Complexity:** 3
- **Value:** 80
- **Status:** TODO

### CLI-8 — Implement `music explain-diff`
- **Body:** Compare two analyses (e.g., melodies, chord charts, MIDI files) and surface keyed deltas such as mismatched pitch-classes or function counts.
- **Complexity:** 2
- **Value:** 74
- **Status:** TODO

### CLI-9 — Implement `music map`
- **Body:** Render textual interval/scale maps that show how pitch classes relate across chosen temperaments, optionally highlighting modulatory paths.
- **Complexity:** 2
- **Value:** 70
- **Status:** TODO

### CLI-10 — Implement `music profile`
- **Body:** Summarize timing, density, and register usage for long inputs, including percentile-based ranges and swing detection hooks.
- **Complexity:** 3
- **Value:** 77
- **Status:** TODO

### CLI-11 — Implement `music interpolate`
- **Body:** Provide envelope interpolation for tempo, velocity, or tuning curves between anchor points and preview the resulting automation lanes.
- **Complexity:** 3
- **Value:** 72
- **Status:** TODO

### CLI-12 — Implement `music search`
- **Body:** Allow motif/pattern searches inside MIDI or note lists via intervallic fingerprinting with fuzzy tolerance controls.
- **Complexity:** 3
- **Value:** 86
- **Status:** TODO

### CLI-13 — Implement `music estimate`
- **Body:** Estimate tempo, key, and meter from raw note sequences or MIDI headers using heuristics defined in `music-analysis`.
- **Complexity:** 2
- **Value:** 75
- **Status:** TODO

### CLI-14 — Implement `music resolve`
- **Body:** Suggest voice-leading resolutions for active chords, highlighting stepwise options per part and exposing target-system constraints.
- **Complexity:** 3
- **Value:** 79
- **Status:** TODO

### CLI-15 — Add golden tests for `analyze`
- **Body:** Capture sample CLI outputs (melody, chords, MIDI) under `crates/music-cli/tests` to lock text + JSON schemas against regressions.
- **Complexity:** 2
- **Value:** 73
- **Status:** TODO

### CLI-16 — Structured JSON schema export
- **Body:** Define and version JSON schemas for all CLI responses so downstream tools can validate outputs programmatically.
- **Complexity:** 2
- **Value:** 68
- **Status:** TODO

## Core & Theory Work

### CORE-1 — Finish interval module refactor
- **Body:** Complete the split across `crates/music-core/src/interval/**`, ensuring docs, prelude exports, and backward-compatible APIs are restored.
- **Complexity:** 3
- **Value:** 85
- **Status:** TODO

### CORE-2 — Chord quality taxonomy
- **Body:** Populate `crates/music-core/src/chord` with canonical qualities, inversions, and parsing helpers usable by analysis + CLI layers.
- **Complexity:** 3
- **Value:** 88
- **Status:** TODO

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
