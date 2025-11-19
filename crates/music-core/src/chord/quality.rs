use crate::{TuningRegistry, system::PitchSystemId};

use super::{errors::ChordBuildError, implementation::Chord, pattern::ChordPattern};

/// Common chord formulas expressed as semitone offsets from the root.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChordQuality {
    // Triads
    MajorTriad,
    MinorTriad,
    DiminishedTriad,
    AugmentedTriad,
    SuspendedSecond,
    SuspendedFourth,
    // Seventh chords
    DominantSeventh,
    MajorSeventh,
    MinorSeventh,
    MinorMajorSeventh,
    HalfDiminishedSeventh,
    DiminishedSeventh,
    // Extended chords (9/11/13)
    Dominant9,
    Major9,
    Minor9,
    Add9,
    Dominant11,
    Major11,
    Minor11,
    Dominant13,
    Major13,
    Minor13,
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
            | Self::DiminishedSeventh
            | Self::Add9 => 4,
            Self::Dominant9
            | Self::Major9
            | Self::Minor9
            | Self::Dominant11
            | Self::Major11
            | Self::Minor11
            | Self::Dominant13
            | Self::Major13
            | Self::Minor13 => 5,
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
            // Triads
            Self::MajorTriad => &[0, 4, 7],
            Self::MinorTriad => &[0, 3, 7],
            Self::DiminishedTriad => &[0, 3, 6],
            Self::AugmentedTriad => &[0, 4, 8],
            Self::SuspendedSecond => &[0, 2, 7],
            Self::SuspendedFourth => &[0, 5, 7],
            // Seventh chords
            Self::DominantSeventh => &[0, 4, 7, 10],
            Self::MajorSeventh => &[0, 4, 7, 11],
            Self::MinorSeventh => &[0, 3, 7, 10],
            Self::MinorMajorSeventh => &[0, 3, 7, 11],
            Self::HalfDiminishedSeventh => &[0, 3, 6, 10],
            Self::DiminishedSeventh => &[0, 3, 6, 9],
            // Extensions
            Self::Add9 => &[0, 4, 7, 14], // major triad + 9th (no 7th)
            Self::Dominant9 => &[0, 4, 7, 10, 14], // dom7 + 9th
            Self::Major9 => &[0, 4, 7, 11, 14], // maj7 + 9th
            Self::Minor9 => &[0, 3, 7, 10, 14], // min7 + 9th
            Self::Dominant11 => &[0, 4, 7, 10, 17], // dom7 + 11th (omit 9th)
            Self::Major11 => &[0, 4, 7, 11, 17], // maj7 + 11th (omit 9th)
            Self::Minor11 => &[0, 3, 7, 10, 17], // min7 + 11th (omit 9th)
            Self::Dominant13 => &[0, 4, 7, 10, 21], // dom7 + 13th (omit 9th, 11th)
            Self::Major13 => &[0, 4, 7, 11, 21], // maj7 + 13th (omit 9th, 11th)
            Self::Minor13 => &[0, 3, 7, 10, 21], // min7 + 13th (omit 9th, 11th)
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
            // Triads
            [0, 4, 7] => Some(Self::MajorTriad),
            [0, 3, 7] => Some(Self::MinorTriad),
            [0, 3, 6] => Some(Self::DiminishedTriad),
            [0, 4, 8] => Some(Self::AugmentedTriad),
            [0, 2, 7] => Some(Self::SuspendedSecond),
            [0, 5, 7] => Some(Self::SuspendedFourth),
            // Seventh chords
            [0, 4, 7, 10] => Some(Self::DominantSeventh),
            [0, 4, 7, 11] => Some(Self::MajorSeventh),
            [0, 3, 7, 10] => Some(Self::MinorSeventh),
            [0, 3, 7, 11] => Some(Self::MinorMajorSeventh),
            [0, 3, 6, 10] => Some(Self::HalfDiminishedSeventh),
            [0, 3, 6, 9] => Some(Self::DiminishedSeventh),
            // Extensions
            [0, 4, 7, 14] => Some(Self::Add9),
            [0, 4, 7, 10, 14] => Some(Self::Dominant9),
            [0, 4, 7, 11, 14] => Some(Self::Major9),
            [0, 3, 7, 10, 14] => Some(Self::Minor9),
            [0, 4, 7, 10, 17] => Some(Self::Dominant11),
            [0, 4, 7, 11, 17] => Some(Self::Major11),
            [0, 3, 7, 10, 17] => Some(Self::Minor11),
            [0, 4, 7, 10, 21] => Some(Self::Dominant13),
            [0, 4, 7, 11, 21] => Some(Self::Major13),
            [0, 3, 7, 10, 21] => Some(Self::Minor13),
            _ => None,
        }
    }
}
