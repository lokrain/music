//! core/music-theory/src/pitch.rs
//! Temperament-aware pitch and pitch-class definitions.
//!
//! [`PitchClass`] values are the atomic musical alphabet. They retain their temperament context,
//! so helper APIs like [`PitchClass::transpose`] and [`Pitch::transpose`] work with any tuning
//! that implements [`music_acoustic::Temperament`]. Higher-level modules (`scale`, `chord`,
//! `key`) all operate on these primitives.

use core::cmp::Ordering;
use core::hash::{Hash, Hasher};
use core::marker::PhantomData;

use music_acoustic::{T12, Temperament};

use crate::interval::Interval;

/// Pitch-class in a given temperament.
#[derive(Debug)]
pub struct PitchClass<T: Temperament> {
    index: u16,
    _marker: PhantomData<T>,
}

impl<T: Temperament> PitchClass<T> {
    #[must_use]
    pub fn new(index: u16) -> Self {
        Self { index: index % T::STEPS_PER_OCTAVE, _marker: PhantomData }
    }

    #[must_use]
    pub fn index(&self) -> u16 {
        self.index
    }

    /// Transpose this pitch-class by the given interval.
    #[must_use]
    pub fn transpose(self, interval: Interval<T>) -> Self {
        let steps = i32::from(T::STEPS_PER_OCTAVE);
        let total = i32::from(self.index) + interval.steps();
        let wrapped = total.rem_euclid(steps);
        Self::new(wrapped as u16)
    }
}

impl<T: Temperament> PartialEq for PitchClass<T> {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index
    }
}

impl<T: Temperament> Eq for PitchClass<T> {}

impl<T: Temperament> Hash for PitchClass<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.index.hash(state);
    }
}

impl<T: Temperament> Copy for PitchClass<T> {}

impl<T: Temperament> Clone for PitchClass<T> {
    fn clone(&self) -> Self {
        *self
    }
}

/// Absolute pitch (pc + octave) in a given temperament.
#[derive(Debug)]
pub struct Pitch<T: Temperament> {
    pub pitch_class: PitchClass<T>,
    pub octave: i16,
}

impl<T: Temperament> Pitch<T> {
    #[must_use]
    pub fn new(pitch_class: PitchClass<T>, octave: i16) -> Self {
        Self { pitch_class, octave }
    }

    /// Transpose the pitch by a temperament-specific interval.
    #[must_use]
    pub fn transpose(self, interval: Interval<T>) -> Self {
        let steps_per_octave = i32::from(T::STEPS_PER_OCTAVE);
        let total = i32::from(self.pitch_class.index()) + interval.steps();
        let wrapped = total.rem_euclid(steps_per_octave);
        let octave_delta = (total - wrapped) / steps_per_octave;
        let pitch_class = PitchClass::new(wrapped as u16);
        let octave = self
            .octave
            .checked_add(i16::try_from(octave_delta).expect("octave delta exceeds i16 range"))
            .expect("octave overflow during transposition");
        Self { pitch_class, octave }
    }

    /// Shift the pitch by the provided octave count (positive or negative).
    #[must_use]
    pub fn shift_octaves(self, octaves: i16) -> Self {
        Self { octave: self.octave + octaves, ..self }
    }

    /// Convenience helper to raise the pitch by one octave.
    #[must_use]
    pub fn octave_up(self) -> Self {
        self.shift_octaves(1)
    }

    /// Convenience helper to lower the pitch by one octave.
    #[must_use]
    pub fn octave_down(self) -> Self {
        self.shift_octaves(-1)
    }
}

impl<T: Temperament> PartialEq for Pitch<T> {
    fn eq(&self, other: &Self) -> bool {
        self.pitch_class == other.pitch_class && self.octave == other.octave
    }
}

impl<T: Temperament> Eq for Pitch<T> {}

impl<T: Temperament> Hash for Pitch<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.pitch_class.hash(state);
        self.octave.hash(state);
    }
}

impl<T: Temperament> Copy for Pitch<T> {}

impl<T: Temperament> Clone for Pitch<T> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<T: Temperament> Ord for Pitch<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.octave.cmp(&other.octave) {
            Ordering::Equal => self.pitch_class.index().cmp(&other.pitch_class.index()),
            result => result,
        }
    }
}

impl<T: Temperament> PartialOrd for Pitch<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// 12-TET pitch-class alias.
pub type PitchClass12 = PitchClass<T12>;

/// 12-TET pitch alias.
pub type Pitch12 = Pitch<T12>;

impl PitchClass<T12> {
    /// Construct a 12-TET pitch-class from semitone index.
    #[must_use]
    pub fn from_semitones(semitones: u16) -> Self {
        Self::new(semitones)
    }

    /// Retrieve the semitone index (0–11).
    #[must_use]
    pub fn to_semitones(self) -> u16 {
        self.index()
    }
}

impl Pitch<T12> {
    /// Construct a pitch from semitone index + octave number.
    #[must_use]
    pub fn from_semitones_and_octave(semitones: u16, octave: i16) -> Self {
        Self::new(PitchClass12::from_semitones(semitones), octave)
    }

    /// Semitone component within the octave.
    #[must_use]
    pub fn semitone(&self) -> u16 {
        self.pitch_class.to_semitones()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interval::Interval12;

    #[test]
    fn pitch_class_transposition_wraps() {
        let c = PitchClass12::from_semitones(0);
        let g = c.transpose(Interval12::from_semitones(7));
        assert_eq!(g.to_semitones(), 7);

        let back = g.transpose(Interval12::from_semitones(-7));
        assert_eq!(back.to_semitones(), 0);

        let b = c.transpose(Interval12::from_semitones(-1));
        assert_eq!(b.to_semitones(), 11);
    }

    #[test]
    fn pitch_transposition_updates_octaves() {
        let c4 = Pitch12::from_semitones_and_octave(0, 4);
        let d5 = c4.transpose(Interval12::from_semitones(14));
        assert_eq!(d5.semitone(), 2);
        assert_eq!(d5.octave, 5);

        let back = d5.transpose(Interval12::from_semitones(-14));
        assert_eq!(back.semitone(), 0);
        assert_eq!(back.octave, 4);
    }

    #[test]
    fn octave_shifts_and_ordering() {
        let g4 = Pitch12::from_semitones_and_octave(7, 4);
        let g5 = g4.octave_up();
        let g3 = g4.octave_down();
        assert_eq!(g5.octave, 5);
        assert_eq!(g3.octave, 3);

        assert!(g3 < g4);
        assert!(g4 < g5);
    }
}
