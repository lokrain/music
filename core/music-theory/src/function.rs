//! core/music-theory/src/function.rs
//! Harmonic function representation for tonal analysis.
//!
//! [`HarmonicFunction`] instances are derived from [`crate::key::Key`] degree lookups and describe
//! how a pitch class (and by extension, its chords) behaves inside a tonal context: tonic,
//! predominant/subdominant, dominant, etc.

use music_acoustic::Temperament;

use crate::key::Key;
use crate::pitch::PitchClass;

/// High-level tonal function kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FunctionKind {
    Tonic,
    Subdominant,
    Dominant,
    Mediant,
    Submediant,
    Supertonic,
    LeadingTone,
}

/// Harmonic function for a scale degree in a key.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HarmonicFunction<T: Temperament> {
    /// Descriptive bucket (tonic, dominant, etc.).
    pub kind: FunctionKind,
    /// Scale degree (1–7 for heptatonic modes).
    pub degree: u8,
    pub key: Key<T>,
    pub pitch_class: PitchClass<T>,
}

impl<T: Temperament> HarmonicFunction<T> {
    /// Construct a new harmonic function descriptor.
    #[must_use]
    pub fn new(kind: FunctionKind, degree: u8, key: Key<T>, pitch_class: PitchClass<T>) -> Self {
        Self { kind, degree, key, pitch_class }
    }
}
