use alloc::vec::Vec;

use crate::{
    TuningRegistry,
    interval::{Interval, IntervalError},
    pitch::{AbstractPitch, Pitch, PitchError},
    system::PitchSystemId,
};

use super::{
    ScaleBuildError, ScaleModeError,
    iterators::{BoundedScaleDegrees, ScaleDegrees},
    pattern::ScalePattern,
};

/// Scale anchored at a root pitch with an associated step pattern.
#[derive(Debug, Clone)]
pub struct Scale {
    pub(super) root: Pitch,
    pub(super) pattern: ScalePattern,
}

impl Scale {
    /// Build a scale from an abstract root pitch.
    #[must_use]
    pub const fn from_abstract_root(root: AbstractPitch, pattern: ScalePattern) -> Self {
        Self {
            root: Pitch::Abstract(root),
            pattern,
        }
    }

    /// Build a scale from any pitch (literal or abstract).
    #[must_use]
    pub const fn from_pitch(root: Pitch, pattern: ScalePattern) -> Self {
        Self { root, pattern }
    }

    /// Construct a twelve-tone equal temperament scale from semitone step sizes.
    ///
    /// # Errors
    ///
    /// Returns [`ScaleBuildError`] if the step list is empty or if the registry cannot generate
    /// the requested intervals.
    pub fn from_twelve_tet_steps(
        root_index: i32,
        system: PitchSystemId,
        semitone_steps: &[i32],
        registry: &TuningRegistry,
    ) -> Result<Self, ScaleBuildError> {
        let pattern = ScalePattern::from_twelve_tet_steps(semitone_steps, &system, registry)?;
        Ok(Self::from_abstract_root(
            AbstractPitch::new(root_index, system),
            pattern,
        ))
    }

    /// Convenience constructor for the standard major pattern in twelve-tone equal temperament.
    ///
    /// # Errors
    ///
    /// Propagates [`ScaleBuildError`] from [`Scale::from_twelve_tet_steps`].
    pub fn twelve_tet_major(
        root_index: i32,
        system: PitchSystemId,
        registry: &TuningRegistry,
    ) -> Result<Self, ScaleBuildError> {
        const MAJOR: [i32; 7] = [2, 2, 1, 2, 2, 2, 1];
        Self::from_twelve_tet_steps(root_index, system, &MAJOR, registry)
    }

    /// Access the root pitch.
    #[must_use]
    pub const fn root(&self) -> &Pitch {
        &self.root
    }

    /// Access the underlying pattern.
    #[must_use]
    pub const fn pattern(&self) -> &ScalePattern {
        &self.pattern
    }

    /// Interval between the root and the requested degree.
    ///
    /// # Errors
    ///
    /// Returns [`IntervalError`] if the composed interval becomes invalid.
    pub fn degree_interval(&self, degree: usize) -> Result<Interval, IntervalError> {
        self.pattern.degree_interval(degree)
    }

    /// Intervals between the root and every degree up to `highest_degree` (inclusive).
    ///
    /// This is a thin wrapper around [`ScalePattern::degree_intervals`], exposed here so callers
    /// working purely with [`Scale`] do not need to reach into the pattern directly.
    ///
    /// # Errors
    ///
    /// Returns [`IntervalError`] if intermediate composition produces an invalid ratio.
    pub fn degree_intervals(&self, highest_degree: usize) -> Result<Vec<Interval>, IntervalError> {
        self.pattern.degree_intervals(highest_degree)
    }

    /// Lazily iterate over degree intervals without requiring registry access.
    #[must_use]
    pub fn degree_interval_iter(&self) -> super::pattern::DegreeIntervals<'_> {
        self.pattern.degree_intervals_iter()
    }

    /// Number of steps in a single-octave traversal of the scale.
    #[must_use]
    pub const fn step_count(&self) -> usize {
        self.pattern.len()
    }

    /// Resolve pitches for every degree up to `highest_degree`, starting from the root.
    ///
    /// The returned vector always contains the root at index `0`, with subsequent degrees wrapping
    /// around the pattern as needed.
    ///
    /// # Errors
    ///
    /// Propagates [`PitchError`] variants that occur while applying interval steps.
    pub fn degree_pitches(
        &self,
        highest_degree: usize,
        registry: &TuningRegistry,
    ) -> Result<Vec<Pitch>, PitchError> {
        let mut pitches = Vec::with_capacity(highest_degree + 1);
        pitches.push(self.root.clone());

        if highest_degree == 0 || self.pattern.steps.is_empty() {
            return Ok(pitches);
        }

        let len = self.pattern.steps.len();
        let mut current = self.root.clone();
        for idx in 0..highest_degree {
            let step = &self.pattern.steps[idx % len];
            current = step.apply_to(&current, registry)?;
            pitches.push(current.clone());
        }
        Ok(pitches)
    }

    /// Resolve the pitch at `degree`, counting from zero.
    ///
    /// Degree `0` returns the root, `1` the first step above the root, etc. Higher degrees wrap
    /// around the pattern, effectively continuing into additional octaves.
    ///
    /// # Errors
    ///
    /// Propagates [`PitchError`] variants that occur while applying interval steps.
    pub fn degree_pitch(
        &self,
        degree: usize,
        registry: &TuningRegistry,
    ) -> Result<Pitch, PitchError> {
        if degree == 0 || self.pattern.steps.is_empty() {
            return Ok(self.root.clone());
        }

        let mut pitch = self.root.clone();
        let len = self.pattern.steps.len();
        for idx in 0..degree {
            let step = &self.pattern.steps[idx % len];
            pitch = step.apply_to(&pitch, registry)?;
        }
        Ok(pitch)
    }

    /// Lazily iterate over scale degrees, yielding `(degree, interval, pitch)` triples.
    #[must_use]
    pub fn degrees<'a>(&'a self, registry: &'a TuningRegistry) -> ScaleDegrees<'a> {
        ScaleDegrees::new(self, registry)
    }

    /// Convenience wrapper around [`Scale::degrees`] that caps iteration to `highest_degree`,
    /// returning a [`BoundedScaleDegrees`] iterator with a precise length.
    #[must_use]
    pub fn degrees_up_to<'a>(
        &'a self,
        highest_degree: usize,
        registry: &'a TuningRegistry,
    ) -> BoundedScaleDegrees<'a> {
        BoundedScaleDegrees::new(self, highest_degree, registry)
    }

    /// Produce a modal rotation of the scale, shifting the root to `degree` and rotating the pattern.
    ///
    /// # Errors
    ///
    /// Propagates [`PitchError`] if the new root cannot be resolved.
    pub fn mode(&self, degree: usize, registry: &TuningRegistry) -> Result<Self, PitchError> {
        if self.pattern.steps.is_empty() {
            return Ok(self.clone());
        }

        let new_root = self.degree_pitch(degree, registry)?;
        let rotated_pattern = self.pattern.rotate(degree % self.pattern.steps.len());
        Ok(Self {
            root: new_root,
            pattern: rotated_pattern,
        })
    }

    /// Rotate the scale backwards by `degree` steps, restoring previous modal orientations.
    ///
    /// # Errors
    ///
    /// Returns [`ScaleModeError`] if interval inversion fails or a pitch cannot be resolved.
    pub fn mode_back(
        &self,
        degree: usize,
        registry: &TuningRegistry,
    ) -> Result<Self, ScaleModeError> {
        if self.pattern.steps.is_empty() || degree == 0 {
            return Ok(self.clone());
        }

        let new_root = self.shift_root_backward(degree, registry)?;
        let rotation = if self.pattern.is_empty() {
            0
        } else {
            (self.pattern.len() - (degree % self.pattern.len())) % self.pattern.len()
        };
        let rotated_pattern = self.pattern.rotate(rotation);
        Ok(Self {
            root: new_root,
            pattern: rotated_pattern,
        })
    }

    /// Rotate the scale by a signed offset, supporting both forward and backward traversal.
    ///
    /// # Errors
    ///
    /// Returns [`ScaleModeError`] if pitch resolution fails or an intermediate interval cannot be
    /// inverted.
    pub fn mode_with_offset(
        &self,
        offset: isize,
        registry: &TuningRegistry,
    ) -> Result<Self, ScaleModeError> {
        if self.pattern.steps.is_empty() || offset == 0 {
            return Ok(self.clone());
        }
        if offset > 0 {
            let forward = offset.abs_diff(0);
            return self.mode(forward, registry).map_err(ScaleModeError::from);
        }
        self.mode_back(offset.abs_diff(0), registry)
    }

    fn shift_root_backward(
        &self,
        steps: usize,
        registry: &TuningRegistry,
    ) -> Result<Pitch, ScaleModeError> {
        let total_steps = self.pattern.steps.len();
        if total_steps == 0 || steps == 0 {
            return Ok(self.root.clone());
        }

        let mut pitch = self.root.clone();
        for offset in 0..steps {
            let idx = (total_steps + total_steps - 1 - (offset % total_steps)) % total_steps;
            let step = &self.pattern.steps[idx];
            let inverse = step.inverse().map_err(ScaleModeError::from)?;
            pitch = inverse
                .apply_to(&pitch, registry)
                .map_err(ScaleModeError::from)?;
        }
        Ok(pitch)
    }
}
