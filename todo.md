 
### T1.1 — Chord/Tension Taxonomy
- **Status:** IN_PROGRESS
- **Complexity:** 2
- **Value:** 95
- **Description:** Introduce canonical chord model (ChordQuality, Extension bitflags, Alteration enum, UpperStructureTriad, ChordSpec) in music-core.
- **Design Notes:** Foundation for voicing registry & passing-chord engine; must avoid root duplication logic; ensure interval mapping deterministic.

### T1.2 — Chord Parser & Canonicalizer
- **Status:** TODO
- **Complexity:** 2
- **Value:** 90
- **Description:** Parse string forms (e.g. Cmaj9, G7#9, Dmin11, F#7alt) into ChordSpec; normalize extension order; error enum.

### T1.3 — Upper-Structure Triad Mapping
- **Status:** TODO
- **Complexity:** 1
- **Value:** 80
- **Description:** Provide function listing available USTs for a given dominant/major/minor spec (e.g. D maj over Cmaj9 yields tensions {9,#11,13}).

### T1.4 — Interval Generation Tests
- **Status:** TODO
- **Complexity:** 1
- **Value:** 75
- **Description:** Round-trip tests: spec→intervals→reconstruct quality/extensions; property tests for no duplicates/ordering.

### T1.5 — Chord Error Enum
- **Status:** TODO
- **Complexity:** 1
- **Value:** 70
- **Description:** Define ChordError (Parse, DuplicateExtension, InvalidAlterationCombo, UnsupportedQuality) with Display + serde.

### T16.3 — Template management CLI/API
- **Complexity:** 2  
- **Value:** 88  
- **Description:** Add CLI/API endpoints for managing templates: list, show, import from file, export to file. Ensure error messages are clear when templates are invalid or missing.

### T16.4 — Explainability view in CLI
- **Complexity:** 2  
- **Value:** 85  
- **Description:** Provide a CLI “explain” mode that formats `ExplainEvents` into readable bar-by-bar narratives (showing rule names, tension, modulation decisions).

### T16.5 — End-to-end integration tests (CLI/API)
- **Complexity:** 3  
- **Value:** 92  
- **Description:** Add end-to-end tests that drive the CLI/API: load a template, run a plan, inspect JSON output, and verify invariants on chords/functions/keys and explanations.
