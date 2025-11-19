#[cfg(feature = "schema")]
use schemars::JsonSchema;
use serde::Serialize;
use std::fmt::Write as FmtWrite;

use crate::reports::analysis::Ambitus;
use crate::responses::FunctionCounts; // until moved fully

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
