//! Planning-specific request and response DTOs shared between the CLI and API layers.

use serde::{Deserialize, Serialize};

/// Request payload for running the harmonic planner via the API.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PlanRequest {
    /// Template selection strategy (built-in registry or inline DSL document).
    pub template: TemplateSelector,
    /// Desired key (tonic + mode) for the plan.
    pub key: KeySpecification,
    /// Style preset (optionally customized) guiding planner heuristics.
    pub style: StyleSpecification,
    /// Explainability capture mode requested for the run.
    pub explain: ExplainModeDto,
}

/// Planner response returned by the API and reused by the CLI JSON output.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PlanResponse {
    /// Template metadata for the emitted plan.
    pub template: TemplateDescriptor,
    /// Key (tonic + mode) applied during planning.
    pub key: KeySpecification,
    /// Style preset identifier applied for the plan.
    pub style: String,
    /// Explainability mode used for the explain summaries below.
    pub explain_mode: ExplainModeDto,
    /// Diagnostics emitted by the planner.
    pub diagnostics: Vec<String>,
    /// Raw planner states (per bar) emitted by the beam search.
    pub states: Vec<StateSnapshot>,
    /// Human-friendly per-bar summaries derived from explain data.
    pub bars: Vec<BarSummary>,
    /// Phrase summaries describing cadences and highlights.
    pub phrases: Vec<PhraseSummary>,
    /// Cadence-focused summaries for quick lookups.
    pub cadences: Vec<CadenceSummary>,
}

/// Selector describing how the planner should resolve a template.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum TemplateSelector {
    /// Reference a built-in template bundled with the workspace.
    Builtin {
        /// Identifier of the built-in template.
        id: String,
    },
    /// Reference a user template stored in the registry by ID.
    Registry {
        /// Identifier of the stored template.
        id: String,
    },
    /// Provide an inline DSL document (+ format) instead of referencing a registry entry.
    Inline {
        /// DSL format of the inline template payload.
        format: TemplateFormatDto,
        /// Raw DSL payload encoded as a UTF-8 string.
        data: String,
    },
}

/// Descriptor for the template materialized in the response.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TemplateDescriptor {
    /// Template identifier (built-in or registry ID).
    pub id: String,
    /// Template version as defined by the DSL metadata.
    pub version: u16,
    /// Number of bars contained by the template.
    pub bars: u16,
    /// Descriptor describing how the template was resolved.
    pub source: TemplateSourceDescriptor,
}

/// Source descriptor for templates referenced by responses.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TemplateSourceDescriptor {
    /// Optional built-in identifier.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub builtin: Option<String>,
    /// Optional registry identifier (distinct from built-ins when stored server-side).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub registry_id: Option<String>,
    /// Optional path for templates loaded from disk by the CLI.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}

/// Representation of a musical key in the API request/response types.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct KeySpecification {
    /// Pitch-class tonic label (e.g., "C", "F#", "Bb").
    pub tonic: String,
    /// Requested mode (major/minor for v1).
    pub mode: ModeDto,
}

/// Style preset descriptor plus optional field-level overrides.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StyleSpecification {
    /// Preset identifier (e.g., "balanced", "gospel_drive").
    pub preset: String,
    /// Optional overrides for select numeric knobs.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub overrides: Option<StyleOverrides>,
}

/// Optional overrides for style presets, allowing partial updates without breaking compatibility.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct StyleOverrides {
    /// Override the beam width (search breadth).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub beam_width: Option<u16>,
    /// Override planner max depth.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_depth: Option<u16>,
    /// Override global risk level.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub risk_level: Option<f32>,
    /// Override reharmonization search depth.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reharm_depth: Option<f32>,
    /// Override voice-leading strictness.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub voice_leading_strictness: Option<f32>,
    /// Override modulation aggressiveness.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub modulation_aggressiveness: Option<f32>,
    /// Override allowed chord complexity.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_chord_complexity: Option<f32>,
}

/// Planner state snapshot mirrored from the internal beam search state.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StateSnapshot {
    /// 1-indexed bar position.
    pub bar: u16,
    /// Scale degree selected for the bar.
    pub scale_degree: u8,
    /// Harmonic function label.
    pub function: String,
    /// Chord symbol for the state.
    pub chord: String,
    /// Planner tension reading for the state.
    pub tension: f32,
    /// Cadence status summary.
    pub cadence: String,
    /// Optional phrase label for the state.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phrase: Option<String>,
}

/// Per-bar explainability summary derived from captured events.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BarSummary {
    /// 1-indexed bar number.
    pub bar: u16,
    /// Optional phrase label that contains the bar.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phrase: Option<String>,
    /// Harmonic function label.
    pub function: String,
    /// Chord label.
    pub chord: String,
    /// Target tension from the template.
    pub tension_target: f32,
    /// Actual achieved tension.
    pub tension_actual: f32,
    /// Per-bar reharmonization risk.
    pub reharm_risk: f32,
    /// Cadence information.
    pub cadence: String,
    /// Highlight notes or commentary captured for the bar.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub notes: Vec<String>,
}

/// Phrase-level summary describing cadences and highlights inside the phrase.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PhraseSummary {
    /// Human-readable phrase label.
    pub phrase: String,
    /// 1-indexed start bar.
    pub start_bar: u16,
    /// 1-indexed end bar.
    pub end_bar: u16,
    /// Cadence labels encountered inside the phrase.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub cadences: Vec<String>,
    /// Key highlights (e.g., rule firings) captured for the phrase.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub highlights: Vec<String>,
}

/// Cadence-specific summary for quick inspection.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CadenceSummary {
    /// 1-indexed bar number where the cadence occurs.
    pub bar: u16,
    /// Optional phrase label containing the cadence.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phrase: Option<String>,
    /// Cadence label (half/authentic/etc.).
    pub cadence: String,
    /// Optionally expose the expected function for debugging.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expected_function: Option<String>,
}

/// Template DSL encoding supported by the inline selector variant.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum TemplateFormatDto {
    /// JSON5 DSL file.
    Json5,
    /// RON DSL file.
    Ron,
}

/// Mode descriptor for API payloads.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ModeDto {
    /// Major key mode.
    Major,
    /// Minor key mode.
    Minor,
}

/// Explainability capture modes exposed by the API.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ExplainModeDto {
    /// Disable explainability completely.
    None,
    /// Capture lightweight explain summaries.
    Brief,
    /// Capture detailed per-bar notes and candidate info.
    Detailed,
    /// Capture debug-level traces (heavyweight).
    Debug,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{Value, json};

    #[test]
    fn plan_request_round_trips() {
        let request = PlanRequest {
            template: TemplateSelector::Builtin { id: "jazz_aaba_v1".into() },
            key: KeySpecification { tonic: "C".into(), mode: ModeDto::Major },
            style: StyleSpecification {
                preset: "balanced".into(),
                overrides: Some(StyleOverrides {
                    beam_width: Some(8),
                    max_depth: Some(18),
                    risk_level: Some(0.6),
                    reharm_depth: None,
                    voice_leading_strictness: None,
                    modulation_aggressiveness: Some(0.5),
                    max_chord_complexity: Some(0.7),
                }),
            },
            explain: ExplainModeDto::Detailed,
        };
        let json = serde_json::to_string(&request).expect("serialize request");
        let decoded: PlanRequest = serde_json::from_str(&json).expect("deserialize request");
        assert_eq!(decoded, request);
    }

    #[test]
    fn plan_response_round_trips() {
        let response = PlanResponse {
            template: TemplateDescriptor {
                id: "jazz_aaba_v1".into(),
                version: 1,
                bars: 32,
                source: TemplateSourceDescriptor {
                    builtin: Some("jazz_aaba_v1".into()),
                    registry_id: None,
                    path: None,
                },
            },
            key: KeySpecification { tonic: "C".into(), mode: ModeDto::Major },
            style: "balanced".into(),
            explain_mode: ExplainModeDto::Brief,
            diagnostics: vec!["tension fallback".into()],
            states: vec![StateSnapshot {
                bar: 1,
                scale_degree: 1,
                function: "Tonic".into(),
                chord: "Cmaj7".into(),
                tension: 0.1,
                cadence: "none".into(),
                phrase: Some("A1".into()),
            }],
            bars: vec![BarSummary {
                bar: 1,
                phrase: Some("A1".into()),
                function: "Tonic".into(),
                chord: "Cmaj7".into(),
                tension_target: 0.1,
                tension_actual: 0.12,
                reharm_risk: 0.2,
                cadence: "none".into(),
                notes: vec!["expected_function:tonic".into()],
            }],
            phrases: vec![PhraseSummary {
                phrase: "A1".into(),
                start_bar: 1,
                end_bar: 8,
                cadences: vec!["half".into()],
                highlights: vec!["modulation:IV".into()],
            }],
            cadences: vec![CadenceSummary {
                bar: 8,
                phrase: Some("A1".into()),
                cadence: "half".into(),
                expected_function: Some("Dominant".into()),
            }],
        };
        let value: Value = serde_json::to_value(&response).expect("serialize response");
        assert_eq!(value["template"]["id"], json!("jazz_aaba_v1"));
        let decoded: PlanResponse = serde_json::from_value(value).expect("deserialize response");
        assert_eq!(decoded, response);
    }
}
