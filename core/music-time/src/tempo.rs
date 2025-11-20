//! core/music-time/src/tempo.rs
//! Tempo in beats per minute with time conversion helpers.

use crate::{meter::Meter, timespan::TimeSpan};

/// Tempo definition.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Tempo {
    bpm: f64,
}

impl Tempo {
    /// Construct a tempo in beats per minute.
    ///
    /// # Panics
    ///
    /// Panics if `bpm` is non-positive or not finite.
    #[must_use]
    pub fn new(bpm: f64) -> Self {
        assert!(bpm.is_finite() && bpm > 0.0, "tempo must be positive and finite");
        Self { bpm }
    }

    /// BPM accessor.
    #[must_use]
    pub fn bpm(&self) -> f64 {
        self.bpm
    }

    /// Beats per second.
    #[must_use]
    pub fn beats_per_second(&self) -> f64 {
        self.bpm / 60.0
    }

    /// Seconds per beat.
    #[must_use]
    pub fn seconds_per_beat(&self) -> f64 {
        60.0 / self.bpm
    }

    /// Seconds per bar for the supplied meter.
    #[must_use]
    pub fn seconds_per_bar(&self, meter: Meter) -> f64 {
        meter.beats_per_bar() * self.seconds_per_beat()
    }

    /// Convert a beat-based [`TimeSpan`] to seconds.
    #[must_use]
    pub fn seconds_for_span(&self, span: TimeSpan) -> f64 {
        span.as_beats() * self.seconds_per_beat()
    }

    /// Convert seconds to a beat-based [`TimeSpan`].
    #[must_use]
    pub fn span_for_seconds(&self, seconds: f64) -> TimeSpan {
        assert!(seconds.is_finite() && seconds >= 0.0, "seconds must be non-negative and finite");
        TimeSpan::new(seconds / self.seconds_per_beat())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seconds_per_bar_across_meters() {
        let tempo = Tempo::new(120.0);
        assert!((tempo.seconds_per_bar(Meter::FOUR_FOUR) - 2.0).abs() < 1e-9);
        assert!((tempo.seconds_per_bar(Meter::THREE_FOUR) - 1.5).abs() < 1e-9);
        assert!((tempo.seconds_per_bar(Meter::SIX_EIGHT) - 1.5).abs() < 1e-9);
        assert!((tempo.seconds_per_bar(Meter::FIVE_FOUR) - 2.5).abs() < 1e-9);
        assert!((tempo.seconds_per_bar(Meter::SEVEN_EIGHT) - 1.75).abs() < 1e-9);
    }

    #[test]
    fn span_second_roundtrip() {
        let tempo = Tempo::new(90.0);
        let span = TimeSpan::new(3.0);
        let seconds = tempo.seconds_for_span(span);
        assert!((seconds - 2.0).abs() < 1e-9);
        let roundtrip = tempo.span_for_seconds(seconds);
        assert!((roundtrip.as_beats() - span.as_beats()).abs() < 1e-9);
    }
}
