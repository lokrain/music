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
