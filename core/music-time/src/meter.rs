//! core/music-time/src/meter.rs
//! Meter (time signature).

use crate::timespan::TimeSpan;

/// Time signature / meter.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Meter {
    pub numerator: u8,
    pub denominator: u8,
}

impl Meter {
    /// Common 4/4 meter.
    pub const FOUR_FOUR: Meter = Meter { numerator: 4, denominator: 4 };
    /// 3/4 meter.
    pub const THREE_FOUR: Meter = Meter { numerator: 3, denominator: 4 };
    /// 6/8 compound meter.
    pub const SIX_EIGHT: Meter = Meter { numerator: 6, denominator: 8 };
    /// 5/4 asymmetric meter.
    pub const FIVE_FOUR: Meter = Meter { numerator: 5, denominator: 4 };
    /// 7/8 asymmetric meter.
    pub const SEVEN_EIGHT: Meter = Meter { numerator: 7, denominator: 8 };

    /// Construct a new meter.
    ///
    /// # Panics
    ///
    /// Panics if either `numerator` or `denominator` is zero.
    #[must_use]
    pub fn new(numerator: u8, denominator: u8) -> Self {
        assert!(numerator > 0, "numerator must be > 0");
        assert!(denominator > 0, "denominator must be > 0");
        Self { numerator, denominator }
    }

    /// Beats per bar, normalized so a quarter note equals 1 beat.
    #[must_use]
    pub fn beats_per_bar(&self) -> f64 {
        f64::from(self.numerator) * (4.0 / f64::from(self.denominator))
    }

    /// Length of a single bar as a [`TimeSpan`] using quarter-note beats.
    #[must_use]
    pub fn bar_span(&self) -> TimeSpan {
        TimeSpan::new(self.beats_per_bar())
    }

    /// How many bars fit within the provided span.
    #[must_use]
    pub fn bars_for_span(&self, span: TimeSpan) -> f64 {
        span.as_beats() / self.beats_per_bar()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn beats_per_bar_matches_common_meters() {
        assert_eq!(Meter::FOUR_FOUR.beats_per_bar(), 4.0);
        assert_eq!(Meter::THREE_FOUR.beats_per_bar(), 3.0);
        assert_eq!(Meter::SIX_EIGHT.beats_per_bar(), 3.0);
        assert!((Meter::SEVEN_EIGHT.beats_per_bar() - 3.5).abs() < 1e-9);
    }

    #[test]
    fn bar_span_and_bars_for_span() {
        let meter = Meter::new(5, 4);
        let bar = meter.bar_span();
        assert_eq!(bar.as_beats(), 5.0);
        let span = TimeSpan::new(15.0);
        assert_eq!(meter.bars_for_span(span), 3.0);
    }
}
