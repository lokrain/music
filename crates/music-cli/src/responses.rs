#![allow(dead_code)]

use std::fmt::Write as FmtWrite;

use chord::DiatonicChord;
use music_engine::prelude::{Pitch, PitchError, Scale, TuningRegistry, chord};
use serde::Serialize;

pub type PitchResult<T> = Result<T, PitchError>;

#[derive(Debug, Serialize)]
pub struct PitchSummary {
    pub system: String,
    pub index: i32,
    pub label: String,
    pub frequency_hz: f32,
}

impl PitchSummary {
    pub fn render_text(&self) -> String {
        format!(
            "Pitch {index} in {system}: {label} ({freq:.3} Hz)",
            index = self.index,
            system = self.system,
            label = self.label,
            freq = self.frequency_hz
        )
    }
}

#[derive(Debug, Serialize)]
pub struct PitchExplanation {
    pub summary: PitchSummary,
    pub octave: i32,
    pub pitch_class: u8,
    pub pitch_class_label: String,
    pub semitone_delta_from_a4: i32,
    pub cents_offset_from_a4: f32,
    pub context: Option<PitchContext>,
    pub narrative: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct PitchContext {
    pub key: String,
    pub mode: String,
    pub degree: usize,
    pub degree_label: String,
    pub function: String,
}

impl PitchExplanation {
    pub fn render_text(&self) -> String {
        let mut out = String::new();
        let summary = self.summary.render_text();
        let _ = writeln!(&mut out, "{summary}");
        let _ = writeln!(
            &mut out,
            "Octave {octave}, pitch class {pc} ({label}).",
            octave = self.octave,
            pc = self.pitch_class,
            label = self.pitch_class_label
        );
        let _ = writeln!(
            &mut out,
            "Relative to A4: {delta:+} semitone(s), {cents:+.1} cents.",
            delta = self.semitone_delta_from_a4,
            cents = self.cents_offset_from_a4
        );
        if let Some(context) = &self.context {
            let _ = writeln!(
                &mut out,
                "In {key} {mode}, this is degree {degree} ({label}), functioning as {function}.",
                key = context.key,
                mode = context.mode,
                degree = context.degree,
                label = context.degree_label,
                function = context.function
            );
        }
        for paragraph in &self.narrative {
            let _ = writeln!(&mut out);
            let _ = writeln!(&mut out, "{paragraph}");
        }
        out
    }
}

#[derive(Debug, Serialize)]
pub struct ChordListing {
    pub system: String,
    pub root_index: i32,
    pub root_label: Option<String>,
    pub scale: String,
    pub voicing: String,
    pub chord_count: usize,
    pub chords: Vec<ChordDetails>,
}

impl ChordListing {
    pub fn render_text(&self) -> String {
        let mut out = String::new();
        let root = self
            .root_label
            .as_deref()
            .map(|value| format!(" ({value})"))
            .unwrap_or_default();
        let _ = writeln!(
            &mut out,
            "{count} {voicing} derived from the {scale} scale in {system} (root {index}{root}).",
            count = self.chord_count,
            voicing = self.voicing,
            scale = self.scale,
            system = self.system,
            index = self.root_index,
            root = root
        );
        let _ = writeln!(&mut out);

        for chord in &self.chords {
            let quality = chord
                .quality
                .as_deref()
                .map(str::to_string)
                .unwrap_or_else(|| "Unclassified".into());
            let _ = writeln!(
                &mut out,
                "{numeral:<4} (degree {degree}): {quality}",
                numeral = chord.numeral,
                degree = chord.degree + 1,
                quality = quality
            );
            for tone in &chord.tones {
                let label = tone.label.as_deref().unwrap_or("—");
                let _ = writeln!(
                    &mut out,
                    "  • tone {index}: {label} ({freq:.3} Hz)",
                    index = tone.index,
                    label = label,
                    freq = tone.frequency_hz
                );
            }
            let _ = writeln!(&mut out);
        }

        out
    }
}

#[derive(Debug, Serialize)]
pub struct ChordDetails {
    pub degree: usize,
    pub numeral: String,
    pub quality: Option<String>,
    pub tones: Vec<ChordToneSummary>,
}

#[derive(Debug, Serialize)]
pub struct ChordToneSummary {
    pub index: usize,
    pub label: Option<String>,
    pub frequency_hz: f32,
}

#[derive(Debug, Serialize)]
pub struct ChordExplanation {
    pub system: String,
    pub scale_name: String,
    pub scale_root_index: i32,
    pub scale_root_label: Option<String>,
    pub voicing: String,
    pub summary: ChordDetails,
    pub function: String,
    pub narrative: Vec<String>,
}

impl ChordExplanation {
    pub fn render_text(&self) -> String {
        let mut out = String::new();
        let root_label = self
            .scale_root_label
            .as_deref()
            .map(|label| format!("{label} "))
            .unwrap_or_default();
        let _ = writeln!(
            &mut out,
            "Chord explanation in {system}: {scale} rooted at {root_label}{root_index} ({voicing}).",
            system = self.system,
            scale = self.scale_name,
            root_label = root_label,
            root_index = self.scale_root_index,
            voicing = self.voicing
        );
        let quality = self.summary.quality.as_deref().unwrap_or("Unclassified");
        let _ = writeln!(
            &mut out,
            "Degree {degree} ({numeral}) — {quality}, function: {function}.",
            degree = self.summary.degree + 1,
            numeral = self.summary.numeral,
            quality = quality,
            function = self.function
        );
        for tone in &self.summary.tones {
            let label = tone.label.as_deref().unwrap_or("—");
            let _ = writeln!(
                &mut out,
                "  • tone {idx}: {label} ({freq:.3} Hz)",
                idx = tone.index,
                label = label,
                freq = tone.frequency_hz
            );
        }
        for paragraph in &self.narrative {
            let _ = writeln!(&mut out);
            let _ = writeln!(&mut out, "{paragraph}");
        }
        out
    }
}

#[derive(Debug, Serialize)]
pub struct PatternContext {
    pub system: String,
    pub scale: String,
    pub root_index: i32,
    pub root_label: Option<String>,
    pub density: String,
}

#[derive(Debug, Serialize)]
pub struct GeneratedNote {
    pub degree: usize,
    pub octave: i32,
    pub index: i32,
    pub label: Option<String>,
    pub frequency_hz: f32,
}

#[derive(Debug, Serialize)]
pub struct MotifGeneration {
    pub context: PatternContext,
    pub note_count: usize,
    pub contour: Vec<i32>,
    pub notes: Vec<GeneratedNote>,
    pub description: String,
}

impl MotifGeneration {
    pub fn render_text(&self) -> String {
        let mut out = String::new();
        let root_label = self
            .context
            .root_label
            .as_deref()
            .map(|label| format!(" ({label})"))
            .unwrap_or_default();
        let _ = writeln!(
            &mut out,
            "Motif ({density}) in {scale} / {system}, root {root}{root_label}.",
            density = self.context.density,
            scale = self.context.scale,
            system = self.context.system,
            root = self.context.root_index,
            root_label = root_label
        );
        if !self.description.is_empty() {
            let _ = writeln!(&mut out, "{desc}", desc = self.description);
        }
        if !self.contour.is_empty() {
            let markings: Vec<String> = self
                .contour
                .iter()
                .enumerate()
                .map(|(idx, delta)| format!("Δ{step:02} {delta:+}", step = idx + 1, delta = delta))
                .collect();
            let _ = writeln!(&mut out, "Contour: {values}.", values = markings.join(", "));
        }
        for (idx, note) in self.notes.iter().enumerate() {
            let label = note.label.as_deref().unwrap_or("—");
            let _ = writeln!(
                &mut out,
                "  {order:>2}. degree {degree}{octave:+}, index {index}, {label} ({freq:.3} Hz)",
                order = idx + 1,
                degree = note.degree,
                octave = note.octave,
                index = note.index,
                label = label,
                freq = note.frequency_hz
            );
        }
        out
    }
}

#[derive(Debug, Serialize)]
pub struct ArpeggioStep {
    pub order: usize,
    pub direction: String,
    #[serde(flatten)]
    pub note: GeneratedNote,
}

#[derive(Debug, Serialize)]
pub struct ArpeggioGeneration {
    pub context: PatternContext,
    pub register_span: i32,
    pub steps: Vec<ArpeggioStep>,
    pub description: String,
}

impl ArpeggioGeneration {
    pub fn render_text(&self) -> String {
        let mut out = String::new();
        let root_label = self
            .context
            .root_label
            .as_deref()
            .map(|label| format!(" ({label})"))
            .unwrap_or_default();
        let _ = writeln!(
            &mut out,
            "Arpeggio ({density}) in {scale} / {system}, root {root}{root_label}. Span {span} semitones.",
            density = self.context.density,
            scale = self.context.scale,
            system = self.context.system,
            root = self.context.root_index,
            root_label = root_label,
            span = self.register_span
        );
        if !self.description.is_empty() {
            let _ = writeln!(&mut out, "{desc}", desc = self.description);
        }
        for step in &self.steps {
            let label = step.note.label.as_deref().unwrap_or("—");
            let _ = writeln!(
                &mut out,
                "  {order:>2}. [{direction:^4}] degree {degree}{octave:+}, index {index}, {label} ({freq:.3} Hz)",
                order = step.order,
                direction = step.direction,
                degree = step.note.degree,
                octave = step.note.octave,
                index = step.note.index,
                label = label,
                freq = step.note.frequency_hz
            );
        }
        out
    }
}

#[derive(Debug, Serialize)]
pub struct RhythmEvent {
    pub beat: f32,
    pub duration_beats: f32,
    pub accent: bool,
    pub suggested_degree: usize,
}

#[derive(Debug, Serialize)]
pub struct RhythmCellGeneration {
    pub context: PatternContext,
    pub meter: String,
    pub length_beats: f32,
    pub events: Vec<RhythmEvent>,
    pub description: String,
}

impl RhythmCellGeneration {
    pub fn render_text(&self) -> String {
        let mut out = String::new();
        let root_label = self
            .context
            .root_label
            .as_deref()
            .map(|label| format!(" ({label})"))
            .unwrap_or_default();
        let _ = writeln!(
            &mut out,
            "Rhythm cell ({density}) for {scale} / {system}, root {root}{root_label}, meter {meter}, {beats:.1} beats.",
            density = self.context.density,
            scale = self.context.scale,
            system = self.context.system,
            root = self.context.root_index,
            root_label = root_label,
            meter = self.meter,
            beats = self.length_beats
        );
        if !self.description.is_empty() {
            let _ = writeln!(&mut out, "{desc}", desc = self.description);
        }
        if self.events.is_empty() {
            return out;
        }

        let _ = writeln!(&mut out, "  beat  dur  acc  degree");
        for event in &self.events {
            let accent = if event.accent { "*" } else { "." };
            let _ = writeln!(
                &mut out,
                "  {beat:>4.1}  {dur:>3.1}   {accent}     {degree}",
                beat = event.beat,
                dur = event.duration_beats,
                accent = accent,
                degree = event.suggested_degree
            );
        }
        out
    }
}

#[derive(Debug, Serialize)]
pub struct ScaleExplanation {
    pub system: String,
    pub root_index: i32,
    pub root_label: String,
    pub scale_name: String,
    pub mode_alias: Option<String>,
    pub degrees: Vec<ScaleDegreeSummary>,
    pub pattern_cents: Vec<f32>,
    pub narrative: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct ScaleDegreeSummary {
    pub degree: usize,
    pub label: String,
    pub frequency_hz: f32,
    pub interval_cents: f32,
    pub semitone_offset: Option<i32>,
    pub role: String,
}

impl ScaleExplanation {
    pub fn render_text(&self) -> String {
        let mut out = String::new();
        let _ = writeln!(
            &mut out,
            "Scale explanation: {scale} rooted at {label} ({root_index}) in {system}.",
            scale = self.scale_name,
            label = self.root_label,
            root_index = self.root_index,
            system = self.system
        );
        if let Some(alias) = &self.mode_alias {
            let _ = writeln!(&mut out, "Alias/rotation: {alias}.");
        }
        if !self.pattern_cents.is_empty() {
            let descriptions: Vec<String> = self
                .pattern_cents
                .iter()
                .enumerate()
                .map(|(idx, cents)| {
                    let step = idx + 1;
                    format!("step {step}: {cents:.1} cents", step = step, cents = cents)
                })
                .collect();
            let _ = writeln!(
                &mut out,
                "Step pattern: {pattern}.",
                pattern = descriptions.join(", ")
            );
        }
        for degree in &self.degrees {
            let offset = degree
                .semitone_offset
                .map(|value| format!(" {value:+} st"))
                .unwrap_or_default();
            let _ = writeln!(
                &mut out,
                "  • Degree {deg} ({label}) — {role}, {freq:.3} Hz, {cents:.1} cents{offset}.",
                deg = degree.degree,
                label = degree.label,
                role = degree.role,
                freq = degree.frequency_hz,
                cents = degree.interval_cents,
                offset = offset
            );
        }
        for paragraph in &self.narrative {
            let _ = writeln!(&mut out);
            let _ = writeln!(&mut out, "{paragraph}");
        }
        out
    }
}

#[derive(Debug, Serialize)]
pub struct SystemsListing {
    pub reference_index: i32,
    pub systems: Vec<SystemSummary>,
}

impl SystemsListing {
    pub fn render_text(&self) -> String {
        let mut out = String::new();
        let count = self.systems.len();
        let _ = writeln!(
            &mut out,
            "{count} tuning systems registered (reference index {reference}).",
            count = count,
            reference = self.reference_index
        );
        for system in &self.systems {
            let label = system
                .label
                .as_deref()
                .map(|value| format!(" — {value}"))
                .unwrap_or_default();
            let _ = writeln!(
                &mut out,
                "  • {id}: {freq:.3} Hz at index {index}{label}",
                id = system.id,
                freq = system.frequency_hz,
                index = system.reference_index,
                label = label
            );
        }
        out
    }
}

#[derive(Debug, Serialize)]
pub struct SystemSummary {
    pub id: String,
    pub reference_index: i32,
    pub frequency_hz: f32,
    pub label: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ModeListing {
    pub system: String,
    pub root_index: i32,
    pub scale: String,
    pub mode_count: usize,
    pub modes: Vec<ModeSummary>,
}

impl ModeListing {
    pub fn render_text(&self) -> String {
        let mut out = String::new();
        let _ = writeln!(
            &mut out,
            "{count} mode(s) derived from the {scale} scale (root {root}) in {system}.",
            count = self.mode_count,
            scale = self.scale,
            root = self.root_index,
            system = self.system
        );
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
            for pitch in &mode.pitches {
                let label = pitch.label.as_deref().unwrap_or("—");
                let _ = writeln!(
                    &mut out,
                    "    • degree {degree}: {label} ({freq:.3} Hz)",
                    degree = pitch.degree + 1,
                    label = label,
                    freq = pitch.frequency_hz
                );
            }
            let _ = writeln!(&mut out);
        }

        out
    }
}

#[derive(Debug, Serialize)]
pub struct ModeSummary {
    pub mode_index: usize,
    pub rotation_degree: usize,
    pub mode_name: Option<String>,
    pub root_index: Option<i32>,
    pub root_label: Option<String>,
    pub pitches: Vec<ModePitch>,
}

#[derive(Debug, Serialize)]
pub struct ModePitch {
    pub degree: usize,
    pub index: Option<i32>,
    pub label: Option<String>,
    pub frequency_hz: f32,
}

#[derive(Debug, Serialize)]
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
pub struct ModeReharm {
    pub mode_index: usize,
    pub rotation_degree: usize,
    pub mode_name: Option<String>,
    pub root_index: Option<i32>,
    pub root_label: Option<String>,
    pub borrowed_chords: Vec<BorrowedChord>,
}

#[derive(Debug, Serialize)]
pub struct BorrowedChord {
    #[serde(flatten)]
    pub details: ChordDetails,
    pub root_index: Option<i32>,
    pub root_label: Option<String>,
    pub root_frequency_hz: Option<f32>,
    pub matches_target: bool,
}

#[derive(Debug, Serialize)]
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

#[derive(Debug, Serialize)]
pub struct Ambitus {
    pub lowest: i32,
    pub highest: i32,
    pub span: i32,
}

#[derive(Debug, Serialize)]
pub struct KeyHypothesis {
    pub tonic_label: String,
    pub tonic_pitch_class: u8,
    pub tonic_index: i32,
    pub mode: String,
    pub match_ratio: f32,
    pub enforced: bool,
}

#[derive(Debug, Serialize)]
pub struct PitchClassBin {
    pub pitch_class: u8,
    pub count: usize,
}

#[derive(Debug, Serialize)]
pub struct TensionMetrics {
    pub out_of_scale: usize,
    pub percent_out_of_scale: f32,
}

#[derive(Debug, Serialize)]
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

#[derive(Debug, Default, Serialize, Clone, Copy)]
pub struct FunctionCounts {
    pub tonic: usize,
    pub predominant: usize,
    pub dominant: usize,
    pub other: usize,
}

#[derive(Debug, Serialize)]
pub struct CadenceSummary {
    pub pattern: String,
    pub confidence: f32,
    pub description: String,
}

#[derive(Debug, Serialize)]
pub struct MelodyProfileSummary {
    pub note_count: usize,
    pub distinct_pitch_classes: usize,
    pub ambitus: Ambitus,
    pub histogram: Vec<PitchClassBin>,
}

#[derive(Debug, Serialize)]
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
                let _ = writeln!(&mut out, "- {note}");
            }
        }
        out
    }
}

fn render_melody_side(out: &mut String, label: &str, profile: &MelodyProfileSummary) {
    let _ = writeln!(
        out,
        "{label}: {notes} notes, {pcs} unique pcs, ambitus {span} st.",
        label = label,
        notes = profile.note_count,
        pcs = profile.distinct_pitch_classes,
        span = profile.ambitus.span
    );
}

#[derive(Debug, Serialize)]
pub struct ProgressionProfileSummary {
    pub progression: Vec<String>,
    pub chord_count: usize,
    pub unique_chords: usize,
    pub function_counts: FunctionCounts,
    pub cadence: Option<CadenceSummary>,
}

#[derive(Debug, Serialize)]
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
        let _ = writeln!(
            &mut out,
            "Progression diff — shared {shared}, left unique {left}, right unique {right}.",
            shared = self.shared_chords.len(),
            left = self.left_unique.len(),
            right = self.right_unique.len()
        );
        if let Some(key) = &self.key_hint {
            let _ = writeln!(&mut out, "Context key: {key}.");
        }
        render_progression_side(&mut out, "Left", &self.left);
        render_progression_side(&mut out, "Right", &self.right);
        if !self.shared_chords.is_empty() {
            let _ = writeln!(
                &mut out,
                "Shared chords: {list}.",
                list = self.shared_chords.join(", ")
            );
        }
        if !self.left_unique.is_empty() {
            let _ = writeln!(
                &mut out,
                "Left-only chords: {list}.",
                list = self.left_unique.join(", ")
            );
        }
        if !self.right_unique.is_empty() {
            let _ = writeln!(
                &mut out,
                "Right-only chords: {list}.",
                list = self.right_unique.join(", ")
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

fn render_progression_side(out: &mut String, label: &str, profile: &ProgressionProfileSummary) {
    let _ = writeln!(
        out,
        "{label}: {unique}/{total} unique chords, tonic {tonic}, predominant {pred}, dominant {dom}, other {other}.",
        label = label,
        unique = profile.unique_chords,
        total = profile.chord_count,
        tonic = profile.function_counts.tonic,
        pred = profile.function_counts.predominant,
        dom = profile.function_counts.dominant,
        other = profile.function_counts.other
    );
    match &profile.cadence {
        Some(cadence) => {
            let _ = writeln!(
                out,
                "  Cadence: {pattern} ({desc}, conf {conf:.0}%).",
                pattern = cadence.pattern,
                desc = cadence.description,
                conf = cadence.confidence * 100.0
            );
        }
        None => {
            let _ = writeln!(out, "  Cadence: none detected.");
        }
    }
}

#[derive(Debug, Serialize)]
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

#[derive(Debug, Serialize, Clone)]
pub struct ScalePitchClassProjection {
    pub degree: usize,
    pub pitch_class: u8,
    pub index: i32,
    pub label: Option<String>,
    pub frequency_hz: f32,
}

#[derive(Debug, Serialize, Clone)]
pub struct PitchClassMapEntry {
    pub pitch_class: u8,
    pub occupied: bool,
    pub degree: Option<usize>,
    pub label: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ModulatoryPathSummary {
    pub rotation: usize,
    pub mode_name: Option<String>,
    pub root_index: i32,
    pub root_label: Option<String>,
    pub pivot_pitch_classes: Vec<u8>,
}

#[derive(Debug, Serialize)]
pub struct ScaleMapReport {
    pub system: String,
    pub root_index: i32,
    pub root_label: Option<String>,
    pub scale: String,
    pub members: Vec<ScalePitchClassProjection>,
    pub pitch_class_map: Vec<PitchClassMapEntry>,
    pub modulatory_paths: Vec<ModulatoryPathSummary>,
}

#[derive(Debug, Serialize, Clone)]
pub struct InterpolatedPoint {
    pub time: f32,
    pub value: f32,
}

#[derive(Debug, Serialize)]
pub struct InterpolationContext {
    pub curve: String,
    pub samples: usize,
}

#[derive(Debug, Serialize)]
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
pub struct VelocityEnvelopeReport {
    pub context: InterpolationContext,
    pub anchors: Vec<InterpolatedPoint>,
    pub samples: Vec<InterpolatedPoint>,
    pub min_value: i32,
    pub max_value: i32,
}

#[derive(Debug, Serialize)]
pub struct ScaleSearchReport {
    pub system: String,
    pub criteria: Vec<u8>,
    pub match_count: usize,
    pub matches: Vec<ScaleSearchMatch>,
}

#[derive(Debug, Serialize)]
pub struct ScaleSearchMatch {
    pub scale: String,
    pub root_index: i32,
    pub root_label: String,
}

impl ScaleSearchReport {
    pub fn render_text(&self) -> String {
        let mut out = String::new();
        let _ = writeln!(
            &mut out,
            "Scale search in {system}: {count} match(es) for pcs {pcs:?}.",
            system = self.system,
            count = self.match_count,
            pcs = self.criteria
        );
        for entry in &self.matches {
            let _ = writeln!(
                &mut out,
                "  - {scale} rooted at {label} ({index}).",
                scale = entry.scale,
                label = entry.root_label,
                index = entry.root_index
            );
        }
        out
    }
}

#[derive(Debug, Serialize)]
pub struct ChordSearchReport {
    pub system: String,
    pub criteria: Vec<u8>,
    pub voicing: String,
    pub match_count: usize,
    pub matches: Vec<ChordSearchMatch>,
}

#[derive(Debug, Serialize)]
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

impl ScaleMapReport {
    pub fn render_text(&self) -> String {
        let mut out = String::new();
        let root_label = self
            .root_label
            .as_deref()
            .map(|label| format!(" ({label})"))
            .unwrap_or_default();
        let _ = writeln!(
            &mut out,
            "Scale map for {scale} in {system}, root {root}{label}.",
            scale = self.scale,
            system = self.system,
            root = self.root_index,
            label = root_label
        );
        let _ = writeln!(&mut out, "Pitch-class layout:");
        for entry in &self.pitch_class_map {
            let marker = if entry.occupied { '●' } else { '·' };
            if let Some(degree) = entry.degree {
                let lbl = entry.label.as_deref().unwrap_or("—");
                let _ = writeln!(
                    &mut out,
                    "  pc {pc:>2}: {marker} degree {degree}, label {label}",
                    pc = entry.pitch_class,
                    marker = marker,
                    degree = degree,
                    label = lbl
                );
            } else {
                let _ = writeln!(
                    &mut out,
                    "  pc {pc:>2}: {marker}",
                    pc = entry.pitch_class,
                    marker = marker
                );
            }
        }
        if !self.modulatory_paths.is_empty() {
            let _ = writeln!(&mut out);
            let _ = writeln!(&mut out, "Modulatory paths:");
            for path in &self.modulatory_paths {
                let mode = path.mode_name.as_deref().unwrap_or("(unknown mode)");
                let pivots = if path.pivot_pitch_classes.is_empty() {
                    "none".into()
                } else {
                    format!("{:?}", path.pivot_pitch_classes)
                };
                let root_label = path
                    .root_label
                    .as_deref()
                    .map(|label| format!(" ({label})"))
                    .unwrap_or_default();
                let _ = writeln!(
                    &mut out,
                    "  rot {rot}: root {root}{label}, mode {mode}, pivot pcs {pivots}.",
                    rot = path.rotation,
                    root = path.root_index,
                    label = root_label,
                    mode = mode,
                    pivots = pivots
                );
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
            "Progression score: {total:.1}/100 ({unique} unique / {total_chords} total).",
            total = self.total_score,
            unique = self.unique_chords,
            total_chords = self.total_chords
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
            "Coverage {coverage:.0}% · Cadence {cadence:.0}% · Variety {variety:.0}%.",
            coverage = self.coverage_score * 100.0,
            cadence = self.cadence_score * 100.0,
            variety = self.variety_score * 100.0
        );
        match &self.cadence {
            Some(cadence) => {
                let _ = writeln!(
                    &mut out,
                    "Cadence detected: {pattern} ({desc}, confidence {conf:.0}%).",
                    pattern = cadence.pattern,
                    desc = cadence.description,
                    conf = cadence.confidence * 100.0
                );
            }
            None => {
                let _ = writeln!(&mut out, "Cadence detected: none.");
            }
        }
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
pub struct InvalidMelodyNote {
    pub position: usize,
    pub note_index: i32,
    pub pitch_class: u8,
}

#[derive(Debug, Serialize)]
pub struct LeapViolation {
    pub position: usize,
    pub from: i32,
    pub to: i32,
    pub interval: i32,
}

#[derive(Debug, Serialize)]
pub struct ProgressionValidationReport {
    pub progression: Vec<String>,
    pub chord_count: usize,
    pub key_hint: Option<String>,
    pub invalid_chords: Vec<InvalidProgressionToken>,
    pub duplicate_positions: Vec<usize>,
    pub function_counts: FunctionCounts,
    pub cadence: Option<CadenceSummary>,
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
pub struct InvalidProgressionToken {
    pub position: usize,
    pub token: String,
    pub normalized: String,
}

#[derive(Debug, Serialize)]
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
pub struct TuningSampleRow {
    pub index: i32,
    pub frequency_hz: f32,
    pub label: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct MonotonicViolation {
    pub lower_index: i32,
    pub lower_frequency_hz: f32,
    pub higher_index: i32,
    pub higher_frequency_hz: f32,
}

#[derive(Debug, Serialize)]
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
pub enum MidiConversionDirection {
    MidiToCsv,
    CsvToMidi,
}

#[derive(Clone, Debug, Serialize)]
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
pub struct MelodyPrediction {
    pub pitch_class: u8,
    pub suggested_index: i32,
    pub label: Option<String>,
    pub frequency_hz: f32,
    pub probability: f32,
}

#[derive(Debug, Serialize)]
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
pub struct ChordPrediction {
    pub chord: String,
    pub probability: f32,
}

#[derive(Debug, Serialize)]
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
pub struct PitchRangeReport {
    pub min_pitch: Option<u8>,
    pub max_pitch: Option<u8>,
    pub median_pitch: Option<f64>,
    pub p25_pitch: Option<f64>,
    pub p75_pitch: Option<f64>,
}

#[derive(Debug, Serialize)]
pub struct TimingReport {
    pub min_ioi_sec: Option<f64>,
    pub max_ioi_sec: Option<f64>,
    pub median_ioi_sec: Option<f64>,
    pub p25_ioi_sec: Option<f64>,
    pub p75_ioi_sec: Option<f64>,
    pub swing_ratio: Option<f64>,
}

#[derive(Debug, Serialize)]
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
