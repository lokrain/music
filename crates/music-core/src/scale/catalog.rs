use crate::{pitch::AbstractPitch, registry::TuningRegistry, system::PitchSystemId};

use super::{Scale, ScaleBuildError, ScalePattern};

const IONIAN_STEPS: [i32; 7] = [2, 2, 1, 2, 2, 2, 1];
const DORIAN_STEPS: [i32; 7] = [2, 1, 2, 2, 2, 1, 2];
const PHRYGIAN_STEPS: [i32; 7] = [1, 2, 2, 2, 1, 2, 2];
const LYDIAN_STEPS: [i32; 7] = [2, 2, 2, 1, 2, 2, 1];
const MIXOLYDIAN_STEPS: [i32; 7] = [2, 2, 1, 2, 2, 1, 2];
const AEOLIAN_STEPS: [i32; 7] = [2, 1, 2, 2, 1, 2, 2];
const LOCRIAN_STEPS: [i32; 7] = [1, 2, 2, 1, 2, 2, 2];
const HARMONIC_MINOR_STEPS: [i32; 7] = [2, 1, 2, 2, 1, 3, 1];
const MELODIC_MINOR_STEPS: [i32; 7] = [2, 1, 2, 2, 2, 2, 1];

fn pattern_from_steps(
    steps: &[i32],
    system: &PitchSystemId,
    registry: &TuningRegistry,
) -> Result<ScalePattern, ScaleBuildError> {
    ScalePattern::from_twelve_tet_steps(steps, system, registry)
}

fn scale_from_steps(
    root_index: i32,
    steps: &[i32],
    system: &PitchSystemId,
    registry: &TuningRegistry,
) -> Result<Scale, ScaleBuildError> {
    let pattern = pattern_from_steps(steps, system, registry)?;
    Ok(Scale::from_abstract_root(
        AbstractPitch::new(root_index, system.clone()),
        pattern,
    ))
}

macro_rules! pattern_fn {
    ($name:ident, $steps:ident, $pattern_fn:ident, $scale_fn:ident) => {
        #[doc = concat!(
                                            "Build the ",
                                            stringify!($name),
                                            " scale pattern in twelve-tone equal temperament."
                                        )]
        ///
        /// # Errors
        ///
        /// Returns [`ScaleBuildError`] if the pattern cannot be constructed for the provided system.
        pub fn $pattern_fn(
            system: &PitchSystemId,
            registry: &TuningRegistry,
        ) -> Result<ScalePattern, ScaleBuildError> {
            pattern_from_steps(&$steps, system, registry)
        }

        #[doc = concat!("Build the ", stringify!($name), " scale rooted at `root_index`.")]
        ///
        /// # Errors
        ///
        /// Returns [`ScaleBuildError`] if the pattern cannot be constructed or the registry lacks
        /// the requested system.
        pub fn $scale_fn(
            root_index: i32,
            system: &PitchSystemId,
            registry: &TuningRegistry,
        ) -> Result<Scale, ScaleBuildError> {
            scale_from_steps(root_index, &$steps, system, registry)
        }
    };
}

/// Alias for [`ionian_pattern`]; provided for ergonomics when working with major keys.
///
/// # Errors
///
/// Returns [`ScaleBuildError`] if the pattern cannot be constructed for the provided system.
pub fn major_pattern(
    system: &PitchSystemId,
    registry: &TuningRegistry,
) -> Result<ScalePattern, ScaleBuildError> {
    ionian_pattern(system, registry)
}

/// Alias for [`ionian_scale`]; provided for ergonomics when working with major keys.
///
/// # Errors
///
/// Returns [`ScaleBuildError`] if the pattern cannot be constructed or the registry lacks the
/// requested system.
pub fn major_scale(
    root_index: i32,
    system: &PitchSystemId,
    registry: &TuningRegistry,
) -> Result<Scale, ScaleBuildError> {
    ionian_scale(root_index, system, registry)
}

/// Alias for [`aeolian_pattern`]; exposed as both `minor` and `natural_minor` for clarity.
///
/// # Errors
///
/// Returns [`ScaleBuildError`] if the pattern cannot be constructed for the provided system.
pub fn minor_pattern(
    system: &PitchSystemId,
    registry: &TuningRegistry,
) -> Result<ScalePattern, ScaleBuildError> {
    aeolian_pattern(system, registry)
}

/// Alias for [`aeolian_pattern`]; provided when explicitly requesting the natural minor scale.
///
/// # Errors
///
/// Returns [`ScaleBuildError`] if the pattern cannot be constructed for the provided system.
pub fn natural_minor_pattern(
    system: &PitchSystemId,
    registry: &TuningRegistry,
) -> Result<ScalePattern, ScaleBuildError> {
    aeolian_pattern(system, registry)
}

/// Alias for [`aeolian_scale`]; provided for ergonomics when working with minor keys.
///
/// # Errors
///
/// Returns [`ScaleBuildError`] if the pattern cannot be constructed or the registry lacks the
/// requested system.
pub fn minor_scale(
    root_index: i32,
    system: &PitchSystemId,
    registry: &TuningRegistry,
) -> Result<Scale, ScaleBuildError> {
    aeolian_scale(root_index, system, registry)
}

/// Alias for [`aeolian_scale`]; explicitly names the natural minor scale.
///
/// # Errors
///
/// Returns [`ScaleBuildError`] if the pattern cannot be constructed or the registry lacks the
/// requested system.
pub fn natural_minor_scale(
    root_index: i32,
    system: &PitchSystemId,
    registry: &TuningRegistry,
) -> Result<Scale, ScaleBuildError> {
    aeolian_scale(root_index, system, registry)
}

pattern_fn!(ionian, IONIAN_STEPS, ionian_pattern, ionian_scale);
pattern_fn!(dorian, DORIAN_STEPS, dorian_pattern, dorian_scale);
pattern_fn!(phrygian, PHRYGIAN_STEPS, phrygian_pattern, phrygian_scale);
pattern_fn!(lydian, LYDIAN_STEPS, lydian_pattern, lydian_scale);
pattern_fn!(
    mixolydian,
    MIXOLYDIAN_STEPS,
    mixolydian_pattern,
    mixolydian_scale
);
pattern_fn!(aeolian, AEOLIAN_STEPS, aeolian_pattern, aeolian_scale);
pattern_fn!(locrian, LOCRIAN_STEPS, locrian_pattern, locrian_scale);
pattern_fn!(
    harmonic_minor,
    HARMONIC_MINOR_STEPS,
    harmonic_minor_pattern,
    harmonic_minor_scale
);
pattern_fn!(
    melodic_minor,
    MELODIC_MINOR_STEPS,
    melodic_minor_pattern,
    melodic_minor_scale
);
