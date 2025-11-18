use alloc::vec::Vec;

use crate::{TuningRegistry, interval::Interval, scale::Scale};

use super::{
    errors::ChordDiatonicError,
    implementation::Chord,
    pattern::{ChordPattern, ChordPatternError},
    quality::ChordQuality,
};

/// Result of stacking scale degrees to form a diatonic chord.
#[derive(Debug, Clone)]
pub struct DiatonicChord {
    pub degree: usize,
    pub chord: Chord,
    pub quality: Option<ChordQuality>,
}

/// Build a diatonic triad by stacking thirds within the provided scale.
///
/// # Errors
///
/// Returns [`ChordDiatonicError`] when the scale cannot resolve the requested degrees or when
/// chord construction fails.
pub fn diatonic_triad(
    scale: &Scale,
    degree: usize,
    registry: &TuningRegistry,
) -> Result<DiatonicChord, ChordDiatonicError> {
    build_diatonic(scale, degree, 3, registry)
}

/// Build a diatonic seventh chord by stacking four consecutive thirds within the provided scale.
///
/// # Errors
///
/// Returns [`ChordDiatonicError`] when the scale cannot resolve the requested degrees or when
/// chord construction fails.
pub fn diatonic_seventh(
    scale: &Scale,
    degree: usize,
    registry: &TuningRegistry,
) -> Result<DiatonicChord, ChordDiatonicError> {
    build_diatonic(scale, degree, 4, registry)
}

/// Build every diatonic triad for the provided scale.
///
/// # Errors
///
/// Returns [`ChordDiatonicError`] when the scale pattern is empty or resolving a degree fails.
pub fn diatonic_triads(
    scale: &Scale,
    registry: &TuningRegistry,
) -> Result<Vec<DiatonicChord>, ChordDiatonicError> {
    collect_all(scale, registry, 3)
}

/// Build every diatonic seventh chord for the provided scale.
///
/// # Errors
///
/// Returns [`ChordDiatonicError`] when the scale pattern is empty or resolving a degree fails.
pub fn diatonic_sevenths(
    scale: &Scale,
    registry: &TuningRegistry,
) -> Result<Vec<DiatonicChord>, ChordDiatonicError> {
    collect_all(scale, registry, 4)
}

fn collect_all(
    scale: &Scale,
    registry: &TuningRegistry,
    tone_count: usize,
) -> Result<Vec<DiatonicChord>, ChordDiatonicError> {
    let step_count = scale.step_count();
    if step_count == 0 {
        return Err(ChordPatternError::EmptyPattern.into());
    }

    let mut chords = Vec::with_capacity(step_count);
    for degree in 0..step_count {
        chords.push(build_diatonic(scale, degree, tone_count, registry)?);
    }
    Ok(chords)
}

fn build_diatonic(
    scale: &Scale,
    degree: usize,
    tone_count: usize,
    registry: &TuningRegistry,
) -> Result<DiatonicChord, ChordDiatonicError> {
    let step_count = scale.step_count();
    if step_count == 0 {
        return Err(ChordPatternError::EmptyPattern.into());
    }
    if tone_count == 0 {
        return Err(ChordPatternError::EmptyPattern.into());
    }

    let reduced_degree = degree % step_count;
    let root = scale.degree_pitch(reduced_degree, registry)?;

    let mut intervals = Vec::with_capacity(tone_count);
    intervals.push(Interval::identity());

    for idx in 1..tone_count {
        let target_degree = reduced_degree + idx * 2;
        let target = scale.degree_pitch(target_degree, registry)?;
        let interval = Interval::between(&root, &target, registry)?;
        intervals.push(interval);
    }

    let pattern = ChordPattern::from_intervals(intervals)?;
    let chord = Chord::from_pitch(root, pattern);
    let quality = ChordQuality::classify(chord.pattern());

    Ok(DiatonicChord {
        degree: reduced_degree,
        chord,
        quality,
    })
}
