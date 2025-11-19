//! Error types for interval operations.
//!
//! This module defines error types that can occur when constructing or manipulating intervals,
//! including validation failures for ratios and errors when deriving intervals from pitch pairs.

use core::fmt;

use crate::pitch::PitchError;

/// Errors that can occur while constructing or manipulating [`Interval`](super::Interval) instances.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum IntervalError {
    /// Interval ratios must be finite (not `NaN` or infinite).
    NonFiniteRatio(f32),
    /// Interval ratios must be strictly positive.
    NonPositiveRatio(f32),
}

impl fmt::Display for IntervalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NonFiniteRatio(value) => {
                write!(f, "interval ratio must be finite (got {value})")
            }
            Self::NonPositiveRatio(value) => {
                write!(f, "interval ratio must be positive (got {value})")
            }
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for IntervalError {}

/// Errors that can occur while deriving an [`Interval`](super::Interval) from two [`Pitch`](crate::pitch::Pitch) instances.
#[derive(Debug, Clone, PartialEq)]
pub enum IntervalBetweenError {
    /// Resolving one of the pitches failed (unknown system, invalid literal, etc.).
    Pitch(PitchError),
    /// Derived ratio was invalid even though both pitches resolved successfully.
    Interval {
        /// Ratio validation failure that prevented interval construction.
        source: IntervalError,
        /// Frequency provided by the target pitch, used to maintain backwards compatibility with
        /// [`PitchError::InvalidLiteralFrequency`].
        target_freq: f32,
    },
}

impl IntervalBetweenError {
    pub(super) const fn interval(source: IntervalError, target_freq: f32) -> Self {
        Self::Interval {
            source,
            target_freq,
        }
    }
}

impl fmt::Display for IntervalBetweenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Pitch(err) => write!(f, "{err}"),
            Self::Interval { source, .. } => write!(f, "failed to build interval: {source}"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for IntervalBetweenError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Pitch(err) => Some(err),
            Self::Interval { source, .. } => Some(source),
        }
    }
}

impl From<PitchError> for IntervalBetweenError {
    fn from(value: PitchError) -> Self {
        Self::Pitch(value)
    }
}
