use alloc::string::String;
use core::fmt;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Rich label metadata describing how a pitch should be rendered to humans.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum PitchLabel {
    /// Symbolic name provided by the tuning system (e.g., "12-TET(69)").
    Named(String),
    /// Literal frequency fallback, rendered in Hz.
    Frequency(f32),
}

impl PitchLabel {
    /// Returns the label as a user-facing string.
    #[must_use]
    pub fn to_string_lossy(&self) -> String {
        self.to_string()
    }

    /// Access the literal frequency if this label represents one.
    #[must_use]
    pub const fn as_frequency(&self) -> Option<f32> {
        match self {
            Self::Named(_) => None,
            Self::Frequency(freq) => Some(*freq),
        }
    }

    /// True when the label is symbolic (named) instead of numeric.
    #[must_use]
    pub const fn is_symbolic(&self) -> bool {
        matches!(self, Self::Named(_))
    }
}

impl fmt::Display for PitchLabel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Named(name) => f.write_str(name),
            Self::Frequency(freq) => write!(f, "{freq:.3} Hz"),
        }
    }
}
