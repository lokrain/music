#[cfg(feature = "schema")]
use schemars::JsonSchema;
use serde::Serialize;
use std::fmt::Write as FmtWrite;

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct Ambitus {
    pub lowest: i32,
    pub highest: i32,
    pub span: i32,
}

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct KeyHypothesis {
    pub tonic_label: String,
    pub tonic_pitch_class: u8,
    pub tonic_index: i32,
    pub mode: String,
    pub match_ratio: f32,
    pub enforced: bool,
}

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct PitchClassBin {
    pub pitch_class: u8,
    pub count: usize,
}

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct TensionMetrics {
    pub out_of_scale: usize,
    pub percent_out_of_scale: f32,
}

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct MelodyAnalysisReport {
    pub note_count: usize,
    pub distinct_pitch_classes: usize,
    pub ambitus: Ambitus,
    pub best_key: KeyHypothesis,
    pub histogram: Vec<PitchClassBin>,
    pub tension: TensionMetrics,
}

impl MelodyAnalysisReport {
    pub fn render_text(&self) -> String {
        let mut out = String::new();
        let _ = writeln!(
            &mut out,
            "Melody analysis: {notes} notes ({classes} unique pitch classes).",
            notes = self.note_count,
            classes = self.distinct_pitch_classes
        );
        let _ = writeln!(
            &mut out,
            "Ambitus: {span} semitones (lowest {low}, highest {high}).",
            span = self.ambitus.span,
            low = self.ambitus.lowest,
            high = self.ambitus.highest
        );
        let forced = if self.best_key.enforced {
            " (forced)"
        } else {
            ""
        };
        let _ = writeln!(
            &mut out,
            "Best key: {label} {mode}{forced} — match {ratio:.1}%",
            label = self.best_key.tonic_label,
            mode = self.best_key.mode,
            forced = forced,
            ratio = self.best_key.match_ratio * 100.0
        );
        let _ = writeln!(
            &mut out,
            "Tension: {out_of_scale} notes ({percent:.1}%) outside the implied scale.",
            out_of_scale = self.tension.out_of_scale,
            percent = self.tension.percent_out_of_scale
        );
        if !self.histogram.is_empty() {
            let _ = writeln!(&mut out, "Pitch-class histogram:");
            for bin in &self.histogram {
                let _ = writeln!(
                    &mut out,
                    "  pc {pc:>2}: {count}",
                    pc = bin.pitch_class,
                    count = bin.count
                );
            }
        }
        out
    }
}

#[derive(Debug, Default, Serialize, Clone, Copy)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct FunctionCounts {
    pub tonic: usize,
    pub predominant: usize,
    pub dominant: usize,
    pub other: usize,
}

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct CadenceSummary {
    pub pattern: String,
    pub description: String,
    pub confidence: f32,
}

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct ChordAnalysisReport {
    pub progression: Vec<String>,
    pub chord_count: usize,
    pub unique_chords: usize,
    pub function_counts: FunctionCounts,
    pub cadence: Option<CadenceSummary>,
    pub key_hint: Option<String>,
}

impl ChordAnalysisReport {
    pub fn render_text(&self) -> String {
        let mut out = String::new();
        let _ = writeln!(
            &mut out,
            "Chord progression ({count} chords, {unique} unique):",
            count = self.chord_count,
            unique = self.unique_chords
        );
        let _ = writeln!(&mut out, "  {}", self.progression.join(" → "));
        let _ = writeln!(
            &mut out,
            "Functional counts: tonic {tonic}, predominant {pred}, dominant {dom}, other {other}.",
            tonic = self.function_counts.tonic,
            pred = self.function_counts.predominant,
            dom = self.function_counts.dominant,
            other = self.function_counts.other
        );
        match &self.cadence {
            Some(cadence) => {
                let _ = writeln!(
                    &mut out,
                    "Cadence: {pattern} — {desc} (confidence {conf:.0}%).",
                    pattern = cadence.pattern,
                    desc = cadence.description,
                    conf = cadence.confidence * 100.0
                );
            }
            None => {
                let _ = writeln!(&mut out, "Cadence: none detected.");
            }
        }
        if let Some(key) = &self.key_hint {
            let _ = writeln!(&mut out, "Context key hint: {key}.");
        }
        out
    }
}
