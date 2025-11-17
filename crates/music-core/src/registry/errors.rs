use core::fmt;

use crate::system::PitchSystemId;

/// Errors that can occur while mutating the registry (e.g., inserting duplicates).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RegistryInsertError {
    /// Attempted to register a tuning system whose identifier already exists.
    DuplicateSystem(PitchSystemId),
}

impl fmt::Display for RegistryInsertError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DuplicateSystem(id) => write!(f, "tuning system {id} is already registered"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for RegistryInsertError {}

/// Errors that can occur when resolving tunings from the registry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TuningError {
    /// Requested tuning system was not registered.
    UnknownSystem(PitchSystemId),
}

impl fmt::Display for TuningError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownSystem(id) => write!(f, "unknown tuning system: {id}"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for TuningError {}
