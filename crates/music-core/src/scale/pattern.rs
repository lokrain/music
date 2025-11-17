use alloc::vec::Vec;
use core::fmt;

use crate::{
    TuningRegistry,
    interval::{Interval, IntervalError},
    pitch::Pitch,
    system::PitchSystemId,
};

use super::ScaleBuildError;

/// Errors that can occur while constructing a [`ScalePattern`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScalePatternError {
    /// Patterns must contain at least one interval step.
    EmptyPattern,
}

impl fmt::Display for ScalePatternError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyPattern => f.write_str("scale pattern must contain at least one step"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for ScalePatternError {}

/// Ordered collection of interval steps describing a single-octave scale pattern.
#[derive(Debug, Clone)]
pub struct ScalePattern {
    pub(super) steps: Vec<Interval>,
}

impl ScalePattern {
    /// Create a pattern from the supplied sequence of intervals between consecutive degrees.
    ///
    /// # Errors
    ///
    /// Returns [`ScalePatternError::EmptyPattern`] when `steps` is empty.
    pub fn from_steps(steps: Vec<Interval>) -> Result<Self, ScalePatternError> {
        if steps.is_empty() {
            return Err(ScalePatternError::EmptyPattern);
        }
        Ok(Self { steps })
    }

    /// Number of steps in this pattern.
    #[must_use]
    pub const fn len(&self) -> usize {
        self.steps.len()
    }

    /// Whether this pattern contains no steps.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.steps.is_empty()
    }

    /// Ordered slice of interval steps.
    #[must_use]
    pub fn steps(&self) -> &[Interval] {
        &self.steps
    }

    /// Rotate the pattern to begin at a different degree (useful for deriving modes).
    #[must_use]
    pub fn rotate(&self, offset: usize) -> Self {
        if self.steps.is_empty() {
            return self.clone();
        }
        let len = self.steps.len();
        let shift = offset % len;
        if shift == 0 {
            return self.clone();
        }
        let mut rotated = self.steps[shift..].to_vec();
        rotated.extend_from_slice(&self.steps[..shift]);
        Self { steps: rotated }
    }

    /// Interval accumulated from the root to the provided degree (wrapping across octaves).
    ///
    /// # Errors
    ///
    /// Returns [`IntervalError`] if intermediate composition produces an invalid ratio.
    pub fn degree_interval(&self, degree: usize) -> Result<Interval, IntervalError> {
        if degree == 0 || self.steps.is_empty() {
            return Ok(Interval::identity());
        }

        let len = self.steps.len();
        let mut acc = self.steps[0].clone();
        if degree == 1 {
            return Ok(acc);
        }

        for idx in 1..degree {
            acc = acc.compose(&self.steps[idx % len])?;
        }
        Ok(acc)
    }

    /// Intervals from the root to each degree up to `highest_degree` (inclusive).
    ///
    /// The returned vector always includes the identity interval for degree `0` at index `0`.
    /// Subsequent entries correspond to cumulative intervals for each requested degree.
    ///
    /// # Errors
    ///
    /// Returns [`IntervalError`] if intermediate composition produces an invalid ratio.
    pub fn degree_intervals(&self, highest_degree: usize) -> Result<Vec<Interval>, IntervalError> {
        let mut intervals = Vec::with_capacity(highest_degree + 1);
        let mut iter = self.degree_intervals_iter();
        for _ in 0..=highest_degree {
            let (_, interval) = match iter.next() {
                Some(Ok((degree, interval))) => (degree, interval),
                Some(Err(err)) => return Err(err),
                None => break,
            };
            intervals.push(interval);
        }
        Ok(intervals)
    }

    /// Lazily iterate over degree intervals, yielding `(degree, interval)` pairs indefinitely.
    #[must_use]
    pub fn degree_intervals_iter(&self) -> DegreeIntervals<'_> {
        DegreeIntervals::new(self)
    }

    /// Build a twelve-tone equal-temperament pattern from semitone step sizes.
    ///
    /// # Errors
    ///
    /// Returns [`ScaleBuildError`] if the pattern is empty or if the registry lacks the provided
    /// system, preventing interval construction.
    pub fn from_twelve_tet_steps(
        semitone_steps: &[i32],
        system: &PitchSystemId,
        registry: &TuningRegistry,
    ) -> Result<Self, ScaleBuildError> {
        let mut steps = Vec::with_capacity(semitone_steps.len());
        let mut index = 0;
        for delta in semitone_steps {
            let start = Pitch::abstract_pitch(index, system.clone());
            index += *delta;
            let target = Pitch::abstract_pitch(index, system.clone());
            let interval = Interval::between(&start, &target, registry)?;
            steps.push(interval);
        }
        Self::from_steps(steps).map_err(ScaleBuildError::from)
    }
}

/// Iterator over cumulative degree intervals for a [`ScalePattern`].
#[derive(Clone)]
pub struct DegreeIntervals<'a> {
    steps: &'a [Interval],
    next_degree: usize,
    current_interval: Option<Interval>,
    empty: bool,
    halted: bool,
}

impl<'a> DegreeIntervals<'a> {
    fn new(pattern: &'a ScalePattern) -> Self {
        Self {
            steps: &pattern.steps,
            next_degree: 0,
            current_interval: None,
            empty: pattern.steps.is_empty(),
            halted: false,
        }
    }
}

impl Iterator for DegreeIntervals<'_> {
    type Item = Result<(usize, Interval), IntervalError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.halted {
            return None;
        }

        if self.empty {
            let degree = self.next_degree;
            self.next_degree += 1;
            return Some(Ok((degree, Interval::identity())));
        }

        if self.steps.is_empty() {
            return None;
        }

        if self.next_degree == 0 {
            self.next_degree = 1;
            return Some(Ok((0, Interval::identity())));
        }

        let len = self.steps.len();
        let step = &self.steps[(self.next_degree - 1) % len];
        let next_interval = match &self.current_interval {
            Some(interval) => match interval.compose(step) {
                Ok(value) => value,
                Err(err) => {
                    self.halted = true;
                    return Some(Err(err));
                }
            },
            None => step.clone(),
        };
        self.current_interval = Some(next_interval.clone());

        let degree = self.next_degree;
        self.next_degree += 1;
        Some(Ok((degree, next_interval)))
    }
}
