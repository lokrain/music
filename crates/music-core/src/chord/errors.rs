use core::fmt;

use crate::pitch::PitchError;

use super::pattern::ChordPatternError;

/// Errors that can occur while constructing helper chord patterns.
#[derive(Debug, Clone, PartialEq)]
pub enum ChordBuildError {
    /// Resolving an interval required registry access that failed.
    Pitch(PitchError),
    /// Supplied offsets or intervals violated pattern invariants.
    Pattern(ChordPatternError),
}

impl fmt::Display for ChordBuildError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Pitch(err) => write!(f, "{err}"),
            Self::Pattern(err) => write!(f, "{err}"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for ChordBuildError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Pitch(err) => Some(err),
            Self::Pattern(err) => Some(err),
        }
    }
}

impl From<PitchError> for ChordBuildError {
    fn from(value: PitchError) -> Self {
        Self::Pitch(value)
    }
}

impl From<ChordPatternError> for ChordBuildError {
    fn from(value: ChordPatternError) -> Self {
        Self::Pattern(value)
    }
}

/// Errors that can arise while deriving chords from an existing scale.
#[derive(Debug, Clone, PartialEq)]
pub enum ChordDiatonicError {
    /// Resolving one of the requested scale degrees failed.
    Pitch(PitchError),
    /// Constructed intervals violated chord pattern invariants.
    Pattern(ChordPatternError),
}

impl fmt::Display for ChordDiatonicError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Pitch(err) => write!(f, "{err}"),
            Self::Pattern(err) => write!(f, "{err}"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for ChordDiatonicError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Pitch(err) => Some(err),
            Self::Pattern(err) => Some(err),
        }
    }
}

impl From<PitchError> for ChordDiatonicError {
    fn from(value: PitchError) -> Self {
        Self::Pitch(value)
    }
}

impl From<ChordPatternError> for ChordDiatonicError {
    fn from(value: ChordPatternError) -> Self {
        Self::Pattern(value)
    }
}
