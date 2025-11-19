use music_engine::prelude::TuningRegistry;
use music_engine::prelude::chord::DiatonicChord;
use serde::Serialize;
use std::fmt::Write as FmtWrite;

#[cfg(feature = "schema")]
use schemars::JsonSchema;

use crate::reports::pitch::PitchResult;

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
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
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct ChordDetails {
    pub degree: usize,
    pub numeral: String,
    pub quality: Option<String>,
    pub tones: Vec<ChordToneSummary>,
}

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct ChordToneSummary {
    pub index: usize,
    pub label: Option<String>,
    pub frequency_hz: f32,
}

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
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

fn roman_numeral(degree: usize) -> String {
    const ROMANS: [&str; 7] = ["I", "II", "III", "IV", "V", "VI", "VII"];
    ROMANS
        .get(degree % ROMANS.len())
        .copied()
        .unwrap_or("I")
        .to_string()
}
