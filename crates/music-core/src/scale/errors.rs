use core::fmt;

use crate::{interval::IntervalError, pitch::PitchError};

use super::pattern::ScalePatternError;

/// Errors that can occur while lazily traversing scale degrees.
#[derive(Debug, Clone, PartialEq)]
pub enum ScaleDegreeError {
    /// Applying an interval required resolving a pitch that failed (unknown system, invalid literal, etc.).
    Pitch(PitchError),
    /// Composing an intermediate interval produced an invalid ratio.
    Interval(IntervalError),
}

impl fmt::Display for ScaleDegreeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Pitch(err) => write!(f, "{err}"),
            Self::Interval(err) => write!(f, "{err}"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for ScaleDegreeError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Pitch(err) => Some(err),
            Self::Interval(err) => Some(err),
        }
    }
}

impl From<PitchError> for ScaleDegreeError {
    fn from(value: PitchError) -> Self {
        Self::Pitch(value)
    }
}

impl From<IntervalError> for ScaleDegreeError {
    fn from(value: IntervalError) -> Self {
        Self::Interval(value)
    }
}

/// Errors that can arise while rotating a scale forwards or backwards.
#[derive(Debug, Clone, PartialEq)]
pub enum ScaleModeError {
    /// Pitch resolution failed while transposing the scale root.
    Pitch(PitchError),
    /// Interval inversion failed while walking backwards through the pattern.
    Interval(IntervalError),
}

impl fmt::Display for ScaleModeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Pitch(err) => write!(f, "{err}"),
            Self::Interval(err) => write!(f, "{err}"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for ScaleModeError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Pitch(err) => Some(err),
            Self::Interval(err) => Some(err),
        }
    }
}

impl From<PitchError> for ScaleModeError {
    fn from(value: PitchError) -> Self {
        Self::Pitch(value)
    }
}

impl From<IntervalError> for ScaleModeError {
    fn from(value: IntervalError) -> Self {
        Self::Interval(value)
    }
}

/// Errors that occur while constructing helper scales or patterns.
#[derive(Debug, Clone, PartialEq)]
pub enum ScaleBuildError {
    /// Building the pattern required registry access that failed.
    Pitch(PitchError),
    /// Input steps violated pattern constraints (e.g., empty list).
    Pattern(ScalePatternError),
}

impl fmt::Display for ScaleBuildError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Pitch(err) => write!(f, "{err}"),
            Self::Pattern(err) => write!(f, "{err}"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for ScaleBuildError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Pitch(err) => Some(err),
            Self::Pattern(err) => Some(err),
        }
    }
}

impl From<PitchError> for ScaleBuildError {
    fn from(value: PitchError) -> Self {
        Self::Pitch(value)
    }
}

impl From<ScalePatternError> for ScaleBuildError {
    fn from(value: ScalePatternError) -> Self {
        Self::Pattern(value)
    }
}
