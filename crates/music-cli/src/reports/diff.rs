#[cfg(feature = "schema")]
use schemars::JsonSchema;
use serde::Serialize;
use std::fmt::Write as FmtWrite;

use crate::reports::analysis::{Ambitus, PitchClassBin};
use crate::responses::FunctionCounts; // until FunctionCounts migrates fully

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct MelodyProfileSummary {
    pub note_count: usize,
    pub distinct_pitch_classes: usize,
    pub ambitus: Ambitus,
    pub histogram: Vec<PitchClassBin>,
}

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct MelodyDiffReport {
    pub system: String,
    pub key_hint: Option<String>,
    pub left: MelodyProfileSummary,
    pub right: MelodyProfileSummary,
    pub shared_pitch_classes: Vec<u8>,
    pub left_only_pitch_classes: Vec<u8>,
    pub right_only_pitch_classes: Vec<u8>,
    pub histogram_distance: f32,
    pub commentary: Vec<String>,
}

fn render_melody_side(out: &mut String, label: &str, profile: &MelodyProfileSummary) {
    let _ = writeln!(
        out,
        "{label} — notes {count}, distinct pcs {distinct}, ambitus {span} st (lowest {low}, highest {high}).",
        label = label,
        count = profile.note_count,
        distinct = profile.distinct_pitch_classes,
        span = profile.ambitus.span,
        low = profile.ambitus.lowest,
        high = profile.ambitus.highest
    );
    if !profile.histogram.is_empty() {
        let _ = writeln!(out, "  Histogram:");
        for bin in &profile.histogram {
            let _ = writeln!(
                out,
                "    pc {pc:>2}: {count}",
                pc = bin.pitch_class,
                count = bin.count
            );
        }
    }
}

impl MelodyDiffReport {
    pub fn render_text(&self) -> String {
        let mut out = String::new();
        let _ = writeln!(
            &mut out,
            "Melody diff ({system}) — shared pcs {shared:?}, distance {distance:.1}%.",
            system = self.system,
            shared = self.shared_pitch_classes,
            distance = self.histogram_distance * 100.0
        );
        if let Some(key) = &self.key_hint {
            let _ = writeln!(&mut out, "Context key: {key}.");
        }
        render_melody_side(&mut out, "Left", &self.left);
        render_melody_side(&mut out, "Right", &self.right);
        if !self.left_only_pitch_classes.is_empty() {
            let _ = writeln!(
                &mut out,
                "Left-only pcs: {:?}.",
                self.left_only_pitch_classes
            );
        }
        if !self.right_only_pitch_classes.is_empty() {
            let _ = writeln!(
                &mut out,
                "Right-only pcs: {:?}.",
                self.right_only_pitch_classes
            );
        }
        if !self.commentary.is_empty() {
            let _ = writeln!(&mut out);
            for note in &self.commentary {
                let _ = writeln!(&mut out, "  • {note}");
            }
        }
        out
    }
}

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct MidiFileSummary {
    pub file: String,
    pub size_bytes: u64,
    pub header_format: Option<u16>,
    pub declared_tracks: Option<u16>,
    pub detected_tracks: usize,
    pub ticks_per_quarter: Option<u16>,
    pub is_standard_midi: bool,
}

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct MidiDiffReport {
    pub left: MidiFileSummary,
    pub right: MidiFileSummary,
    pub size_delta: i64,
    pub track_delta: i32,
    pub commentary: Vec<String>,
}

impl MidiDiffReport {
    pub fn render_text(&self) -> String {
        let mut out = String::new();
        let _ = writeln!(
            &mut out,
            "MIDI diff — size Δ {size} bytes, track Δ {tracks}.",
            size = self.size_delta,
            tracks = self.track_delta
        );
        render_midi_side(&mut out, "Left", &self.left);
        render_midi_side(&mut out, "Right", &self.right);
        if !self.commentary.is_empty() {
            let _ = writeln!(&mut out);
            for note in &self.commentary {
                let _ = writeln!(&mut out, "- {note}");
            }
        }
        out
    }
}

fn render_midi_side(out: &mut String, label: &str, summary: &MidiFileSummary) {
    let _ = writeln!(
        out,
        "{label}: {file} ({size} bytes, fmt {fmt:?}, declared {declared:?}, detected {detected}).",
        label = label,
        file = summary.file,
        size = summary.size_bytes,
        fmt = summary.header_format,
        declared = summary.declared_tracks,
        detected = summary.detected_tracks
    );
    if let Some(ticks) = summary.ticks_per_quarter {
        let _ = writeln!(out, "  TPQ: {ticks}.");
    }
    let _ = writeln!(
        out,
        "  Standard MIDI header: {status}.",
        status = if summary.is_standard_midi {
            "yes"
        } else {
            "no"
        }
    );
}

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct ProgressionProfileSummary {
    pub progression: Vec<String>,
    pub chord_count: usize,
    pub unique_chords: usize,
    pub function_counts: FunctionCounts,
    pub cadence: Option<crate::reports::analysis::CadenceSummary>,
}

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct ProgressionDiffReport {
    pub key_hint: Option<String>,
    pub left: ProgressionProfileSummary,
    pub right: ProgressionProfileSummary,
    pub shared_chords: Vec<String>,
    pub left_unique: Vec<String>,
    pub right_unique: Vec<String>,
    pub commentary: Vec<String>,
}

impl ProgressionDiffReport {
    pub fn render_text(&self) -> String {
        let mut out = String::new();
        if let Some(key) = &self.key_hint {
            let _ = writeln!(&mut out, "Context key: {key}.");
        }
        let _ = writeln!(
            &mut out,
            "Left progression ({left_total} chords, {left_unique} unique):",
            left_total = self.left.chord_count,
            left_unique = self.left.unique_chords
        );
        let _ = writeln!(&mut out, "  {}", self.left.progression.join(" → "));
        let _ = writeln!(
            &mut out,
            "Right progression ({right_total} chords, {right_unique} unique):",
            right_total = self.right.chord_count,
            right_unique = self.right.unique_chords
        );
        let _ = writeln!(&mut out, "  {}", self.right.progression.join(" → "));
        if !self.shared_chords.is_empty() {
            let _ = writeln!(&mut out, "Shared chords: {}", self.shared_chords.join(", "));
        }
        if !self.left_unique.is_empty() {
            let _ = writeln!(&mut out, "Left-only: {}", self.left_unique.join(", "));
        }
        if !self.right_unique.is_empty() {
            let _ = writeln!(&mut out, "Right-only: {}", self.right_unique.join(", "));
        }
        if !self.commentary.is_empty() {
            let _ = writeln!(&mut out);
            for note in &self.commentary {
                let _ = writeln!(&mut out, "  • {note}");
            }
        }
        out
    }
}
