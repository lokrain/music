use crate::{
    pitch::{AbstractPitch, PitchError},
    registry::TuningRegistry,
};

use super::{Pitch, validate_literal_frequency};

impl Pitch {
    /// Resolve to a literal-frequency pitch while preserving the original on success.
    ///
    /// # Errors
    ///
    /// Returns [`PitchError::UnknownSystem`] if the pitch's tuning system is absent.
    #[must_use = "discarding the resolved pitch means the computation was wasted"]
    pub fn resolved(&self, registry: &TuningRegistry) -> Result<Self, PitchError> {
        self.clone().into_resolved(registry)
    }

    /// Resolve to a literal-frequency pitch, consuming `self` when successful.
    ///
    /// # Errors
    ///
    /// Returns [`PitchError::UnknownSystem`] when the associated tuning system is absent or
    /// [`PitchError::InvalidLiteralFrequency`] for invalid literal pitches.
    pub fn into_resolved(self, registry: &TuningRegistry) -> Result<Self, PitchError> {
        match self {
            Self::Frequency(freq) => Ok(Self::Frequency(validate_literal_frequency(freq)?)),
            Self::Abstract(_) => self.try_freq_hz(registry).map(Self::Frequency),
        }
    }

    /// Resolve to a frequency using a registry (if needed).
    #[must_use]
    pub fn freq_hz(&self, registry: &TuningRegistry) -> Option<f32> {
        self.try_freq_hz(registry).ok()
    }

    /// Resolve to a frequency, returning a descriptive error when the system is missing.
    ///
    /// # Errors
    ///
    /// Returns [`PitchError::UnknownSystem`] if the tuning system cannot be found.
    pub fn try_freq_hz(&self, registry: &TuningRegistry) -> Result<f32, PitchError> {
        match self {
            Self::Frequency(f) => validate_literal_frequency(*f),
            Self::Abstract(AbstractPitch { index, system }) => registry
                .resolve_frequency(system, *index)
                .map_err(PitchError::from),
        }
    }

    /// Consume the pitch while resolving it into a literal frequency.
    ///
    /// # Errors
    ///
    /// Propagates [`PitchError::UnknownSystem`] and [`PitchError::InvalidLiteralFrequency`].
    pub fn into_freq_hz(self, registry: &TuningRegistry) -> Result<f32, PitchError> {
        match self {
            Self::Frequency(f) => validate_literal_frequency(f),
            Self::Abstract(_) => self.try_freq_hz(registry),
        }
    }
}
