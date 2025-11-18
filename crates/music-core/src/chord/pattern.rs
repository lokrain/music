use alloc::vec::Vec;
use core::{f32, fmt};

use crate::{
    TuningRegistry,
    interval::Interval,
    pitch::{Pitch, PitchError},
    system::PitchSystemId,
};

use super::errors::ChordBuildError;

/// Errors that can occur while constructing or manipulating chord patterns.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChordPatternError {
    /// Patterns must describe at least the root interval.
    EmptyPattern,
    /// Patterns must begin with the identity interval (root to root).
    MissingRootInterval,
}

impl fmt::Display for ChordPatternError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyPattern => f.write_str("chord pattern must describe at least one tone"),
            Self::MissingRootInterval => {
                f.write_str("chord pattern must begin with the identity interval")
            }
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for ChordPatternError {}

/// Ordered collection of intervals describing each chord tone relative to the root.
#[derive(Debug, Clone)]
pub struct ChordPattern {
    intervals: Vec<Interval>,
}

const ROOT_RATIO_EPSILON: f32 = f32::EPSILON;

impl ChordPattern {
    /// Create a pattern from explicit intervals offset from the root.
    ///
    /// The first interval must be [`Interval::identity`], representing the root itself.
    ///
    /// # Errors
    ///
    /// Returns [`ChordPatternError::EmptyPattern`] if no intervals are supplied or
    /// [`ChordPatternError::MissingRootInterval`] when the first interval is not the identity.
    pub fn from_intervals(intervals: Vec<Interval>) -> Result<Self, ChordPatternError> {
        if intervals.is_empty() {
            return Err(ChordPatternError::EmptyPattern);
        }
        let root_is_identity = matches!(
            intervals.first(),
            Some(interval) if (interval.ratio() - 1.0_f32).abs() <= ROOT_RATIO_EPSILON
        );
        if !root_is_identity {
            return Err(ChordPatternError::MissingRootInterval);
        }
        Ok(Self { intervals })
    }

    /// Number of tones described by the pattern.
    #[must_use]
    pub const fn len(&self) -> usize {
        self.intervals.len()
    }

    /// Whether the pattern is empty.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.intervals.is_empty()
    }

    /// Ordered slice of intervals.
    #[must_use]
    pub fn intervals(&self) -> &[Interval] {
        &self.intervals
    }

    /// Extract stored step offsets when every interval retains abstract step information.
    ///
    /// Returns `None` when any interval was constructed purely from literal frequencies, which
    /// prevents lossless recovery of the original offsets.
    #[must_use]
    pub fn step_offsets(&self) -> Option<Vec<i32>> {
        if self.intervals.is_empty() {
            return Some(Vec::new());
        }

        let mut offsets = Vec::with_capacity(self.intervals.len());
        offsets.push(0);

        let mut reference_system: Option<&PitchSystemId> = None;
        for interval in self.intervals.iter().skip(1) {
            let (delta, system) = interval.steps()?;
            if let Some(existing) = reference_system {
                if existing != system {
                    return None;
                }
            } else {
                reference_system = Some(system);
            }
            offsets.push(delta);
        }

        Some(offsets)
    }

    /// Apply the pattern to `root`, returning every chord tone.
    ///
    /// # Errors
    ///
    /// Returns [`PitchError`] if any tone fails to resolve through the registry.
    pub fn tones(&self, root: &Pitch, registry: &TuningRegistry) -> Result<Vec<Pitch>, PitchError> {
        if self.intervals.is_empty() {
            return Ok(Vec::new());
        }

        let mut tones = Vec::with_capacity(self.intervals.len());
        tones.push(root.clone());
        for interval in self.intervals.iter().skip(1) {
            let pitch = interval.apply_to(root, registry)?;
            tones.push(pitch);
        }
        Ok(tones)
    }

    /// Build a twelve-tone equal temperament pattern from semitone offsets.
    ///
    /// Offsets are measured from the root (0 semitones) and must be provided in ascending order.
    ///
    /// # Errors
    ///
    /// Returns [`ChordBuildError`] if offsets are empty, omit the root, or if interval construction
    /// fails because the registry lacks the provided system.
    pub fn from_twelve_tet_offsets(
        semitone_offsets: &[i32],
        system: &PitchSystemId,
        registry: &TuningRegistry,
    ) -> Result<Self, ChordBuildError> {
        if semitone_offsets.is_empty() {
            return Err(ChordPatternError::EmptyPattern.into());
        }
        if semitone_offsets[0] != 0 {
            return Err(ChordPatternError::MissingRootInterval.into());
        }

        let root = Pitch::abstract_pitch(0, system.clone());
        let mut intervals = Vec::with_capacity(semitone_offsets.len());
        for offset in semitone_offsets {
            let target = Pitch::abstract_pitch(*offset, system.clone());
            let interval = Interval::between(&root, &target, registry)?;
            intervals.push(interval);
        }

        Self::from_intervals(intervals).map_err(ChordBuildError::from)
    }
}
