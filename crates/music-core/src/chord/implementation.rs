use alloc::vec::Vec;

use crate::{
    TuningRegistry,
    pitch::{AbstractPitch, Pitch, PitchError},
    system::PitchSystemId,
};

use super::{errors::ChordBuildError, pattern::ChordPattern};

/// Collection of chord tones anchored at a root pitch.
#[derive(Debug, Clone)]
pub struct Chord {
    root: Pitch,
    pattern: ChordPattern,
}

impl Chord {
    /// Build a chord from an abstract root pitch.
    #[must_use]
    pub const fn from_abstract_root(root: AbstractPitch, pattern: ChordPattern) -> Self {
        Self {
            root: Pitch::Abstract(root),
            pattern,
        }
    }

    /// Build a chord from any pitch (literal or abstract).
    #[must_use]
    pub const fn from_pitch(root: Pitch, pattern: ChordPattern) -> Self {
        Self { root, pattern }
    }

    /// Construct a chord in twelve-tone equal temperament using semitone offsets from the root.
    ///
    /// # Errors
    ///
    /// Returns [`ChordBuildError`] if offset validation fails or the registry lacks the provided
    /// system.
    pub fn from_twelve_tet_offsets(
        root_index: i32,
        system: PitchSystemId,
        semitone_offsets: &[i32],
        registry: &TuningRegistry,
    ) -> Result<Self, ChordBuildError> {
        let pattern = ChordPattern::from_twelve_tet_offsets(semitone_offsets, &system, registry)?;
        Ok(Self::from_abstract_root(
            AbstractPitch::new(root_index, system),
            pattern,
        ))
    }

    /// Access the chord root.
    #[must_use]
    pub const fn root(&self) -> &Pitch {
        &self.root
    }

    /// Access the underlying chord pattern.
    #[must_use]
    pub const fn pattern(&self) -> &ChordPattern {
        &self.pattern
    }

    /// Number of tones contained in the chord.
    #[must_use]
    pub const fn tone_count(&self) -> usize {
        self.pattern.len()
    }

    /// Resolve every chord tone.
    ///
    /// # Errors
    ///
    /// Propagates [`PitchError`] variants that occur while applying the stored intervals.
    pub fn tones(&self, registry: &TuningRegistry) -> Result<Vec<Pitch>, PitchError> {
        self.pattern.tones(&self.root, registry)
    }

    /// Resolve a single chord tone by index.
    ///
    /// Returns [`None`] when `index` exceeds the number of tones.
    ///
    /// # Errors
    ///
    /// Propagates [`PitchError`] variants that occur while applying the stored intervals.
    pub fn tone(
        &self,
        index: usize,
        registry: &TuningRegistry,
    ) -> Result<Option<Pitch>, PitchError> {
        if index >= self.pattern.len() {
            return Ok(None);
        }
        if index == 0 {
            return Ok(Some(self.root.clone()));
        }
        let interval = &self.pattern.intervals()[index];
        let pitch = interval.apply_to(&self.root, registry)?;
        Ok(Some(pitch))
    }
}
