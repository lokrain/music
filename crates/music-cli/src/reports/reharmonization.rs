//! Reharmonization listing report types.

use std::fmt::Write as FmtWrite;

use music_engine::prelude::TuningRegistry;
use serde::Serialize;

#[cfg(feature = "schema")]
use schemars::JsonSchema;

use crate::reports::chord::ChordDetails;
use crate::reports::pitch::PitchResult;
use music_engine::prelude::chord::DiatonicChord;

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

/// Construct a borrowed chord summary with details and root information.
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

    let details = crate::responses::summarize_chord(chord, registry)?;

    Ok(BorrowedChord {
        details,
        root_index,
        root_label,
        root_frequency_hz,
        matches_target,
    })
}
