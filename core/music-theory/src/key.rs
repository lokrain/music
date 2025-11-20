//! core/music-theory/src/key.rs
//! Key, mode, and tonal function helpers tying together pitch, scale, and chords.
//!
//! Keys wrap a tonic [`PitchClass`] and a [`Mode`], expose the derived diatonic [`Scale`], and
//! produce [`crate::function::HarmonicFunction`] descriptors for any diatonic degree.
//! They provide the glue for mapping melodic/harmonic elements to tonal roles.
//!
//! ```
//! use music_theory::{FunctionKind, Key12, PitchClass12};
//! let key = Key12::minor(PitchClass12::from_semitones(9)); // A minor
//! let dominant = key.function_for_degree(5).unwrap();
//! assert_eq!(dominant.kind, FunctionKind::Dominant);
//! assert_eq!(dominant.pitch_class, PitchClass12::from_semitones(4));
//! ```

use crate::function::{FunctionKind, HarmonicFunction};
use crate::pitch::{PitchClass, PitchClass12};
use crate::scale::{Scale, Scale12};
use music_acoustic::{T12, Temperament};

/// Mode descriptor (major/minor and extensions later).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Mode {
    Major,
    Minor,
}

impl Mode {
    #[must_use]
    pub const fn step_pattern(self) -> &'static [i32; 7] {
        match self {
            Self::Major => &[2, 2, 1, 2, 2, 2, 1],
            Self::Minor => &[2, 1, 2, 2, 1, 2, 2],
        }
    }

    #[must_use]
    pub const fn function_kinds(self) -> &'static [FunctionKind; 7] {
        match self {
            Self::Major => &[
                FunctionKind::Tonic,
                FunctionKind::Supertonic,
                FunctionKind::Mediant,
                FunctionKind::Subdominant,
                FunctionKind::Dominant,
                FunctionKind::Submediant,
                FunctionKind::LeadingTone,
            ],
            Self::Minor => &[
                FunctionKind::Tonic,
                FunctionKind::Supertonic,
                FunctionKind::Mediant,
                FunctionKind::Subdominant,
                FunctionKind::Dominant,
                FunctionKind::Submediant,
                FunctionKind::LeadingTone,
            ],
        }
    }

    #[must_use]
    pub const fn function_kind(self, degree: u8) -> Option<FunctionKind> {
        if degree == 0 || degree > 7 {
            return None;
        }
        Some(self.function_kinds()[(degree - 1) as usize])
    }
}

/// Tonal key in a temperament.
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Key<T: Temperament> {
    pub tonic: PitchClass<T>,
    pub mode: Mode,
}

impl<T: Temperament> Copy for Key<T> {}

impl<T: Temperament> Clone for Key<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: Temperament> Key<T> {
    #[must_use]
    pub fn new(tonic: PitchClass<T>, mode: Mode) -> Self {
        Self { tonic, mode }
    }

    /// Return the heptatonic scale associated with this key.
    #[must_use]
    pub fn scale(&self) -> Scale<T> {
        Scale::from_step_pattern(self.tonic, self.mode.step_pattern())
    }

    /// Retrieve a pitch-class for the given 1-indexed scale degree.
    #[must_use]
    pub fn degree_pitch_class(&self, degree: u8) -> Option<PitchClass<T>> {
        if degree == 0 {
            return None;
        }
        self.scale().degrees.get((degree - 1) as usize).copied()
    }

    /// Construct a harmonic function for the provided degree.
    #[must_use]
    pub fn function_for_degree(&self, degree: u8) -> Option<HarmonicFunction<T>> {
        let kind = self.mode.function_kind(degree)?;
        let pitch_class = self.degree_pitch_class(degree)?;
        Some(HarmonicFunction::new(kind, degree, *self, pitch_class))
    }

    /// Look up the harmonic function for a pitch-class, if it belongs to the key.
    #[must_use]
    pub fn function_for_pitch_class(
        &self,
        pitch_class: PitchClass<T>,
    ) -> Option<HarmonicFunction<T>> {
        let scale = self.scale();
        let degree = scale.degree_of(pitch_class)? as u8;
        let kind = self.mode.function_kind(degree)?;
        Some(HarmonicFunction::new(kind, degree, *self, pitch_class))
    }
}

pub type Key12 = Key<T12>;

impl Key12 {
    #[must_use]
    pub fn major(tonic: PitchClass12) -> Self {
        Self::new(tonic, Mode::Major)
    }

    #[must_use]
    pub fn minor(tonic: PitchClass12) -> Self {
        Self::new(tonic, Mode::Minor)
    }

    #[must_use]
    pub fn scale12(&self) -> Scale12 {
        Scale12::from_step_pattern(self.tonic, self.mode.step_pattern())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pc(semitone: u16) -> PitchClass12 {
        PitchClass12::from_semitones(semitone)
    }

    #[test]
    fn c_major_functions() {
        let key = Key12::major(pc(0));
        let tonic = key.function_for_degree(1).expect("tonic exists");
        assert_eq!(tonic.kind, FunctionKind::Tonic);
        assert_eq!(tonic.pitch_class, pc(0));

        let dominant = key.function_for_degree(5).expect("dominant exists");
        assert_eq!(dominant.kind, FunctionKind::Dominant);
        assert_eq!(dominant.pitch_class, pc(7));

        let mediant = key.function_for_pitch_class(pc(4)).expect("E is mediant");
        assert_eq!(mediant.degree, 3);
        assert_eq!(mediant.kind, FunctionKind::Mediant);
    }

    #[test]
    fn a_minor_functions() {
        let key = Key12::minor(pc(9));
        let subdominant = key.function_for_degree(4).expect("exists");
        assert_eq!(subdominant.pitch_class, pc(2));
        assert_eq!(subdominant.kind, FunctionKind::Subdominant);

        let leading = key.function_for_pitch_class(pc(7)).expect("G belongs to key");
        assert_eq!(leading.kind, FunctionKind::LeadingTone);
        assert_eq!(leading.degree, 7);
    }
}
