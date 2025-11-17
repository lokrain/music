use core::fmt;
use core::ops::{Add, AddAssign, Sub, SubAssign};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::system::PitchSystemId;

/// Abstract pitch reference, independent of any specific temperament.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AbstractPitch {
    pub index: i32,
    pub system: PitchSystemId,
}

impl AbstractPitch {
    #[must_use]
    pub const fn new(index: i32, system: PitchSystemId) -> Self {
        Self { index, system }
    }

    /// Shift the pitch index by the provided amount, returning a new pitch.
    #[must_use]
    pub fn transpose(&self, steps: i32) -> Self {
        Self {
            index: self.index + steps,
            system: self.system.clone(),
        }
    }

    /// Replace the tuning system while keeping the index.
    #[must_use]
    pub const fn with_system(&self, system: PitchSystemId) -> Self {
        Self {
            index: self.index,
            system,
        }
    }

    /// Return the `(index, system)` pair for ergonomic pattern matching.
    #[must_use]
    pub const fn components(&self) -> (i32, &PitchSystemId) {
        (self.index, &self.system)
    }
}

impl fmt::Display for AbstractPitch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}@{}", self.index, self.system)
    }
}

impl From<(i32, PitchSystemId)> for AbstractPitch {
    fn from((index, system): (i32, PitchSystemId)) -> Self {
        Self { index, system }
    }
}

impl<'a> From<(i32, &'a PitchSystemId)> for AbstractPitch {
    fn from((index, system): (i32, &'a PitchSystemId)) -> Self {
        Self {
            index,
            system: system.clone(),
        }
    }
}

impl Add<i32> for AbstractPitch {
    type Output = Self;

    fn add(mut self, rhs: i32) -> Self::Output {
        self.index += rhs;
        self
    }
}

impl Sub<i32> for AbstractPitch {
    type Output = Self;

    fn sub(mut self, rhs: i32) -> Self::Output {
        self.index -= rhs;
        self
    }
}

impl AddAssign<i32> for AbstractPitch {
    fn add_assign(&mut self, rhs: i32) {
        self.index += rhs;
    }
}

impl SubAssign<i32> for AbstractPitch {
    fn sub_assign(&mut self, rhs: i32) {
        self.index -= rhs;
    }
}
