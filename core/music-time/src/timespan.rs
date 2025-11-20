//! core/music-time/src/timespan.rs
//! Time points and spans in beats.

use core::ops::{Add, AddAssign, Sub, SubAssign};

use crate::Beat;

/// Absolute musical time point in beats.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct TimePoint(f64);

/// Musical duration in beats.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct TimeSpan(f64);

impl TimePoint {
    /// Construct a time point with non-negative beat value.
    #[must_use]
    pub fn new(beats: f64) -> Self {
        assert!(beats.is_finite() && beats >= 0.0, "time point must be non-negative and finite");
        Self(beats)
    }

    /// Create from a [`Beat`].
    #[must_use]
    pub fn from_beat(beat: Beat) -> Self {
        Self::new(beat.as_f64())
    }

    /// Raw beat value.
    #[must_use]
    pub fn as_beats(&self) -> f64 {
        self.0
    }

    /// Add a span, producing a new time point.
    #[must_use]
    pub fn add_span(self, span: TimeSpan) -> Self {
        Self::new(self.0 + span.0)
    }

    /// Subtract a span if possible, returning `None` when it would go negative.
    #[must_use]
    pub fn checked_sub_span(self, span: TimeSpan) -> Option<Self> {
        (self.0 >= span.0).then(|| Self::new(self.0 - span.0))
    }

    /// Distance from another point (absolute difference).
    #[must_use]
    pub fn distance_to(self, other: TimePoint) -> TimeSpan {
        TimeSpan::new((self.0 - other.0).abs())
    }
}

impl TimeSpan {
    /// Construct a span from raw beat units.
    ///
    /// # Panics
    ///
    /// Panics if `beats` is negative or not finite.
    #[must_use]
    pub fn new(beats: f64) -> Self {
        assert!(beats.is_finite() && beats >= 0.0, "duration must be non-negative and finite");
        Self(beats)
    }

    /// Convert from a [`Beat`] wrapper (beat magnitude interpreted as duration).
    #[must_use]
    pub fn from_beats(beat: Beat) -> Self {
        Self::new(beat.as_f64())
    }

    /// A zero-length span.
    #[must_use]
    pub const fn zero() -> Self {
        Self(0.0)
    }

    /// Return the raw beat count.
    #[must_use]
    pub fn as_beats(&self) -> f64 {
        self.0
    }

    /// Add two spans.
    #[must_use]
    pub fn add_span(self, other: Self) -> Self {
        Self::new(self.0 + other.0)
    }

    /// Subtract a smaller span, returning `None` if it would become negative.
    #[must_use]
    pub fn checked_sub(self, other: Self) -> Option<Self> {
        (self.0 >= other.0).then(|| Self::new(self.0 - other.0))
    }
}

impl Add<TimeSpan> for TimePoint {
    type Output = TimePoint;

    fn add(self, rhs: TimeSpan) -> Self::Output {
        self.add_span(rhs)
    }
}

impl Sub<TimeSpan> for TimePoint {
    type Output = TimePoint;

    fn sub(self, rhs: TimeSpan) -> Self::Output {
        self.checked_sub_span(rhs).expect("cannot subtract span beyond origin")
    }
}

impl Sub for TimePoint {
    type Output = TimeSpan;

    fn sub(self, rhs: Self) -> Self::Output {
        assert!(self.0 >= rhs.0, "time point subtraction cannot go negative");
        TimeSpan::new(self.0 - rhs.0)
    }
}

impl Add for TimeSpan {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        self.add_span(rhs)
    }
}

impl AddAssign for TimeSpan {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Sub for TimeSpan {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.checked_sub(rhs).expect("duration cannot go negative")
    }
}

impl SubAssign for TimeSpan {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point_span_arithmetic() {
        let start = TimePoint::new(4.0);
        let span = TimeSpan::new(1.5);
        let end = start + span;
        assert!((end.as_beats() - 5.5).abs() < 1e-9);
        let original = end - span;
        assert!((original.as_beats() - start.as_beats()).abs() < 1e-9);
        let distance = end - start;
        assert!((distance.as_beats() - 1.5).abs() < 1e-9);
        let symmetric = start.distance_to(end);
        assert!((symmetric.as_beats() - 1.5).abs() < 1e-9);
    }

    #[test]
    fn timespan_add_sub() {
        let mut span = TimeSpan::new(2.0);
        span += TimeSpan::new(0.5);
        assert!((span.as_beats() - 2.5).abs() < 1e-9);
        span -= TimeSpan::new(1.0);
        assert!((span.as_beats() - 1.5).abs() < 1e-9);
    }

    #[test]
    #[should_panic(expected = "cannot subtract span beyond origin")]
    fn timepoint_sub_panics_when_negative() {
        let start = TimePoint::new(1.0);
        let span = TimeSpan::new(2.0);
        let _ = start - span;
    }

    #[test]
    #[should_panic(expected = "time point subtraction cannot go negative")]
    fn timepoint_difference_panics_when_negative() {
        let earlier = TimePoint::new(1.0);
        let later = TimePoint::new(0.5);
        let _ = later - earlier;
    }
}
