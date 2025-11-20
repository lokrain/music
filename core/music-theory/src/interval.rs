//! core/music-theory/src/interval.rs
//! Interval definitions and basic operations.
//!
//! Intervals describe relative motion between pitch classes and drive most melodic/harmonic
//! transformations. Scales are defined by interval step patterns, chords are built by stacking
//! intervals above a root, and keys/function analysis maps degrees via interval arithmetic.

use core::ops::{Add, Sub};

use music_acoustic::{T12, Temperament};

/// Generic interval in a temperament: step distance.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Interval<T: Temperament> {
    steps: i32,
    _marker: core::marker::PhantomData<T>,
}

impl<T: Temperament> Interval<T> {
    /// Construct an interval from a step distance (signed).
    #[must_use]
    pub fn new(steps: i32) -> Self {
        Self { steps, _marker: core::marker::PhantomData }
    }

    /// Raw step distance.
    #[must_use]
    pub const fn steps(&self) -> i32 {
        self.steps
    }

    /// Invert the interval within a single octave of the given temperament.
    #[must_use]
    pub fn invert_octave(self) -> Self {
        let octave = i32::from(T::STEPS_PER_OCTAVE);
        Self::new((-self.steps).rem_euclid(octave))
    }
}

impl<T: Temperament> Add for Interval<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.steps + rhs.steps)
    }
}

impl<T: Temperament> Sub for Interval<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.steps - rhs.steps)
    }
}

/// Abstract generic interval class (2nd, 3rd, etc.).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GenericInterval {
    Unison,
    Second,
    Third,
    Fourth,
    Fifth,
    Sixth,
    Seventh,
    Octave,
}

/// Interval quality in tonal 12-TET.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IntervalQuality {
    Perfect,
    Major,
    Minor,
    Augmented,
    Diminished,
}

/// Convenience alias for 12-TET intervals.
pub type Interval12 = Interval<T12>;

impl Interval12 {
    /// Construct a 12-TET interval from semitone distance.
    #[must_use]
    pub fn from_semitones(semitones: i32) -> Self {
        Self::new(semitones)
    }

    /// Semitone distance of this interval.
    #[must_use]
    pub const fn to_semitones(self) -> i32 {
        self.steps
    }

    /// Classify the interval (within an octave) into generic interval + quality.
    ///
    /// Returns `None` for values outside the octave lookup table.
    #[must_use]
    pub fn classify(self) -> Option<(GenericInterval, IntervalQuality)> {
        let semis = self.to_semitones();
        if semis % 12 == 0 && semis != 0 {
            return Some((GenericInterval::Octave, IntervalQuality::Perfect));
        }
        let semis = semis.rem_euclid(12);
        match semis {
            0 => Some((GenericInterval::Unison, IntervalQuality::Perfect)),
            1 => Some((GenericInterval::Second, IntervalQuality::Minor)),
            2 => Some((GenericInterval::Second, IntervalQuality::Major)),
            3 => Some((GenericInterval::Third, IntervalQuality::Minor)),
            4 => Some((GenericInterval::Third, IntervalQuality::Major)),
            5 => Some((GenericInterval::Fourth, IntervalQuality::Perfect)),
            6 => Some((GenericInterval::Fourth, IntervalQuality::Augmented)),
            7 => Some((GenericInterval::Fifth, IntervalQuality::Perfect)),
            8 => Some((GenericInterval::Sixth, IntervalQuality::Minor)),
            9 => Some((GenericInterval::Sixth, IntervalQuality::Major)),
            10 => Some((GenericInterval::Seventh, IntervalQuality::Minor)),
            11 => Some((GenericInterval::Seventh, IntervalQuality::Major)),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classify_all_simple_intervals() {
        let cases = [
            (0, Some((GenericInterval::Unison, IntervalQuality::Perfect))),
            (1, Some((GenericInterval::Second, IntervalQuality::Minor))),
            (2, Some((GenericInterval::Second, IntervalQuality::Major))),
            (3, Some((GenericInterval::Third, IntervalQuality::Minor))),
            (4, Some((GenericInterval::Third, IntervalQuality::Major))),
            (5, Some((GenericInterval::Fourth, IntervalQuality::Perfect))),
            (6, Some((GenericInterval::Fourth, IntervalQuality::Augmented))),
            (7, Some((GenericInterval::Fifth, IntervalQuality::Perfect))),
            (8, Some((GenericInterval::Sixth, IntervalQuality::Minor))),
            (9, Some((GenericInterval::Sixth, IntervalQuality::Major))),
            (10, Some((GenericInterval::Seventh, IntervalQuality::Minor))),
            (11, Some((GenericInterval::Seventh, IntervalQuality::Major))),
            (12, Some((GenericInterval::Octave, IntervalQuality::Perfect))),
        ];
        for (semitones, expected) in cases {
            assert_eq!(
                Interval12::from_semitones(semitones).classify(),
                expected,
                "failed for {semitones} semitone(s)"
            );
        }
    }

    #[test]
    fn compose_and_invert_intervals() {
        let major_third = Interval12::from_semitones(4);
        let minor_third = Interval12::from_semitones(3);
        let perfect_fifth = Interval12::from_semitones(7);
        assert_eq!((major_third + minor_third).to_semitones(), perfect_fifth.to_semitones());
        assert_eq!((perfect_fifth - major_third).to_semitones(), minor_third.to_semitones());

        let inverted = perfect_fifth.invert_octave();
        assert_eq!(inverted.to_semitones(), 5);
    }
}
