use crate::{
    pitch::{AbstractPitch, PitchError, PitchLabel},
    registry::TuningRegistry,
};

use super::{Pitch, validate_literal_frequency};

impl Pitch {
    /// Human-friendly label if the tuning system provides one (falls back to literal frequency).
    #[must_use]
    pub fn name(&self, registry: &TuningRegistry) -> Option<PitchLabel> {
        self.try_label(registry).ok()
    }

    /// Return detailed label metadata, including numeric fallback when symbolic names are absent.
    ///
    /// # Errors
    ///
    /// Returns [`PitchError::UnknownSystem`] when the tuning registry does not contain the system
    /// backing an abstract pitch, or [`PitchError::InvalidLiteralFrequency`] for malformed
    /// literal pitches.
    pub fn try_label(&self, registry: &TuningRegistry) -> Result<PitchLabel, PitchError> {
        match self {
            Self::Frequency(freq) => Ok(PitchLabel::Frequency(validate_literal_frequency(*freq)?)),
            Self::Abstract(AbstractPitch { index, system }) => {
                let system_ref = registry.resolve_system(system).map_err(PitchError::from)?;
                if let Some(name) = system_ref.name_of(*index) {
                    return Ok(PitchLabel::Named(name));
                }
                let freq = system_ref.to_frequency(*index);
                Ok(PitchLabel::Frequency(freq))
            }
        }
    }

    /// Human-friendly name with error awareness that only succeeds when a symbolic label exists.
    ///
    /// # Errors
    ///
    /// Returns [`PitchError::UnknownSystem`] when the tuning system cannot be found or
    /// [`PitchError::NameUnavailable`] / [`PitchError::LiteralHasNoName`] when no symbolic name is
    /// available for the target pitch.
    pub fn try_name(&self, registry: &TuningRegistry) -> Result<String, PitchError> {
        match self {
            Self::Frequency(_) => Err(PitchError::LiteralHasNoName),
            Self::Abstract(AbstractPitch { index, system }) => {
                let system_ref = registry.resolve_system(system).map_err(PitchError::from)?;
                system_ref
                    .name_of(*index)
                    .ok_or_else(|| PitchError::NameUnavailable {
                        system: system.clone(),
                        index: *index,
                    })
            }
        }
    }
}
