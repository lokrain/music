#![allow(dead_code)]

use std::fmt::Write as FmtWrite;

use chord::DiatonicChord;
use music_engine::prelude::{Pitch, PitchError, Scale, TuningRegistry, chord};
// Re-export modularized report types
pub use crate::reports::analysis::*;
pub use crate::reports::chord::*;
#[cfg(feature = "schema")]
pub use crate::reports::diff::*;
pub use crate::reports::generation::*;
pub use crate::reports::pitch::*;
pub use crate::reports::resolution::*;
pub use crate::reports::scale::*;
#[cfg(feature = "schema")]
use schemars::JsonSchema;
use serde::Serialize;

pub type PitchResult<T> = Result<T, PitchError>;

// Pitch-related types have moved to reports/pitch.rs

// Chord-related report types moved to reports/chord.rs

// Generation-related report types moved to reports/generation.rs

#[derive(Debug, Serialize, Clone)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct InterpolatedPoint {
    pub time: f32,
    pub value: f32,
}

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct InterpolationContext {
    pub curve: String,
    pub samples: usize,
}

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct InterpolatedEnvelopeReport {
    pub context: InterpolationContext,
    pub unit: String,
    pub anchors: Vec<InterpolatedPoint>,
    pub samples: Vec<InterpolatedPoint>,
}

impl InterpolatedEnvelopeReport {
    pub fn render_text(&self) -> String {
        let mut out = String::new();
        let _ = writeln!(
            &mut out,
            "Interpolation ({unit}) — curve {curve}, {count} anchors, {samples} samples.",
            unit = self.unit,
            curve = self.context.curve,
            count = self.anchors.len(),
            samples = self.samples.len()
        );
        let _ = writeln!(&mut out, "Anchors:");
        for anchor in &self.anchors {
            let _ = writeln!(
                &mut out,
                "  t={time:>5.2} → {value:>7.3}",
                time = anchor.time,
                value = anchor.value
            );
        }
        let _ = writeln!(&mut out, "Samples:");
        for point in &self.samples {
            let _ = writeln!(
                &mut out,
                "  t={time:>5.2} → {value:>7.3}",
                time = point.time,
                value = point.value
            );
        }
        out
    }
}

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct VelocityEnvelopeReport {
    pub context: InterpolationContext,
    pub anchors: Vec<InterpolatedPoint>,
    pub samples: Vec<InterpolatedPoint>,
    pub min_value: i32,
    pub max_value: i32,
}

impl VelocityEnvelopeReport {
    pub fn render_text(&self) -> String {
        let mut out = String::new();
        let _ = writeln!(
            &mut out,
            "Velocity interpolation [{min}..{max}] — curve {curve}, {count} anchors.",
            min = self.min_value,
            max = self.max_value,
            curve = self.context.curve,
            count = self.anchors.len()
        );
        let _ = writeln!(&mut out, "Anchors:");
        for anchor in &self.anchors {
            let _ = writeln!(
                &mut out,
                "  t={time:>5.2} → {value:>6.1}",
                time = anchor.time,
                value = anchor.value
            );
        }
        let _ = writeln!(&mut out, "Samples:");
        for point in &self.samples {
            let _ = writeln!(
                &mut out,
                "  t={time:>5.2} → {value:>6.1}",
                time = point.time,
                value = point.value
            );
        }
        out
    }
}

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct ChordSearchReport {
    pub system: String,
    pub criteria: Vec<u8>,
    pub voicing: String,
    pub match_count: usize,
    pub matches: Vec<ChordSearchMatch>,
}

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct ChordSearchMatch {
    pub scale: String,
    pub degree: usize,
    pub numeral: String,
    pub root_label: String,
    pub pitch_classes: Vec<u8>,
}

impl ChordSearchReport {
    pub fn render_text(&self) -> String {
        let mut out = String::new();
        let _ = writeln!(
            &mut out,
            "Chord search ({voicing}) in {system}: {count} match(es) for pcs {pcs:?}.",
            voicing = self.voicing,
            system = self.system,
            count = self.match_count,
            pcs = self.criteria
        );
        for entry in &self.matches {
            let _ = writeln!(
                &mut out,
                "  - {scale} degree {degree} ({numeral}) root {root}, pcs {pcs:?}.",
                scale = entry.scale,
                degree = entry.degree,
                numeral = entry.numeral,
                root = entry.root_label,
                pcs = entry.pitch_classes
            );
        }
        out
    }
}

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct ReharmListing {
    pub system: String,
    pub root_index: i32,
    pub scale: String,
    pub voicing: String,
    pub target_degree: Option<usize>,
    pub target_label: Option<String>,
    pub target_frequency_hz: Option<f32>,
    pub mode_count: usize,
    pub modes: Vec<ModeReharm>,
}

impl ReharmListing {
    pub fn render_text(&self) -> String {
        let mut out = String::new();
        let _ = writeln!(
            &mut out,
            "Parallel-mode reharmonization for the {scale} scale (root {root}) in {system} — {voicing}.",
            scale = self.scale,
            root = self.root_index,
            system = self.system,
            voicing = self.voicing
        );
        if let Some(degree) = self.target_degree {
            let label = self.target_label.as_deref().unwrap_or("—");
            let freq = self
                .target_frequency_hz
                .map(|value| format!("{value:.3} Hz"))
                .unwrap_or_else(|| "unknown".into());
            let _ = writeln!(
                &mut out,
                "Target degree {degree}: {label} ({freq}).",
                degree = degree,
                label = label,
                freq = freq
            );
        }
        let _ = writeln!(&mut out);

        for mode in &self.modes {
            let title = mode
                .mode_name
                .as_deref()
                .map(str::to_string)
                .unwrap_or_else(|| format!("Mode {}", mode.mode_index));
            let root_label = mode.root_label.as_deref().unwrap_or("—");
            let _ = writeln!(
                &mut out,
                "{idx:>2}. {title} (rotation {rotation}, root {root_label})",
                idx = mode.mode_index,
                title = title,
                rotation = mode.rotation_degree + 1,
                root_label = root_label
            );
            if mode.borrowed_chords.is_empty() {
                let _ = writeln!(&mut out, "    (no matching chords)");
            } else {
                for chord in &mode.borrowed_chords {
                    let marker = if chord.matches_target { "*" } else { " " };
                    let root_label = chord.root_label.as_deref().unwrap_or("—");
                    let quality = chord.details.quality.as_deref().unwrap_or("Unclassified");
                    let _ = writeln!(
                        &mut out,
                        "   {marker} {numeral:<4} (degree {degree}) — {quality}, root {root_label}",
                        marker = marker,
                        numeral = chord.details.numeral,
                        degree = chord.details.degree + 1,
                        quality = quality,
                        root_label = root_label
                    );
                }
            }
            let _ = writeln!(&mut out);
        }

        out
    }
}

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct ModeReharm {
    pub mode_index: usize,
    pub rotation_degree: usize,
    pub mode_name: Option<String>,
    pub root_index: Option<i32>,
    pub root_label: Option<String>,
    pub borrowed_chords: Vec<BorrowedChord>,
}

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct BorrowedChord {
    #[serde(flatten)]
    pub details: ChordDetails,
    pub root_index: Option<i32>,
    pub root_label: Option<String>,
    pub root_frequency_hz: Option<f32>,
    pub matches_target: bool,
}

// validation reports moved to reports/validation.rs
#[derive(Debug, Serialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct MelodyScoreReport {
    pub note_count: usize,
    pub ambitus: Ambitus,
    pub leap_ratio: f32,
    pub stepwise_ratio: f32,
    pub direction_changes: usize,
    pub closure_interval: i32,
    pub range_score: f32,
    pub motion_score: f32,
    pub contour_score: f32,
    pub total_score: f32,
    pub commentary: Vec<String>,
    pub key_hint: Option<String>,
}

impl MelodyScoreReport {
    pub fn render_text(&self) -> String {
        let mut out = String::new();
        let _ = writeln!(
            &mut out,
            "Melody score: {total:.1}/100 across {count} notes (ambitus {span} st).",
            total = self.total_score,
            count = self.note_count,
            span = self.ambitus.span
        );
        let _ = writeln!(
            &mut out,
            "Stepwise {step:.0}% · Leaps {leap:.0}% · Direction changes {changes}.",
            step = self.stepwise_ratio * 100.0,
            leap = self.leap_ratio * 100.0,
            changes = self.direction_changes
        );
        let _ = writeln!(
            &mut out,
            "Range {range:.0}% · Motion {motion:.0}% · Contour {contour:.0}% (closure Δ {closure:+}).",
            range = self.range_score * 100.0,
            motion = self.motion_score * 100.0,
            contour = self.contour_score * 100.0,
            closure = self.closure_interval
        );
        if let Some(key) = &self.key_hint {
            let _ = writeln!(&mut out, "Context key hint: {key}.");
        }
        if !self.commentary.is_empty() {
            let _ = writeln!(&mut out);
            for note in &self.commentary {
                let _ = writeln!(&mut out, "- {note}");
            }
        }
        out
    }
}

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct ProgressionScoreReport {
    pub progression: Vec<String>,
    pub total_chords: usize,
    pub unique_chords: usize,
    pub function_counts: FunctionCounts,
    pub cadence: Option<crate::reports::analysis::CadenceSummary>,
    pub coverage_score: f32,
    pub cadence_score: f32,
    pub variety_score: f32,
    pub total_score: f32,
    pub commentary: Vec<String>,
    pub key_hint: Option<String>,
}

impl ProgressionScoreReport {
    pub fn render_text(&self) -> String {
        let mut out = String::new();
        let _ = writeln!(
            &mut out,
            "Progression score: {total:.1}/100 ({unique} unique / {count} total).",
            total = self.total_score,
            unique = self.unique_chords,
            count = self.total_chords
        );
        let _ = writeln!(
            &mut out,
            "Function balance — tonic {tonic}, predominant {pred}, dominant {dom}, other {other}.",
            tonic = self.function_counts.tonic,
            pred = self.function_counts.predominant,
            dom = self.function_counts.dominant,
            other = self.function_counts.other
        );
        let _ = writeln!(
            &mut out,
            "Coverage {cov:.0}% · Cadence {cad:.0}% · Variety {var:.0}%.",
            cov = self.coverage_score * 100.0,
            cad = self.cadence_score * 100.0,
            var = self.variety_score * 100.0
        );
        if let Some(cad) = &self.cadence {
            let _ = writeln!(
                &mut out,
                "Cadence detected: {pattern} ({desc}, {conf:.0}%).",
                pattern = cad.pattern,
                desc = cad.description,
                conf = cad.confidence * 100.0
            );
        } else {
            let _ = writeln!(&mut out, "Cadence detected: none.");
        }
        if let Some(key) = &self.key_hint {
            let _ = writeln!(&mut out, "Context key hint: {key}.");
        }
        let _ = writeln!(&mut out);
        if !self.commentary.is_empty() {
            for note in &self.commentary {
                let _ = writeln!(&mut out, "- {note}");
            }
        }
        out
    }
}

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct ChordScoreReport {
    pub system: String,
    pub note_count: usize,
    pub pitch_span: i32,
    pub unique_pitch_classes: usize,
    pub extensions: usize,
    pub color_score: f32,
    pub stability_score: f32,
    pub tension_index: f32,
    pub total_score: f32,
    pub note_labels: Vec<String>,
    pub commentary: Vec<String>,
}

impl ChordScoreReport {
    pub fn render_text(&self) -> String {
        let mut out = String::new();
        let _ = writeln!(
            &mut out,
            "Chord score: {total:.1}/100 — span {span} st, {classes} pitch classes, {ext} extension(s).",
            total = self.total_score,
            span = self.pitch_span,
            classes = self.unique_pitch_classes,
            ext = self.extensions
        );
        let _ = writeln!(
            &mut out,
            "Color {color:.0}% · Stability {stab:.0}% · Tension index {tension:.0}%.",
            color = self.color_score * 100.0,
            stab = self.stability_score * 100.0,
            tension = self.tension_index * 100.0
        );
        if !self.note_labels.is_empty() {
            let _ = writeln!(
                &mut out,
                "Notes ({system}): {labels}.",
                system = self.system,
                labels = self.note_labels.join(", ")
            );
        }
        if !self.commentary.is_empty() {
            let _ = writeln!(&mut out);
            for note in &self.commentary {
                let _ = writeln!(&mut out, "- {note}");
            }
        }
        out
    }
}

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct StaffRenderReport {
    pub system: String,
    pub unicode: bool,
    pub key_hint: Option<String>,
    pub note_count: usize,
    pub min_index: i32,
    pub max_index: i32,
    pub rows: Vec<String>,
    pub note_labels: Vec<String>,
}

impl StaffRenderReport {
    pub fn render_text(&self) -> String {
        let mut out = String::new();
        let mode = if self.unicode { "Unicode" } else { "ASCII" };
        let _ = writeln!(
            &mut out,
            "Staff render ({mode}) for {count} note(s) in {system} [{min}..{max}].",
            mode = mode,
            count = self.note_count,
            system = self.system,
            min = self.min_index,
            max = self.max_index
        );
        if let Some(key) = &self.key_hint {
            let _ = writeln!(&mut out, "Context key: {key}.");
        }
        for row in &self.rows {
            let _ = writeln!(&mut out, "{row}");
        }
        if !self.note_labels.is_empty() {
            let _ = writeln!(&mut out, "Notes:");
            for label in &self.note_labels {
                let _ = writeln!(&mut out, "  • {label}");
            }
        }
        out
    }
}

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct PianoRollRenderReport {
    pub system: String,
    pub width: usize,
    pub height: usize,
    pub note_count: usize,
    pub min_index: i32,
    pub max_index: i32,
    pub rows: Vec<String>,
    pub note_labels: Vec<String>,
}

impl PianoRollRenderReport {
    pub fn render_text(&self) -> String {
        let mut out = String::new();
        let _ = writeln!(
            &mut out,
            "Piano-roll render ({width}×{height}) for {count} note(s) in {system} [{min}..{max}].",
            width = self.width,
            height = self.height,
            count = self.note_count,
            system = self.system,
            min = self.min_index,
            max = self.max_index
        );
        for row in &self.rows {
            let _ = writeln!(&mut out, "{row}");
        }
        if !self.note_labels.is_empty() {
            let _ = writeln!(&mut out, "Notes:");
            for label in &self.note_labels {
                let _ = writeln!(&mut out, "  • {label}");
            }
        }
        out
    }
}

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct MelodyValidationReport {
    pub system: String,
    pub scale: String,
    pub root_index: i32,
    pub note_count: usize,
    pub ambitus: Ambitus,
    pub max_interval: i32,
    pub allowed_pitch_classes: Vec<u8>,
    pub out_of_scale_notes: Vec<InvalidMelodyNote>,
    pub leap_violations: Vec<LeapViolation>,
}

impl MelodyValidationReport {
    pub fn render_text(&self) -> String {
        let mut out = String::new();
        let _ = writeln!(
            &mut out,
            "Melody validation in {system} — {scale} scale (root {root}, {count} note(s)).",
            system = self.system,
            scale = self.scale,
            root = self.root_index,
            count = self.note_count
        );
        let _ = writeln!(
            &mut out,
            "Ambitus: {span} semitone(s) [{lowest} → {highest}]. Max interval allowed: ±{limit}.",
            span = self.ambitus.span,
            lowest = self.ambitus.lowest,
            highest = self.ambitus.highest,
            limit = self.max_interval
        );
        let _ = writeln!(
            &mut out,
            "Allowed pitch classes (relative to root): {:?}",
            self.allowed_pitch_classes
        );
        if self.out_of_scale_notes.is_empty() && self.leap_violations.is_empty() {
            let _ = writeln!(&mut out, "No validation issues detected.");
            return out;
        }
        if !self.out_of_scale_notes.is_empty() {
            let _ = writeln!(
                &mut out,
                "{count} note(s) fall outside the implied scale:",
                count = self.out_of_scale_notes.len()
            );
            for note in &self.out_of_scale_notes {
                let _ = writeln!(
                    &mut out,
                    "  • idx {idx} (pc {pc}) at position {pos}",
                    idx = note.note_index,
                    pc = note.pitch_class,
                    pos = note.position + 1
                );
            }
        }
        if !self.leap_violations.is_empty() {
            let _ = writeln!(
                &mut out,
                "{count} leap violation(s) exceed ±{limit} semitone(s):",
                count = self.leap_violations.len(),
                limit = self.max_interval
            );
            for violation in &self.leap_violations {
                let _ = writeln!(
                    &mut out,
                    "  • pos {pos}: {from} → {to} ({interval:+} st)",
                    pos = violation.position + 1,
                    from = violation.from,
                    to = violation.to,
                    interval = violation.interval
                );
            }
        }
        out
    }
}

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct InvalidMelodyNote {
    pub position: usize,
    pub note_index: i32,
    pub pitch_class: u8,
}

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct LeapViolation {
    pub position: usize,
    pub from: i32,
    pub to: i32,
    pub interval: i32,
}

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct ProgressionValidationReport {
    pub progression: Vec<String>,
    pub chord_count: usize,
    pub key_hint: Option<String>,
    pub invalid_chords: Vec<InvalidProgressionToken>,
    pub duplicate_positions: Vec<usize>,
    pub function_counts: FunctionCounts,
    pub cadence: Option<crate::reports::analysis::CadenceSummary>,
}

impl ProgressionValidationReport {
    pub fn render_text(&self) -> String {
        let mut out = String::new();
        let _ = writeln!(
            &mut out,
            "Progression validation ({count} chord(s)).",
            count = self.chord_count
        );
        if let Some(key) = &self.key_hint {
            let _ = writeln!(&mut out, "Key hint: {key}.");
        }
        if !self.invalid_chords.is_empty() {
            let _ = writeln!(
                &mut out,
                "{count} invalid chord token(s) found:",
                count = self.invalid_chords.len()
            );
            for chord in &self.invalid_chords {
                let _ = writeln!(
                    &mut out,
                    "  • pos {pos}: '{token}' (normalized '{norm}')",
                    pos = chord.position + 1,
                    token = chord.token,
                    norm = chord.normalized
                );
            }
        } else {
            let _ = writeln!(&mut out, "All chord tokens parse as valid Roman numerals.");
        }
        if !self.duplicate_positions.is_empty() {
            let _ = writeln!(
                &mut out,
                "Adjacent duplicates at positions: {:?}.",
                self.duplicate_positions
                    .iter()
                    .map(|pos| pos + 1)
                    .collect::<Vec<_>>()
            );
        }
        let _ = writeln!(
            &mut out,
            "Function counts — tonic {tonic}, predominant {pred}, dominant {dom}, other {other}.",
            tonic = self.function_counts.tonic,
            pred = self.function_counts.predominant,
            dom = self.function_counts.dominant,
            other = self.function_counts.other
        );
        if let Some(cadence) = &self.cadence {
            let _ = writeln!(
                &mut out,
                "Detected cadence {pattern} ({desc}, confidence {conf:.0}%).",
                pattern = cadence.pattern,
                desc = cadence.description,
                conf = cadence.confidence * 100.0
            );
        } else {
            let _ = writeln!(
                &mut out,
                "No terminal cadence detected (progression ends on {}).",
                self.progression.last().map(String::as_str).unwrap_or("—")
            );
        }
        out
    }
}

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct InvalidProgressionToken {
    pub position: usize,
    pub token: String,
    pub normalized: String,
}

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct TuningValidationReport {
    pub system: String,
    pub requested_indices: Vec<i32>,
    pub resolved_samples: Vec<TuningSampleRow>,
    pub failed_indices: Vec<i32>,
    pub monotonic_violations: Vec<MonotonicViolation>,
}

impl TuningValidationReport {
    pub fn render_text(&self) -> String {
        let mut out = String::new();
        let _ = writeln!(
            &mut out,
            "Tuning validation for {system}: {resolved}/{requested} indices resolved.",
            system = self.system,
            resolved = self.resolved_samples.len(),
            requested = self.requested_indices.len()
        );
        if !self.failed_indices.is_empty() {
            let _ = writeln!(&mut out, "Failed indices: {:?}.", self.failed_indices);
        }
        if !self.monotonic_violations.is_empty() {
            let _ = writeln!(&mut out, "Monotonic violations detected:");
            for violation in &self.monotonic_violations {
                let _ = writeln!(
                    &mut out,
                    "  • {low_idx} ({low_hz:.3} Hz) ≥ {high_idx} ({high_hz:.3} Hz)",
                    low_idx = violation.lower_index,
                    low_hz = violation.lower_frequency_hz,
                    high_idx = violation.higher_index,
                    high_hz = violation.higher_frequency_hz
                );
            }
        }
        if !self.resolved_samples.is_empty() {
            let _ = writeln!(&mut out, "index,frequency_hz,label");
            for sample in &self.resolved_samples {
                let _ = writeln!(
                    &mut out,
                    "{idx},{freq:.3},{label}",
                    idx = sample.index,
                    freq = sample.frequency_hz,
                    label = sample.label.as_deref().unwrap_or("")
                );
            }
        }
        out
    }
}

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct TuningSampleRow {
    pub index: i32,
    pub frequency_hz: f32,
    pub label: Option<String>,
}

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct MonotonicViolation {
    pub lower_index: i32,
    pub lower_frequency_hz: f32,
    pub higher_index: i32,
    pub higher_frequency_hz: f32,
}

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct MidiAnalysisReport {
    pub file: String,
    pub size_bytes: u64,
    pub header_format: Option<u16>,
    pub declared_tracks: Option<u16>,
    pub detected_tracks: usize,
    pub ticks_per_quarter: Option<u16>,
    pub key_hint: Option<String>,
    pub is_standard_midi: bool,
}

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct PitchIndexToFrequency {
    pub system: String,
    pub index: i32,
    pub frequency_hz: f32,
    pub label: Option<String>,
}

impl PitchIndexToFrequency {
    pub fn render_text(&self) -> String {
        let label = self.label.as_deref().unwrap_or("(unnamed)");
        format!(
            "Index {index} in {system} ⇒ {freq:.3} Hz {label}",
            index = self.index,
            system = self.system,
            freq = self.frequency_hz,
            label = label
        )
    }
}

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct FrequencyToIndexReport {
    pub system: String,
    pub input_frequency_hz: f32,
    pub resolved_index: i32,
    pub resolved_frequency_hz: f32,
    pub cents_error: f32,
    pub search_center: i32,
    pub search_span: i32,
    pub label: Option<String>,
}

impl FrequencyToIndexReport {
    pub fn render_text(&self) -> String {
        let label = self.label.as_deref().unwrap_or("(unnamed)");
        format!(
            "{freq:.3} Hz ≈ index {index} in {system} ({label}) — Δ {error:+.2} cents [search center {center}, span {span}]",
            freq = self.input_frequency_hz,
            index = self.resolved_index,
            system = self.system,
            label = label,
            error = self.cents_error,
            center = self.search_center,
            span = self.search_span
        )
    }
}

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct TemperamentRemapReport {
    pub from_system: String,
    pub to_system: String,
    pub mapping_count: usize,
    pub search_span: i32,
    pub mappings: Vec<TemperamentMappingRow>,
}

impl TemperamentRemapReport {
    pub fn render_text(&self) -> String {
        let mut out = String::new();
        let _ = writeln!(
            &mut out,
            "Remapped {count} pitch(es) from {from} → {to} (search span ±{span}).",
            count = self.mapping_count,
            from = self.from_system,
            to = self.to_system,
            span = self.search_span
        );
        let _ = writeln!(
            &mut out,
            "source_index,source_label,source_hz,target_index,target_label,target_hz,cents_delta"
        );
        for mapping in &self.mappings {
            let _ = writeln!(
                &mut out,
                "{src_idx},{src_label},{src_hz:.3},{dst_idx},{dst_label},{dst_hz:.3},{delta:+.2}",
                src_idx = mapping.source_index,
                src_label = mapping.source_label.as_deref().unwrap_or(""),
                src_hz = mapping.source_frequency_hz,
                dst_idx = mapping.target_index,
                dst_label = mapping.target_label.as_deref().unwrap_or(""),
                dst_hz = mapping.target_frequency_hz,
                delta = mapping.cents_delta
            );
        }
        out
    }
}

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct TemperamentMappingRow {
    pub source_index: i32,
    pub source_label: Option<String>,
    pub source_frequency_hz: f32,
    pub target_index: i32,
    pub target_label: Option<String>,
    pub target_frequency_hz: f32,
    pub cents_delta: f32,
}

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct MidiCsvConversionReport {
    pub direction: MidiConversionDirection,
    pub source: String,
    pub destination: Option<String>,
    pub note_count: usize,
    pub emitted_rows: usize,
    pub truncated: bool,
    pub ticks_per_quarter: Option<u16>,
    pub rows: Vec<MidiCsvRow>,
}

impl MidiCsvConversionReport {
    pub fn render_text(&self) -> String {
        match self.direction {
            MidiConversionDirection::MidiToCsv => self.render_as_csv(),
            MidiConversionDirection::CsvToMidi => self.render_csv_to_midi_summary(),
        }
    }

    fn render_as_csv(&self) -> String {
        let mut out = String::new();
        let _ = writeln!(
            &mut out,
            "track,channel,note,start_tick,end_tick,duration_tick,velocity"
        );
        for row in &self.rows {
            let _ = writeln!(
                &mut out,
                "{track},{channel},{note},{start},{end},{dur},{vel}",
                track = row.track,
                channel = row.channel,
                note = row.note,
                start = row.start_tick,
                end = row.end_tick,
                dur = row.duration_tick,
                vel = row.velocity
            );
        }
        if self.truncated {
            let remaining = self.note_count.saturating_sub(self.rows.len());
            let _ = writeln!(
                &mut out,
                "# truncated {remaining} additional row(s); re-run with --max-rows=<larger> to emit all"
            );
        }
        out
    }

    fn render_csv_to_midi_summary(&self) -> String {
        let mut out = String::new();
        let destination = self.destination.as_deref().unwrap_or("(not written)");
        let ticks = self
            .ticks_per_quarter
            .map(|value| value.to_string())
            .unwrap_or_else(|| "unknown".into());
        let _ = writeln!(
            &mut out,
            "Wrote {count} note(s) to {dest} (PPQ {ppq}).",
            count = self.note_count,
            dest = destination,
            ppq = ticks
        );
        let preview = self.rows.len().min(16);
        if preview > 0 {
            let _ = writeln!(
                &mut out,
                "Preview of first {preview} row(s): track,channel,note,start_tick,end_tick,duration_tick,velocity",
                preview = preview
            );
            for row in self.rows.iter().take(preview) {
                let _ = writeln!(
                    &mut out,
                    "{track},{channel},{note},{start},{end},{dur},{vel}",
                    track = row.track,
                    channel = row.channel,
                    note = row.note,
                    start = row.start_tick,
                    end = row.end_tick,
                    dur = row.duration_tick,
                    vel = row.velocity
                );
            }
        }
        out
    }
}

#[derive(Clone, Copy, Debug, Serialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub enum MidiConversionDirection {
    MidiToCsv,
    CsvToMidi,
}

#[derive(Clone, Debug, Serialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct MidiCsvRow {
    pub track: u16,
    pub channel: u8,
    pub note: u8,
    pub start_tick: u32,
    pub end_tick: u32,
    pub duration_tick: u32,
    pub velocity: u8,
}

impl MidiAnalysisReport {
    pub fn render_text(&self) -> String {
        let mut out = String::new();
        let _ = writeln!(
            &mut out,
            "MIDI file: {file} ({bytes} bytes).",
            file = self.file,
            bytes = self.size_bytes
        );
        let status = if self.is_standard_midi {
            "Standard MIDI header detected"
        } else {
            "Unknown/invalid MIDI header"
        };
        let _ = writeln!(&mut out, "{status}.");
        if let Some(format) = self.header_format {
            let _ = writeln!(
                &mut out,
                "Declared format: {format}, tracks: {declared:?}, detected chunks: {detected}.",
                declared = self.declared_tracks,
                detected = self.detected_tracks
            );
        } else {
            let _ = writeln!(
                &mut out,
                "Detected track chunks: {detected}.",
                detected = self.detected_tracks
            );
        }
        if let Some(ticks) = self.ticks_per_quarter {
            let _ = writeln!(&mut out, "Ticks per quarter note: {ticks}.", ticks = ticks);
        }
        if let Some(key) = &self.key_hint {
            let _ = writeln!(&mut out, "Context key hint: {key}.");
        }
        out
    }
}

pub fn summarize_chord(
    chord: DiatonicChord,
    registry: &TuningRegistry,
) -> PitchResult<ChordDetails> {
    let tones = chord
        .chord
        .tones(registry)?
        .into_iter()
        .enumerate()
        .map(|(index, pitch)| {
            let label = pitch
                .try_label(registry)
                .map(|value| value.to_string_lossy())
                .ok();
            let frequency_hz = pitch.try_freq_hz(registry)?;
            Ok(ChordToneSummary {
                index,
                label,
                frequency_hz,
            })
        })
        .collect::<PitchResult<_>>()?;

    Ok(ChordDetails {
        degree: chord.degree,
        numeral: roman_numeral(chord.degree),
        quality: chord.quality.map(|value| format!("{value:?}")),
        tones,
    })
}

pub fn collect_mode_pitches(
    scale: &Scale,
    registry: &TuningRegistry,
    step_count: usize,
) -> PitchResult<Vec<ModePitch>> {
    if step_count == 0 {
        return Ok(Vec::new());
    }

    let limit = step_count - 1;
    let mut summaries = Vec::with_capacity(step_count);
    for (degree, pitch) in scale
        .degree_pitches(limit, registry)?
        .into_iter()
        .enumerate()
    {
        summaries.push(summarize_mode_pitch(degree, pitch, registry)?);
    }
    Ok(summaries)
}

pub fn borrowed_chord(
    chord: DiatonicChord,
    registry: &TuningRegistry,
    target_index: Option<i32>,
) -> PitchResult<BorrowedChord> {
    let root_pitch = chord.chord.root().clone();
    let root_index = root_pitch.index();
    let root_label = root_pitch
        .try_label(registry)
        .map(|label| label.to_string_lossy())
        .ok();
    let root_frequency_hz = root_pitch.try_freq_hz(registry).ok();
    let matches_target = target_index
        .map(|target| Some(target) == root_index)
        .unwrap_or(false);

    let details = summarize_chord(chord, registry)?;

    Ok(BorrowedChord {
        details,
        root_index,
        root_label,
        root_frequency_hz,
        matches_target,
    })
}

fn summarize_mode_pitch(
    degree: usize,
    pitch: Pitch,
    registry: &TuningRegistry,
) -> PitchResult<ModePitch> {
    let label = pitch
        .try_label(registry)
        .map(|value| value.to_string_lossy())
        .ok();
    let frequency_hz = pitch.try_freq_hz(registry)?;
    Ok(ModePitch {
        degree,
        index: pitch.index(),
        label,
        frequency_hz,
    })
}

fn roman_numeral(degree: usize) -> String {
    const ROMANS: [&str; 7] = ["I", "II", "III", "IV", "V", "VI", "VII"];
    ROMANS
        .get(degree % ROMANS.len())
        .copied()
        .unwrap_or("I")
        .to_string()
}

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct MelodyExtrapolationReport {
    pub system: String,
    pub input_count: usize,
    pub input_notes: Vec<i32>,
    pub model_order: usize,
    pub context_pitch_classes: Vec<u8>,
    pub predictions: Vec<MelodyPrediction>,
    pub key_hint: Option<String>,
}

impl MelodyExtrapolationReport {
    pub fn render_text(&self) -> String {
        let mut out = String::new();
        let _ = writeln!(
            &mut out,
            "Melody extrapolation (n-gram order {order}) in {system}.",
            order = self.model_order,
            system = self.system
        );
        let _ = writeln!(
            &mut out,
            "Input: {count} notes → pitch classes {pcs:?}",
            count = self.input_count,
            pcs = self.context_pitch_classes
        );
        if let Some(key) = &self.key_hint {
            let _ = writeln!(&mut out, "Context key: {key}.");
        }
        if self.predictions.is_empty() {
            let _ = writeln!(
                &mut out,
                "No predictions available (context not seen in training)."
            );
            return out;
        }
        let _ = writeln!(&mut out, "Predicted continuations:");
        for (idx, pred) in self.predictions.iter().enumerate() {
            let label = pred.label.as_deref().unwrap_or("—");
            let _ = writeln!(
                &mut out,
                "  {rank:>2}. pc {pc:>2} → index {index:>3} ({label}) — {prob:.1}% ({freq:.3} Hz)",
                rank = idx + 1,
                pc = pred.pitch_class,
                index = pred.suggested_index,
                label = label,
                prob = pred.probability * 100.0,
                freq = pred.frequency_hz
            );
        }
        out
    }
}

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct MelodyPrediction {
    pub pitch_class: u8,
    pub suggested_index: i32,
    pub label: Option<String>,
    pub frequency_hz: f32,
    pub probability: f32,
}

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct ChordExtrapolationReport {
    pub input_count: usize,
    pub input_progression: Vec<String>,
    pub model_order: usize,
    pub context_chords: Vec<String>,
    pub predictions: Vec<ChordPrediction>,
    pub key_hint: Option<String>,
}

impl ChordExtrapolationReport {
    pub fn render_text(&self) -> String {
        let mut out = String::new();
        let _ = writeln!(
            &mut out,
            "Chord extrapolation (n-gram order {order}).",
            order = self.model_order
        );
        let _ = writeln!(
            &mut out,
            "Input progression ({count} chords): {prog}",
            count = self.input_count,
            prog = self.input_progression.join(" → ")
        );
        let _ = writeln!(
            &mut out,
            "Context (last {order} chord(s)): {ctx}",
            order = self.model_order,
            ctx = self.context_chords.join(" → ")
        );
        if let Some(key) = &self.key_hint {
            let _ = writeln!(&mut out, "Context key: {key}.");
        }
        if self.predictions.is_empty() {
            let _ = writeln!(
                &mut out,
                "No predictions available (context not seen in training)."
            );
            return out;
        }
        let _ = writeln!(&mut out, "Predicted next chords:");
        for (idx, pred) in self.predictions.iter().enumerate() {
            let _ = writeln!(
                &mut out,
                "  {rank:>2}. {chord:<6} — {prob:.1}%",
                rank = idx + 1,
                chord = pred.chord,
                prob = pred.probability * 100.0
            );
        }
        out
    }
}

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct ChordPrediction {
    pub chord: String,
    pub probability: f32,
}

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct ProfileReport {
    pub input_type: String,
    pub event_count: usize,
    pub total_duration_sec: Option<f64>,
    pub density_events_per_sec: Option<f64>,
    pub pitch_range: PitchRangeReport,
    pub timing: TimingReport,
}

impl ProfileReport {
    pub fn render_text(&self) -> String {
        let mut out = String::new();
        let _ = writeln!(
            &mut out,
            "Profile for {} ({} events).",
            self.input_type, self.event_count
        );

        if let Some(duration) = self.total_duration_sec {
            let _ = writeln!(&mut out, "Total duration: {:.2} seconds.", duration);
        }

        if let Some(density) = self.density_events_per_sec {
            let _ = writeln!(&mut out, "Density: {:.2} events/second.", density);
        }

        let _ = writeln!(&mut out, "\nPitch Range:");
        if let Some(min) = self.pitch_range.min_pitch {
            let _ = writeln!(&mut out, "  Min pitch: {}", min);
        }
        if let Some(max) = self.pitch_range.max_pitch {
            let _ = writeln!(&mut out, "  Max pitch: {}", max);
        }
        if let Some(median) = self.pitch_range.median_pitch {
            let _ = writeln!(&mut out, "  Median pitch: {:.1}", median);
        }
        if let Some(p25) = self.pitch_range.p25_pitch {
            let _ = writeln!(&mut out, "  25th percentile: {:.1}", p25);
        }
        if let Some(p75) = self.pitch_range.p75_pitch {
            let _ = writeln!(&mut out, "  75th percentile: {:.1}", p75);
        }

        let _ = writeln!(&mut out, "\nTiming:");
        if let Some(min) = self.timing.min_ioi_sec {
            let _ = writeln!(&mut out, "  Min IOI: {:.3} seconds.", min);
        }
        if let Some(max) = self.timing.max_ioi_sec {
            let _ = writeln!(&mut out, "  Max IOI: {:.3} seconds.", max);
        }
        if let Some(median) = self.timing.median_ioi_sec {
            let _ = writeln!(&mut out, "  Median IOI: {:.3} seconds.", median);
        }
        if let Some(p25) = self.timing.p25_ioi_sec {
            let _ = writeln!(&mut out, "  25th percentile IOI: {:.3} seconds.", p25);
        }
        if let Some(p75) = self.timing.p75_ioi_sec {
            let _ = writeln!(&mut out, "  75th percentile IOI: {:.3} seconds.", p75);
        }
        if let Some(swing) = self.timing.swing_ratio {
            let _ = writeln!(&mut out, "  Detected swing ratio: {:.2}", swing);
        }

        out
    }
}

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct PitchRangeReport {
    pub min_pitch: Option<u8>,
    pub max_pitch: Option<u8>,
    pub median_pitch: Option<f64>,
    pub p25_pitch: Option<f64>,
    pub p75_pitch: Option<f64>,
}

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct TimingReport {
    pub min_ioi_sec: Option<f64>,
    pub max_ioi_sec: Option<f64>,
    pub median_ioi_sec: Option<f64>,
    pub p25_ioi_sec: Option<f64>,
    pub p75_ioi_sec: Option<f64>,
    pub swing_ratio: Option<f64>,
}

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct EstimateReport {
    pub input_type: String,
    pub tempo_bpm: Option<f64>,
    pub tempo_confidence: Option<f64>,
    pub key_estimate: Option<String>,
    pub key_confidence: Option<f64>,
    pub meter: Option<String>,
    pub meter_confidence: Option<f64>,
}

impl EstimateReport {
    pub fn render_text(&self) -> String {
        let mut out = String::new();
        let _ = writeln!(
            &mut out,
            "Musical feature estimation for {}.",
            self.input_type
        );

        if let Some(key) = &self.key_estimate {
            let conf = self.key_confidence.unwrap_or(0.0);
            let _ = writeln!(
                &mut out,
                "\nEstimated key: {} (confidence: {:.1}%)",
                key,
                conf * 100.0
            );
        } else {
            let _ = writeln!(&mut out, "\nEstimated key: (not available)");
        }

        if let Some(tempo) = self.tempo_bpm {
            let conf = self.tempo_confidence.unwrap_or(0.0);
            let _ = writeln!(
                &mut out,
                "Estimated tempo: {:.1} BPM (confidence: {:.1}%)",
                tempo,
                conf * 100.0
            );
        } else {
            let _ = writeln!(
                &mut out,
                "Estimated tempo: (not available - no timing data)"
            );
        }

        if let Some(meter) = &self.meter {
            let conf = self.meter_confidence.unwrap_or(0.0);
            let _ = writeln!(
                &mut out,
                "Estimated meter: {} (confidence: {:.1}%)",
                meter,
                conf * 100.0
            );
        } else {
            let _ = writeln!(
                &mut out,
                "Estimated meter: (not available - no timing data)"
            );
        }

        out
    }
}
