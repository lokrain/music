use core::fmt;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{
    TuningRegistry,
    pitch::{AbstractPitch, Pitch, PitchError},
    system::PitchSystemId,
};

/// Errors that can occur while constructing or manipulating [`Interval`] instances.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum IntervalError {
    /// Interval ratios must be finite (not `NaN` or infinite).
    NonFiniteRatio(f32),
    /// Interval ratios must be strictly positive.
    NonPositiveRatio(f32),
}

impl fmt::Display for IntervalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NonFiniteRatio(value) => write!(f, "interval ratio must be finite (got {value})"),
            Self::NonPositiveRatio(value) => {
                write!(f, "interval ratio must be positive (got {value})")
            }
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for IntervalError {}

/// Errors that can occur while deriving an [`Interval`] from two [`Pitch`] instances.
#[derive(Debug, Clone, PartialEq)]
pub enum IntervalBetweenError {
    /// Resolving one of the pitches failed (unknown system, invalid literal, etc.).
    Pitch(PitchError),
    /// Derived ratio was invalid even though both pitches resolved successfully.
    Interval {
        /// Ratio validation failure that prevented interval construction.
        source: IntervalError,
        /// Frequency provided by the target pitch, used to maintain backwards compatibility with
        /// [`PitchError::InvalidLiteralFrequency`].
        target_freq: f32,
    },
}

impl IntervalBetweenError {
    const fn interval(source: IntervalError, target_freq: f32) -> Self {
        Self::Interval {
            source,
            target_freq,
        }
    }
}

impl fmt::Display for IntervalBetweenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Pitch(err) => write!(f, "{err}"),
            Self::Interval { source, .. } => write!(f, "failed to build interval: {source}"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for IntervalBetweenError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Pitch(err) => Some(err),
            Self::Interval { source, .. } => Some(source),
        }
    }
}

impl From<PitchError> for IntervalBetweenError {
    fn from(value: PitchError) -> Self {
        Self::Pitch(value)
    }
}

/// Musical distance between two pitches.
///
/// Intervals retain the ratio between two resolved frequencies and, when possible, the raw step
/// offset within a specific [`PitchSystemId`]. This allows callers to reapply intervals to any
/// pitch—preserving abstract indices when the systems match and falling back to literal
/// frequencies otherwise.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Interval {
    ratio: f32,
    steps: Option<(PitchSystemId, i32)>,
}

impl fmt::Display for Interval {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some((system, steps)) = &self.steps {
            write!(f, "ratio={:.6}, steps={}@{}", self.ratio, steps, system)
        } else {
            write!(f, "ratio={:.6}", self.ratio)
        }
    }
}

impl Interval {
    /// Identity interval (ratio `1.0`, no preserved steps).
    #[must_use]
    pub const fn identity() -> Self {
        Self {
            ratio: 1.0,
            steps: None,
        }
    }

    fn ensure_valid_ratio(ratio: f32) -> Result<f32, IntervalError> {
        if !ratio.is_finite() {
            return Err(IntervalError::NonFiniteRatio(ratio));
        }
        if ratio <= 0.0 {
            return Err(IntervalError::NonPositiveRatio(ratio));
        }
        Ok(ratio)
    }

    fn from_ratio_with_steps(
        ratio: f32,
        steps: Option<(PitchSystemId, i32)>,
    ) -> Result<Self, IntervalError> {
        Ok(Self {
            ratio: Self::ensure_valid_ratio(ratio)?,
            steps,
        })
    }

    /// Construct an interval using an explicit ratio.
    ///
    /// # Errors
    ///
    /// Returns [`IntervalError`] when the ratio is non-finite or non-positive.
    pub fn from_ratio(ratio: f32) -> Result<Self, IntervalError> {
        Self::from_ratio_with_steps(ratio, None)
    }

    /// Build an interval from `base` to `target`, resolving frequencies through the provided
    /// registry when necessary.
    ///
    /// # Errors
    ///
    /// Propagates [`PitchError`] variants emitted while resolving either pitch.
    pub fn between(
        base: &Pitch,
        target: &Pitch,
        registry: &TuningRegistry,
    ) -> Result<Self, PitchError> {
        Self::try_between(base, target, registry).map_err(|err| match err {
            IntervalBetweenError::Pitch(err) => err,
            IntervalBetweenError::Interval { target_freq, .. } => {
                PitchError::InvalidLiteralFrequency(target_freq)
            }
        })
    }

    /// Build an interval from `base` to `target`, surfacing both pitch-resolution and ratio errors.
    ///
    /// # Errors
    ///
    /// Returns [`IntervalBetweenError::Pitch`] when either pitch fails to resolve through the
    /// registry, or [`IntervalBetweenError::Interval`] when the derived ratio is invalid.
    pub fn try_between(
        base: &Pitch,
        target: &Pitch,
        registry: &TuningRegistry,
    ) -> Result<Self, IntervalBetweenError> {
        let base_freq = base
            .try_freq_hz(registry)
            .map_err(IntervalBetweenError::from)?;
        let target_freq = target
            .try_freq_hz(registry)
            .map_err(IntervalBetweenError::from)?;
        let ratio = target_freq / base_freq;

        let steps = match (base, target) {
            (Pitch::Abstract(lhs), Pitch::Abstract(rhs)) if lhs.system == rhs.system => {
                Some((lhs.system.clone(), rhs.index - lhs.index))
            }
            _ => None,
        };

        Self::from_ratio_with_steps(ratio, steps)
            .map_err(|source| IntervalBetweenError::interval(source, target_freq))
    }

    /// Multiplicative ratio between the two resolved frequencies.
    #[must_use]
    pub const fn ratio(&self) -> f32 {
        self.ratio
    }

    /// Step offset when the interval originated from pitches within the same tuning system.
    #[must_use]
    pub fn steps(&self) -> Option<(i32, &PitchSystemId)> {
        self.steps.as_ref().map(|(system, delta)| (*delta, system))
    }

    /// Represent the interval in cents.
    #[must_use]
    pub fn cents(&self) -> f32 {
        self.ratio.log2() * 1200.0
    }

    /// Invert the interval (descending becomes ascending and vice versa).
    ///
    /// # Errors
    ///
    /// Returns [`IntervalError`] if the inverted ratio would be non-finite or non-positive.
    pub fn inverse(&self) -> Result<Self, IntervalError> {
        let steps = self
            .steps
            .as_ref()
            .map(|(system, delta)| (system.clone(), -*delta));
        Self::from_ratio_with_steps(1.0 / self.ratio, steps)
    }

    /// Convenience wrapper around [`Self::inverse`] that discards the error information.
    #[must_use]
    pub fn inverse_if_valid(&self) -> Option<Self> {
        self.inverse().ok()
    }

    /// Repeat the interval `times`, multiplying ratios and scaling stored steps when possible.
    ///
    /// # Errors
    ///
    /// Returns [`IntervalError`] if exponentiation yields a non-finite or non-positive ratio.
    pub fn powi(&self, times: i32) -> Result<Self, IntervalError> {
        if times == 0 {
            return Self::from_ratio_with_steps(
                1.0,
                self.steps.as_ref().map(|(system, _)| (system.clone(), 0)),
            );
        }

        let ratio = self.ratio.powi(times);
        let steps = self.steps.as_ref().and_then(|(system, delta)| {
            delta
                .checked_mul(times)
                .map(|value| (system.clone(), value))
        });
        Self::from_ratio_with_steps(ratio, steps)
    }

    /// Compose two intervals, yielding the combined musical distance.
    ///
    /// # Errors
    ///
    /// Returns [`IntervalError`] if the resulting ratio is invalid (non-finite or non-positive).
    pub fn compose(&self, other: &Self) -> Result<Self, IntervalError> {
        let ratio = self.ratio * other.ratio;
        let steps = match (self.steps.as_ref(), other.steps.as_ref()) {
            (Some((lhs_system, lhs_delta)), Some((rhs_system, rhs_delta)))
                if lhs_system == rhs_system =>
            {
                lhs_delta
                    .checked_add(*rhs_delta)
                    .map(|value| (lhs_system.clone(), value))
            }
            _ => None,
        };
        Self::from_ratio_with_steps(ratio, steps)
    }

    /// Convenience wrapper around [`Self::powi`] that discards the error information.
    #[must_use]
    pub fn powi_if_valid(&self, times: i32) -> Option<Self> {
        self.powi(times).ok()
    }

    /// Apply the interval to `pitch`, yielding a transposed pitch.
    ///
    /// Whenever both the interval and pitch reference the same tuning system, the result stays
    /// abstract. Otherwise, the interval falls back to literal frequency multiplication by the
    /// stored ratio.
    ///
    /// # Errors
    ///
    /// Returns [`PitchError`] when the pitch cannot be resolved through the registry.
    pub fn apply_to(&self, pitch: &Pitch, registry: &TuningRegistry) -> Result<Pitch, PitchError> {
        if let Pitch::Abstract(AbstractPitch { index, system }) = pitch
            && let Some((interval_system, delta)) = &self.steps
            && interval_system == system
        {
            return Ok(Pitch::Abstract(AbstractPitch {
                index: index + delta,
                system: system.clone(),
            }));
        }

        let freq = pitch.try_freq_hz(registry)? * self.ratio;
        Ok(Pitch::Frequency(freq))
    }
}
