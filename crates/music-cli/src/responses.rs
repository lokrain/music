use chord::DiatonicChord;
use music_engine::prelude::{PitchError, TuningRegistry, chord};
// Re-export modularized report types
pub use crate::reports::analysis::*;
pub use crate::reports::chord::*;
pub use crate::reports::conversion::*;
#[cfg(feature = "schema")]
pub use crate::reports::diff::*;
pub use crate::reports::estimate::*;
pub use crate::reports::extrapolation::*;
pub use crate::reports::generation::*;
pub use crate::reports::interpolation::*;
pub use crate::reports::pitch::*;
pub use crate::reports::profile::*;
pub use crate::reports::reharmonization::*;
pub use crate::reports::render::*;
pub use crate::reports::resolution::*;
pub use crate::reports::scale::*;
pub use crate::reports::scoring::*;
pub use crate::reports::search::*;
pub type PitchResult<T> = Result<T, PitchError>;

// Pitch-related types have moved to reports/pitch.rs

// Chord-related report types moved to reports/chord.rs

// Generation-related report types moved to reports/generation.rs
// Interpolation types moved to reports/interpolation.rs
// Chord search types moved to reports/search.rs
// Reharmonization types moved to reports/reharmonization.rs

// Scoring reports moved to reports/scoring.rs
// Rendering reports moved to reports/render.rs

// Validation reports moved to reports/validation.rs
// Conversion reports moved to reports/conversion.rs

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

// collect_mode_pitches and summarize_mode_pitch moved to reports/scale.rs
// borrowed_chord moved to reports/reharmonization.rs

fn roman_numeral(degree: usize) -> String {
    const ROMANS: [&str; 7] = ["I", "II", "III", "IV", "V", "VI", "VII"];
    ROMANS
        .get(degree % ROMANS.len())
        .copied()
        .unwrap_or("I")
        .to_string()
}

// Melody extrapolation types moved to reports/extrapolation.rs

// Chord extrapolation types moved to reports/extrapolation.rs

// Profile, PitchRange, and Timing report types moved to reports/profile.rs

// EstimateReport moved to reports/profile.rs
