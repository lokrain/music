//! core/music-time/src/timegrid.rs
//! Time-grid utilities for measure, beat, and subdivision alignment.

use crate::{
    meter::Meter,
    timespan::{TimePoint, TimeSpan},
};

/// Immutable collection of aligned musical positions.
#[derive(Debug, Clone)]
pub struct TimeGrid {
    pub measures: Vec<TimePoint>,
    pub beats: Vec<TimePoint>,
    pub subdivisions: Vec<TimePoint>,
}

impl TimeGrid {
    #[must_use]
    pub fn measures(&self) -> &[TimePoint] {
        &self.measures
    }

    #[must_use]
    pub fn beats(&self) -> &[TimePoint] {
        &self.beats
    }

    #[must_use]
    pub fn subdivisions(&self) -> &[TimePoint] {
        &self.subdivisions
    }
}

/// Configuration for generating a [`TimeGrid`].
#[derive(Debug, Clone, Copy)]
pub struct GridConfig {
    pub start: TimePoint,
    pub meter: Meter,
    pub bars: u32,
    pub subdivisions_per_beat: u32,
}

impl GridConfig {
    #[must_use]
    pub fn new(start: TimePoint, meter: Meter) -> Self {
        Self { start, meter, bars: 1, subdivisions_per_beat: 1 }
    }

    #[must_use]
    pub fn bars(mut self, bars: u32) -> Self {
        assert!(bars > 0, "grid must contain at least one bar");
        self.bars = bars;
        self
    }

    #[must_use]
    pub fn subdivisions_per_beat(mut self, subdivisions: u32) -> Self {
        assert!(subdivisions > 0, "subdivisions per beat must be > 0");
        self.subdivisions_per_beat = subdivisions;
        self
    }

    #[must_use]
    pub fn build(self) -> TimeGrid {
        let measure_span = self.meter.bar_span();
        let measures = accumulate(self.start, usize::try_from(self.bars).unwrap(), measure_span);

        let beats_per_bar = usize::from(self.meter.numerator);
        let total_beats = beats_per_bar * usize::try_from(self.bars).unwrap();
        let beat_span = TimeSpan::new(4.0 / f64::from(self.meter.denominator));
        let beats = accumulate(self.start, total_beats, beat_span);

        let subdivision_steps = total_beats * usize::try_from(self.subdivisions_per_beat).unwrap();
        let subdivision_span =
            TimeSpan::new(beat_span.as_beats() / f64::from(self.subdivisions_per_beat));
        let subdivisions = accumulate(self.start, subdivision_steps, subdivision_span);

        TimeGrid { measures, beats, subdivisions }
    }
}

fn accumulate(start: TimePoint, steps: usize, increment: TimeSpan) -> Vec<TimePoint> {
    let mut points = Vec::with_capacity(steps + 1);
    points.push(start);
    let mut current = start;
    for _ in 0..steps {
        current = current + increment;
        points.push(current);
    }
    points
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::timespan::TimePoint;

    #[test]
    fn grid_small_range() {
        let grid = GridConfig::new(TimePoint::new(0.0), Meter::FOUR_FOUR)
            .bars(2)
            .subdivisions_per_beat(2)
            .build();
        assert_eq!(grid.measures.len(), 3);
        assert_eq!(grid.beats.len(), 2 * 4 + 1);
        assert_eq!(grid.subdivisions.len(), (2 * 4 * 2) + 1);
        let last_measure = *grid.measures.last().unwrap();
        assert!((last_measure.as_beats() - 8.0).abs() < 1e-9);
    }

    #[test]
    fn grid_moderately_large_range() {
        let grid = GridConfig::new(TimePoint::new(16.0), Meter::SEVEN_EIGHT)
            .bars(32)
            .subdivisions_per_beat(3)
            .build();
        assert_eq!(grid.measures.len(), 33);
        assert_eq!(grid.beats.len(), (32 * 7) + 1);
        assert_eq!(grid.subdivisions.len(), (32 * 7 * 3) + 1);
        let span =
            grid.measures.last().unwrap().as_beats() - grid.measures.first().unwrap().as_beats();
        assert!((span - Meter::SEVEN_EIGHT.bar_span().as_beats() * 32.0).abs() < 1e-6);
    }
}
