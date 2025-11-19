//! Chord inversion representation and detection.
//!
//! This module provides types and utilities for working with chord inversions, which describe
//! which chord tone appears in the bass (lowest position).

use core::fmt;

/// Inversion of a chord based on which tone appears in the bass.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Inversion {
    /// Root position: root in bass (0th tone).
    Root,
    /// First inversion: third in bass (1st tone above root).
    First,
    /// Second inversion: fifth in bass (2nd tone above root).
    Second,
    /// Third inversion: seventh in bass (3rd tone above root, only for seventh chords and extensions).
    Third,
    /// Higher inversions for extended chords (9/11/13).
    Higher(u8),
}

impl fmt::Display for Inversion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Root => f.write_str("root position"),
            Self::First => f.write_str("1st inversion"),
            Self::Second => f.write_str("2nd inversion"),
            Self::Third => f.write_str("3rd inversion"),
            Self::Higher(n) => write!(f, "{n}th inversion"),
        }
    }
}

impl Inversion {
    /// Construct an inversion from the bass tone index (0 = root position, 1 = first inversion, etc.).
    #[must_use]
    pub const fn from_bass_index(index: usize) -> Self {
        match index {
            0 => Self::Root,
            1 => Self::First,
            2 => Self::Second,
            3 => Self::Third,
            #[allow(clippy::cast_possible_truncation)]
            n => Self::Higher(n as u8),
        }
    }

    /// The index of the chord tone that appears in the bass (0 = root, 1 = third, 2 = fifth, etc.).
    #[must_use]
    pub const fn bass_index(&self) -> usize {
        match self {
            Self::Root => 0,
            Self::First => 1,
            Self::Second => 2,
            Self::Third => 3,
            Self::Higher(n) => *n as usize,
        }
    }
}
