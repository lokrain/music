# CLI Response Schemas

This directory hosts versioned JSON Schemas for `music-cli` response payloads.

## Versioning Strategy
* Each schema file: `<snake_case_report_name>.schema.json`
* A composite `index.json` enumerates all schema IDs.
* Bump the `schemaVersion` property when a backward-incompatible change occurs.

## Generation
Planned automated generation using `schemars` behind the optional `schema` feature.
Build script (`build.rs`) will emit schemas to this folder when feature enabled.

## Validation
Unit tests under `src/tests/schema_test.rs` will:
1. Load each schema file.
2. Ensure required top-level fields exist.
3. Optionally validate a sample instance.

## Reports Pending Schema
- MelodyAnalysisReport
- ChordAnalysisReport
- MelodyDiffReport
- ProgressionDiffReport
- MidiDiffReport
- ScaleMapReport
- InterpolatedEnvelopeReport
- VelocityEnvelopeReport
- ScaleSearchReport
- ChordSearchReport
- ProgressionScoreReport
- MelodyScoreReport
- ChordScoreReport
- StaffRenderReport
- PianoRollRenderReport
- MelodyValidationReport
- ProgressionValidationReport
- TuningValidationReport
- MidiAnalysisReport
- FrequencyToIndexReport
- TemperamentRemapReport
- MelodyExtrapolationReport
- ChordExtrapolationReport
- ProfileReport
- EstimateReport
- ChordResolutionReport

## Next Steps
Implement `schema` feature in `Cargo.toml` and generator module.
