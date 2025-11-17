use core::{convert::TryFrom, fmt};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::pitch::{AbstractPitch, PitchError};

/// A musical pitch: either a literal frequency, or an abstract pitch interpreted via a tuning system.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Pitch {
    /// Literal frequency in Hz.
    Frequency(f32),

    /// Abstract pitch (index + tuning system).
    Abstract(AbstractPitch),
}

impl TryFrom<Pitch> for AbstractPitch {
    type Error = PitchError;

    fn try_from(value: Pitch) -> Result<Self, Self::Error> {
        match value {
            Pitch::Frequency(_) => Err(PitchError::NotAbstract),
            Pitch::Abstract(pitch) => Ok(pitch),
        }
    }
}

impl fmt::Display for Pitch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Frequency(freq) => write!(f, "{freq:.3} Hz"),
            Self::Abstract(abstract_pitch) => abstract_pitch.fmt(f),
        }
    }
}

impl From<AbstractPitch> for Pitch {
    fn from(pitch: AbstractPitch) -> Self {
        Self::Abstract(pitch)
    }
}
