//! core/music-time/src/beat.rs
//! Beat representation.

use core::ops::{Add, AddAssign, Sub, SubAssign};

/// Abstract beat index within a piece (non-negative floating value).
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Beat(f64);

impl Beat {
    /// Create a beat index ensuring it is finite and non-negative.
    #[must_use]
    pub fn new(value: f64) -> Self {
        assert!(value.is_finite() && value >= 0.0, "beat index must be finite and non-negative");
        Self(value)
    }

    /// Zero-beat convenience.
    #[must_use]
    pub const fn zero() -> Self {
        Self(0.0)
    }

    /// Raw beat value.
    #[must_use]
    pub fn as_f64(self) -> f64 {
        self.0
    }

    /// Attempt to subtract another beat, returning `None` if it would go negative.
    #[must_use]
    pub fn checked_sub(self, other: Self) -> Option<Self> {
        (self.0 >= other.0).then(|| Self::new(self.0 - other.0))
    }
}

impl From<f64> for Beat {
    fn from(value: f64) -> Self {
        Self::new(value)
    }
}

impl From<Beat> for f64 {
    fn from(beat: Beat) -> Self {
        beat.0
    }
}

impl Add for Beat {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.0 + rhs.0)
    }
}

impl AddAssign for Beat {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Sub for Beat {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.checked_sub(rhs).expect("beat subtraction cannot go negative")
    }
}

impl SubAssign for Beat {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn beat_arithmetic() {
        let mut a = Beat::new(4.0);
        let b = Beat::new(1.5);
        a += b;
        assert!((a.as_f64() - 5.5).abs() < 1e-9);
        a -= b;
        assert!((a.as_f64() - 4.0).abs() < 1e-9);
    }

    #[test]
    #[should_panic(expected = "beat subtraction cannot go negative")]
    fn beat_sub_panic() {
        let a = Beat::new(1.0);
        let b = Beat::new(2.0);
        let _ = a - b;
    }
}
