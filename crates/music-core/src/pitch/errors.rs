use core::fmt;

use crate::{registry::TuningError, system::PitchSystemId};

/// Errors emitted when manipulating or resolving pitches.
#[derive(Debug, Clone, PartialEq)]
pub enum PitchError {
    /// Registry does not contain the requested tuning system identifier.
    UnknownSystem(PitchSystemId),
    /// Literal frequency pitches must be finite and positive.
    InvalidLiteralFrequency(f32),
    /// Caller attempted to fetch a symbolic name for a pitch that lacks one.
    NameUnavailable { system: PitchSystemId, index: i32 },
    /// Caller expected an abstract pitch but encountered a literal frequency.
    NotAbstract,
    /// Literal pitches cannot yield symbolic names.
    LiteralHasNoName,
}

impl fmt::Display for PitchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownSystem(id) => write!(f, "unknown tuning system: {id}"),
            Self::InvalidLiteralFrequency(freq) => {
                write!(f, "invalid literal frequency: {freq}")
            }
            Self::NameUnavailable { system, index } => write!(
                f,
                "tuning system {system} does not provide a name for index {index}"
            ),
            Self::NotAbstract => f.write_str("pitch is not abstract"),
            Self::LiteralHasNoName => {
                f.write_str("literal frequency pitches do not have symbolic names")
            }
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for PitchError {}

impl From<TuningError> for PitchError {
    fn from(value: TuningError) -> Self {
        match value {
            TuningError::UnknownSystem(id) => Self::UnknownSystem(id),
        }
    }
}
