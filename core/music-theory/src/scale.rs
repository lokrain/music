//! core/music-theory/src/scale.rs
//! Scale definitions and helpers.
//!
//! Scales are ordered collections of [`PitchClass`] values generated from step patterns. Keys
//! use scales to derive degree membership, chords reference scales to ensure their tones fit,
//! and theory utilities rely on degree lookups when labeling harmonic functions.

use crate::interval::Interval;
use crate::pitch::{PitchClass, PitchClass12};
use music_acoustic::{T12, Temperament};

/// Scale as an ordered set of pitch-classes in a temperament.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Scale<T: Temperament> {
    pub degrees: Vec<PitchClass<T>>,
}

impl<T: Temperament> Scale<T> {
    #[must_use]
    pub fn new(degrees: Vec<PitchClass<T>>) -> Self {
        Self { degrees }
    }

    /// Build a scale from a step pattern (intervals between successive degrees).
    #[must_use]
    pub fn from_step_pattern(root: PitchClass<T>, steps: &[i32]) -> Self {
        let mut degrees = Vec::with_capacity(steps.len() + 1);
        degrees.push(root);
        let mut current = root;
        for &step in steps {
            current = current.transpose(Interval::new(step));
            if degrees.contains(&current) {
                continue;
            }
            degrees.push(current);
        }
        Self::new(degrees)
    }

    /// Number of degrees in the scale.
    #[must_use]
    pub fn len(&self) -> usize {
        self.degrees.len()
    }

    /// Check whether the scale is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.degrees.is_empty()
    }

    /// Test whether a pitch-class is part of the scale.
    #[must_use]
    pub fn contains(&self, pitch_class: PitchClass<T>) -> bool {
        self.degrees.contains(&pitch_class)
    }

    /// Retrieve the 1-indexed scale degree for the given pitch-class.
    #[must_use]
    pub fn degree_of(&self, pitch_class: PitchClass<T>) -> Option<usize> {
        self.degrees.iter().position(|pc| *pc == pitch_class).map(|idx| idx + 1)
    }

    /// Generate the `degree`-th mode (1-indexed) by rotating degrees.
    #[must_use]
    pub fn mode(&self, degree: usize) -> Option<Self> {
        if degree == 0 || degree > self.degrees.len() {
            return None;
        }
        let mut rotated = self.degrees[degree - 1..].to_vec();
        rotated.extend_from_slice(&self.degrees[..degree - 1]);
        Some(Self::new(rotated))
    }
}

/// Convenience alias for 12-TET scales.
pub type Scale12 = Scale<T12>;

impl Scale<T12> {
    /// Ionian (major) scale from root.
    #[must_use]
    pub fn major(root: PitchClass12) -> Self {
        Self::from_step_pattern(root, &[2, 2, 1, 2, 2, 2, 1])
    }

    /// Aeolian (natural minor) scale from root.
    #[must_use]
    pub fn natural_minor(root: PitchClass12) -> Self {
        Self::from_step_pattern(root, &[2, 1, 2, 2, 1, 2, 2])
    }

    #[must_use]
    pub fn dorian(root: PitchClass12) -> Self {
        Self::from_step_pattern(root, &[2, 1, 2, 2, 2, 1, 2])
    }

    #[must_use]
    pub fn phrygian(root: PitchClass12) -> Self {
        Self::from_step_pattern(root, &[1, 2, 2, 2, 1, 2, 2])
    }

    #[must_use]
    pub fn lydian(root: PitchClass12) -> Self {
        Self::from_step_pattern(root, &[2, 2, 2, 1, 2, 2, 1])
    }

    #[must_use]
    pub fn mixolydian(root: PitchClass12) -> Self {
        Self::from_step_pattern(root, &[2, 2, 1, 2, 2, 1, 2])
    }

    #[must_use]
    pub fn locrian(root: PitchClass12) -> Self {
        Self::from_step_pattern(root, &[1, 2, 2, 1, 2, 2, 2])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pcs(indices: &[u16]) -> Vec<PitchClass12> {
        indices.iter().map(|&idx| PitchClass12::from_semitones(idx)).collect()
    }

    #[test]
    fn major_scale_contains_expected_degrees() {
        let scale = Scale12::major(PitchClass12::from_semitones(0));
        let expected = pcs(&[0, 2, 4, 5, 7, 9, 11]);
        assert_eq!(scale.degrees, expected);
        for (degree, pc) in expected.iter().enumerate() {
            assert!(scale.contains(*pc));
            assert_eq!(scale.degree_of(*pc), Some(degree + 1));
        }
        assert!(scale.degree_of(PitchClass12::from_semitones(1)).is_none());
    }

    #[test]
    fn natural_minor_matches_mode_of_major() {
        let c_major = Scale12::major(PitchClass12::from_semitones(0));
        let a_minor = Scale12::natural_minor(PitchClass12::from_semitones(9));
        let mode = c_major.mode(6).expect("sixth mode exists");
        assert_eq!(a_minor.degrees, mode.degrees);
    }

    #[test]
    fn dorian_factory_matches_step_pattern() {
        let d = PitchClass12::from_semitones(2);
        let dorian = Scale12::dorian(d);
        let manual = Scale12::from_step_pattern(d, &[2, 1, 2, 2, 2, 1, 2]);
        assert_eq!(dorian.degrees, manual.degrees);
    }

    #[test]
    fn mode_generation_rotates_degrees() {
        let lydian = Scale12::lydian(PitchClass12::from_semitones(5));
        let start_degree = 3;
        let mode = lydian.mode(start_degree).expect("mode exists");
        let expected_root = lydian.degrees[2];
        assert_eq!(mode.degrees[0], expected_root);
        let invert_shift = lydian.len() - (start_degree - 1);
        let recomposed = mode.mode(invert_shift + 1).expect("rotate back");
        assert_eq!(recomposed.degrees, lydian.degrees);
    }
}
