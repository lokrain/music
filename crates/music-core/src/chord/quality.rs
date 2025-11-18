use crate::{TuningRegistry, system::PitchSystemId};

use super::{errors::ChordBuildError, implementation::Chord, pattern::ChordPattern};

/// Common chord formulas expressed as semitone offsets from the root.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChordQuality {
    MajorTriad,
    MinorTriad,
    DiminishedTriad,
    AugmentedTriad,
    SuspendedSecond,
    SuspendedFourth,
    DominantSeventh,
    MajorSeventh,
    MinorSeventh,
    MinorMajorSeventh,
    HalfDiminishedSeventh,
    DiminishedSeventh,
}

impl ChordQuality {
    /// Number of tones described by the quality.
    #[must_use]
    pub const fn tone_count(&self) -> usize {
        match self {
            Self::MajorTriad
            | Self::MinorTriad
            | Self::DiminishedTriad
            | Self::AugmentedTriad
            | Self::SuspendedSecond
            | Self::SuspendedFourth => 3,
            Self::DominantSeventh
            | Self::MajorSeventh
            | Self::MinorSeventh
            | Self::MinorMajorSeventh
            | Self::HalfDiminishedSeventh
            | Self::DiminishedSeventh => 4,
        }
    }

    /// Whether this quality represents a seventh chord (four-note voicing).
    #[must_use]
    pub const fn is_seventh(&self) -> bool {
        self.tone_count() == 4
    }

    /// Semitone offsets (from the root) that define the quality.
    #[must_use]
    pub const fn semitone_offsets(&self) -> &'static [i32] {
        match self {
            Self::MajorTriad => &[0, 4, 7],
            Self::MinorTriad => &[0, 3, 7],
            Self::DiminishedTriad => &[0, 3, 6],
            Self::AugmentedTriad => &[0, 4, 8],
            Self::SuspendedSecond => &[0, 2, 7],
            Self::SuspendedFourth => &[0, 5, 7],
            Self::DominantSeventh => &[0, 4, 7, 10],
            Self::MajorSeventh => &[0, 4, 7, 11],
            Self::MinorSeventh => &[0, 3, 7, 10],
            Self::MinorMajorSeventh => &[0, 3, 7, 11],
            Self::HalfDiminishedSeventh => &[0, 3, 6, 10],
            Self::DiminishedSeventh => &[0, 3, 6, 9],
        }
    }

    /// Build a chord pattern for this quality in twelve-tone equal temperament.
    ///
    /// # Errors
    ///
    /// Returns [`ChordBuildError`] if the registry cannot resolve the system.
    pub fn build_pattern(
        &self,
        system: &PitchSystemId,
        registry: &TuningRegistry,
    ) -> Result<ChordPattern, ChordBuildError> {
        ChordPattern::from_twelve_tet_offsets(self.semitone_offsets(), system, registry)
    }

    /// Build a chord instance for this quality in twelve-tone equal temperament.
    ///
    /// # Errors
    ///
    /// Returns [`ChordBuildError`] if the registry cannot resolve the system.
    pub fn build_chord(
        &self,
        root_index: i32,
        system: PitchSystemId,
        registry: &TuningRegistry,
    ) -> Result<Chord, ChordBuildError> {
        Chord::from_twelve_tet_offsets(root_index, system, self.semitone_offsets(), registry)
    }

    /// Attempt to classify a chord pattern against the known quality catalog.
    #[must_use]
    pub fn classify(pattern: &ChordPattern) -> Option<Self> {
        pattern
            .step_offsets()
            .and_then(|offsets| Self::from_offsets(&offsets))
    }

    const fn from_offsets(offsets: &[i32]) -> Option<Self> {
        match offsets {
            [0, 4, 7] => Some(Self::MajorTriad),
            [0, 3, 7] => Some(Self::MinorTriad),
            [0, 3, 6] => Some(Self::DiminishedTriad),
            [0, 4, 8] => Some(Self::AugmentedTriad),
            [0, 2, 7] => Some(Self::SuspendedSecond),
            [0, 5, 7] => Some(Self::SuspendedFourth),
            [0, 4, 7, 10] => Some(Self::DominantSeventh),
            [0, 4, 7, 11] => Some(Self::MajorSeventh),
            [0, 3, 7, 10] => Some(Self::MinorSeventh),
            [0, 3, 7, 11] => Some(Self::MinorMajorSeventh),
            [0, 3, 6, 10] => Some(Self::HalfDiminishedSeventh),
            [0, 3, 6, 9] => Some(Self::DiminishedSeventh),
            _ => None,
        }
    }
}
