use crate::{
    interval::{Interval, IntervalBetweenError},
    pitch::PitchError,
    registry::TuningRegistry,
};

use super::Pitch;

impl Pitch {
    /// Compare two pitches using a configurable tolerance (in Hz).
    ///
    /// # Errors
    ///
    /// Returns [`PitchError::UnknownSystem`] whenever either pitch references an unregistered
    /// tuning system or [`PitchError::InvalidLiteralFrequency`] for malformed literal values.
    pub fn approx_eq(
        &self,
        other: &Self,
        registry: &TuningRegistry,
        epsilon: f32,
    ) -> Result<bool, PitchError> {
        let lhs = self.try_freq_hz(registry)?;
        let rhs = other.try_freq_hz(registry)?;
        let threshold = epsilon.max(f32::EPSILON);
        Ok((lhs - rhs).abs() <= threshold)
    }

    /// Derive an [`Interval`] from `self` to `other` using the provided registry.
    ///
    /// # Errors
    ///
    /// Propagates [`PitchError`] variants when either pitch cannot be resolved through the registry.
    pub fn interval_to(
        &self,
        other: &Self,
        registry: &TuningRegistry,
    ) -> Result<Interval, PitchError> {
        Interval::between(self, other, registry)
    }

    /// Derive an [`Interval`] from `self` to `other`, surfacing richer diagnostics when failures
    /// occur.
    ///
    /// # Errors
    ///
    /// Returns [`IntervalBetweenError::Pitch`] when either pitch cannot be resolved through the
    /// registry, or [`IntervalBetweenError::Interval`] when the computed ratio is invalid.
    pub fn try_interval_to(
        &self,
        other: &Self,
        registry: &TuningRegistry,
    ) -> Result<Interval, IntervalBetweenError> {
        Interval::try_between(self, other, registry)
    }

    /// Transpose the pitch by the supplied [`Interval`], returning the resulting pitch.
    ///
    /// When the interval preserves step information for the same tuning system, the result stays
    /// abstract; otherwise, it falls back to literal frequency multiplication.
    ///
    /// # Errors
    ///
    /// Returns [`PitchError`] when the pitch cannot be resolved through the registry.
    pub fn transpose_interval(
        &self,
        interval: &Interval,
        registry: &TuningRegistry,
    ) -> Result<Self, PitchError> {
        interval.apply_to(self, registry)
    }

    /// Compute the cents offset between two pitches (positive when `self` is sharper than
    /// `reference`).
    ///
    /// # Errors
    ///
    /// Returns [`PitchError::UnknownSystem`] or [`PitchError::InvalidLiteralFrequency`] when
    /// either pitch cannot be resolved.
    pub fn cents_offset(
        &self,
        reference: &Self,
        registry: &TuningRegistry,
    ) -> Result<f32, PitchError> {
        let lhs = self.try_freq_hz(registry)?;
        let rhs = reference.try_freq_hz(registry)?;
        let ratio = lhs / rhs;
        Ok(ratio.log2() * 1200.0)
    }
}
