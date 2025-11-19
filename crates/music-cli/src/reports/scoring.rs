//! Scoring reports for melody, chord, and progression quality analysis.

use std::fmt::Write;

#[cfg(feature = "schema")]
use schemars::JsonSchema;
use serde::Serialize;

use crate::reports::analysis::{Ambitus, CadenceSummary, FunctionCounts};

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
    pub cadence: Option<CadenceSummary>,
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
