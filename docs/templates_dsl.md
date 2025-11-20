# Section Template DSL

## Goals
- Describe 4–64 bar sections, phrases, cadences, modulation hints, tension curves, and reharmonization ranges.
- Encode as JSON5/RON friendly schema so templates live in version control and round-trip through the parser.
- Stay aligned with planner invariants: contiguous phrases, bar-level tension targets, and explicit metadata for explainability.

## Schema Reference

| Field | Type | Required | Constraints |
| --- | --- | --- | --- |
| `id` | `string` | ✅ | Unique identifier per template family (e.g., `jazz_aaba_v1`). |
| `version` | `u16` | ⛔ (defaults to `1`) | Increment when breaking planner-facing semantics. |
| `bars` | `u16` | ✅ | Range 4–64 bars; must equal the sum of phrase lengths. |
| `phrases` | `Phrase[]` | ✅ | Sorted, gap-free coverage of `[0, bars)`.
| `tension_curve` | `f32[]` | ✅ | Length == `bars`; each value in `[0.0, 1.0]`.
| `reharm_zones` | `ReharmZone[]` | ⛔ | Optional overlapping hints; planner takes the max risk per bar.

### Phrase
| Field | Type | Required | Notes |
| --- | --- | --- | --- |
| `name` | `string` | ✅ | Human-friendly label for explainability/registry output. |
| `start_bar` | `u16` | ✅ | Must equal the running cursor before inserting the phrase. |
| `length` | `u8` | ✅ | `> 0`; `start_bar + length <= bars`. |
| `cadence` | `string` | ⛔ | One of `none`, `half`, `perfect`, `plagal`, `deceptive`. |
| `modulation_hint` | `string` | ⛔ | Free-form token (e.g., `IV`, `bIII`, `lydian`).

### ReharmZone
| Field | Type | Required | Notes |
| --- | --- | --- | --- |
| `start_bar` | `u16` | ✅ | Inclusive start. |
| `end_bar` | `u16` | ✅ | Exclusive end, `> start_bar`, `<= bars`. |
| `risk` | `f32` | ⛔ (defaults `0.5`) | Clamped to `[0.0, 1.0]`; influences planner reharm weighting.

## Enumerations & Semantics

### Cadence Labels
- `none`: Phrase interior; planner allows any functional class.
- `half`: Expect dominant function, often setting up continuation.
- `perfect`: Expect tonic arrival; planner boosts tonic transitions.
- `plagal`: Expect subdominant resolving to tonic coloration.
- `deceptive`: Expect dominant → submediant motion.

### Modulation Hints
Hints are optional strings interpreted by higher layers (analysis/planner). Recommended tokens:
- Scale-degree moves (`IV`, `ii`, `bIII`).
- Mode nudges (`lydian`, `mixolydian`).
- Descriptive tags (`turnaround`, `backdoor`).

### Reharm Zones
- Multiple zones can overlap; the planner uses the maximum risk covering a bar.
- Risk ≈ expected willingness to entertain non-cadential substitutions. Higher risk softens penalties when bar-rule expectations disagree.

## Validation Checklist
1. `bars > 0` and ≤ 64.
2. `tension_curve.len() == bars` and every entry is within `[0.0, 1.0]`.
3. `phrases` are non-empty, strictly cover `[0, bars)`, and contain no zero-length entries.
4. Cadence labels map to the supported set; unknown strings fail validation.
5. Reharm zones satisfy `start_bar < end_bar <= bars` and valid risk range.
6. After compilation, bar rules exist for every bar with contiguous indices (internal invariant checked by the planner).

## Examples

### 32-Bar Jazz AABA (JSON5)
```json5
{
  id: "jazz_aaba_v1",
  version: 1,
  bars: 32,
  phrases: [
    { name: "A1", start_bar: 0, length: 8, cadence: "half" },
    { name: "A2", start_bar: 8, length: 8, cadence: "half" },
    { name: "B", start_bar: 16, length: 8, cadence: "none", modulation_hint: "IV" },
    { name: "A3", start_bar: 24, length: 8, cadence: "perfect" }
  ],
  tension_curve: [
    0.10, 0.12, 0.14, 0.16, 0.18, 0.22, 0.26, 0.30,
    0.18, 0.20, 0.22, 0.24, 0.28, 0.32, 0.36, 0.40,
    0.30, 0.32, 0.34, 0.36, 0.40, 0.44, 0.48, 0.38,
    0.24, 0.22, 0.20, 0.18, 0.16, 0.14, 0.12, 0.10
  ],
  reharm_zones: [
    { start_bar: 4, end_bar: 8, risk: 0.6 },
    { start_bar: 16, end_bar: 20, risk: 0.8 }
  ]
}
```

### 12-Bar Blues (RON)
```ron
(
  id: "blues_12bar_v1",
  bars: 12,
  phrases: [
    (name: "A", start_bar: 0, length: 4, cadence: "none"),
    (name: "A'", start_bar: 4, length: 4, cadence: "none"),
    (name: "B", start_bar: 8, length: 4, cadence: "perfect", modulation_hint: "V"),
  ],
  tension_curve: [0.15, 0.2, 0.25, 0.3, 0.18, 0.22, 0.28, 0.32, 0.26, 0.3, 0.34, 0.2],
  reharm_zones: [(start_bar: 8, end_bar: 12, risk: 0.9)],
)
```

### 16-Bar Pop Groove (JSON5)
```json5
{
  id: "pop16_hook_v1",
  version: 2,
  bars: 16,
  phrases: [
    { name: "Intro", start_bar: 0, length: 4, cadence: "none" },
    { name: "Verse", start_bar: 4, length: 8, cadence: "half", modulation_hint: "bVII" },
    { name: "Hook", start_bar: 12, length: 4, cadence: "perfect" }
  ],
  tension_curve: [
    0.05, 0.08, 0.10, 0.12,
    0.20, 0.22, 0.25, 0.28,
    0.32, 0.34, 0.36, 0.30,
    0.26, 0.20, 0.14, 0.10
  ],
  reharm_zones: [
    { start_bar: 6, end_bar: 10, risk: 0.5 },
    { start_bar: 12, end_bar: 16, risk: 0.3 }
  ]
}
```

## Built-in Template Library

The planner ships with a small curated set of templates available via
`music_score::builtin_template_ids()` / `builtin_template()`, or pre-loaded through
`builtin_registry()`:

| ID | Bars | Style | Notes |
| --- | --- | --- | --- |
| `jazz_aaba_v1` | 32 | Jazz AABA | Textbook song-form with cadential A sections and IV bridge hint. |
| `blues_12bar_v1` | 12 | Blues | Classic 12-bar form with high reharm appetite over bars 9–12. |
| `pop16_hook_v1` | 16 | Pop / Neo-soul | Intro/verse/hook layout with bVII modulation hint. |
| `gospel_vamp_v1` | 8 | Gospel Vamp | Call/response phrase pair with aggressive reharm zone in the middle. |

These templates follow the schema defined above and can serve as references when crafting new DSL files or seeding registries for CLI/API layers.

## Versioning & Stability Policy

Template stability matters because CLI/API consumers may persist template IDs inside sessions, presets, or downstream arrangements. We follow these rules:

1. **Semantic IDs:** The `id` field encodes the musical form (`jazz_aaba`, `blues_12bar`) plus the canonical version suffix (`_v1`, `_v2`, …). The suffix only changes when backwards-incompatible semantics ship.
2. **`version` field:** Monotonically increasing `u16` that tracks revisions *within* a given template ID. Increment the value whenever constraints, tension curves, or metadata change—even when backward compatible—so registries can detect updates.
3. **Non-breaking changes:** Adjustments that keep phrase/bar layout identical (e.g., minor tension tweaks, documentation improvements) stay on the same `id` and merely bump `version`.
4. **Breaking changes:** Any alteration that changes `bars`, phrase ordering/lengths, cadence expectations, or reharm constraints requires minting a brand-new ID (e.g., `jazz_aaba_v2`). Keep the previous ID available for existing consumers.
5. **Registry compatibility:** Registries should treat `(id, version)` as the stable key. Re-registering a newer version replaces the prior struct but logs the displacement; a different ID gets stored alongside existing entries.
6. **Deprecation:** Mark deprecated templates in documentation and registries via metadata (future work). Never silently remove an ID without a migration plan.

When contributing templates, document the change rationale and update this file's Built-in table (or auxiliary docs) so downstream integrations know which IDs map to which musical forms.
