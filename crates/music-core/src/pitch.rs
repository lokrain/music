use alloc::string::String;
use core::{
    convert::TryFrom,
    fmt,
    ops::{Add, AddAssign, Sub, SubAssign},
};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{
    registry::{TuningError, TuningRegistry},
    system::PitchSystemId,
};

/// Default tolerance (in Hz) used by helper utilities such as [`Pitch::approx_eq`].
pub const DEFAULT_FREQUENCY_EPSILON: f32 = 1.0e-4;

/// Rich label metadata describing how a pitch should be rendered to humans.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum PitchLabel {
    /// Symbolic name provided by the tuning system (e.g., "12-TET(69)").
    Named(String),
    /// Literal frequency fallback, rendered in Hz.
    Frequency(f32),
}

impl PitchLabel {
    /// Returns the label as a user-facing string.
    #[must_use]
    pub fn to_string_lossy(&self) -> String {
        self.to_string()
    }

    /// Access the literal frequency if this label represents one.
    #[must_use]
    pub const fn as_frequency(&self) -> Option<f32> {
        match self {
            Self::Named(_) => None,
            Self::Frequency(freq) => Some(*freq),
        }
    }

    /// True when the label is symbolic (named) instead of numeric.
    #[must_use]
    pub const fn is_symbolic(&self) -> bool {
        matches!(self, Self::Named(_))
    }
}

impl fmt::Display for PitchLabel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Named(name) => f.write_str(name),
            Self::Frequency(freq) => write!(f, "{freq:.3} Hz"),
        }
    }
}

/// Errors emitted when manipulating or resolving pitches.
#[derive(Debug, Clone, PartialEq)]
pub enum PitchError {
    /// Registry does not contain the requested tuning system identifier.
    UnknownSystem(PitchSystemId),
    /// Literal frequency pitches must be finite and positive.
    InvalidLiteralFrequency(f32),
    /// Caller attempted to fetch a symbolic name for a pitch that lacks one.
    NameUnavailable { system: PitchSystemId, index: i32 },
    /// Caller expected an abstract pitch but encountered a literal frequency.
    NotAbstract,
    /// Literal pitches cannot yield symbolic names.
    LiteralHasNoName,
}

impl fmt::Display for PitchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownSystem(id) => write!(f, "unknown tuning system: {id}"),
            Self::InvalidLiteralFrequency(freq) => {
                write!(f, "invalid literal frequency: {freq}")
            }
            Self::NameUnavailable { system, index } => write!(
                f,
                "tuning system {system} does not provide a name for index {index}"
            ),
            Self::NotAbstract => f.write_str("pitch is not abstract"),
            Self::LiteralHasNoName => {
                f.write_str("literal frequency pitches do not have symbolic names")
            }
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for PitchError {}

impl From<TuningError> for PitchError {
    fn from(value: TuningError) -> Self {
        match value {
            TuningError::UnknownSystem(id) => Self::UnknownSystem(id),
        }
    }
}

/// A musical pitch: either a literal frequency, or an abstract pitch interpreted via a tuning system.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Pitch {
    /// Literal frequency in Hz.
    Frequency(f32),

    /// Abstract pitch (index + tuning system).
    Abstract(AbstractPitch),
}

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

impl TryFrom<Pitch> for AbstractPitch {
    type Error = PitchError;

    fn try_from(value: Pitch) -> Result<Self, Self::Error> {
        match value {
            Pitch::Frequency(_) => Err(PitchError::NotAbstract),
            Pitch::Abstract(pitch) => Ok(pitch),
        }
    }
}

impl Pitch {
    /// Convenience: literal frequency pitch.
    #[must_use]
    pub const fn hz(freq: f32) -> Self {
        Self::Frequency(freq)
    }

    /// Convenience: abstract pitch.
    #[must_use]
    pub const fn abstract_pitch(index: i32, system: PitchSystemId) -> Self {
        Self::Abstract(AbstractPitch::new(index, system))
    }

    /// True when the pitch is a literal frequency.
    #[must_use]
    pub const fn is_frequency(&self) -> bool {
        matches!(self, Self::Frequency(_))
    }

    /// True when the pitch is abstract and must be resolved via a registry.
    #[must_use]
    pub const fn is_abstract(&self) -> bool {
        matches!(self, Self::Abstract(_))
    }

    /// Access the literal frequency without resolution when available.
    #[must_use]
    pub const fn as_frequency(&self) -> Option<f32> {
        match self {
            Self::Frequency(freq) => Some(*freq),
            Self::Abstract(_) => None,
        }
    }

    /// Access the abstract pitch metadata when available.
    #[must_use]
    pub const fn as_abstract(&self) -> Option<&AbstractPitch> {
        match self {
            Self::Frequency(_) => None,
            Self::Abstract(value) => Some(value),
        }
    }

    /// Borrow the system identifier if this is an abstract pitch.
    #[must_use]
    pub fn system_id(&self) -> Option<&PitchSystemId> {
        self.as_abstract().map(|pitch| &pitch.system)
    }

    /// Borrow the abstract pitch index when applicable.
    #[must_use]
    pub fn index(&self) -> Option<i32> {
        self.as_abstract().map(|pitch| pitch.index)
    }

    /// Apply a transformation to the abstract pitch variant, leaving literal
    /// frequencies untouched.
    #[must_use]
    pub fn map_abstract<F>(&self, func: F) -> Self
    where
        F: FnOnce(&AbstractPitch) -> AbstractPitch,
    {
        match self {
            Self::Frequency(freq) => Self::Frequency(*freq),
            Self::Abstract(value) => Self::Abstract(func(value)),
        }
    }

    /// Convenience for transposing abstract pitches while leaving literal
    /// frequencies unchanged.
    #[must_use]
    pub fn transpose(&self, steps: i32) -> Self {
        self.map_abstract(|pitch| pitch.transpose(steps))
    }

    /// Resolve to a literal-frequency pitch while preserving the original on success.
    ///
    /// # Errors
    ///
    /// Returns [`PitchError::UnknownSystem`] if the pitch's tuning system is absent.
    #[must_use = "discarding the resolved pitch means the computation was wasted"]
    pub fn resolved(&self, registry: &TuningRegistry) -> Result<Self, PitchError> {
        self.clone().into_resolved(registry)
    }

    /// Resolve to a literal-frequency pitch, consuming `self` when successful.
    ///
    /// # Errors
    ///
    /// Returns [`PitchError::UnknownSystem`] when the associated tuning system is absent or
    /// [`PitchError::InvalidLiteralFrequency`] for invalid literal pitches.
    pub fn into_resolved(self, registry: &TuningRegistry) -> Result<Self, PitchError> {
        match self {
            Self::Frequency(freq) => Ok(Self::Frequency(validate_literal_frequency(freq)?)),
            Self::Abstract(_) => self.try_freq_hz(registry).map(Self::Frequency),
        }
    }

    /// Resolve to a frequency using a registry (if needed).
    #[must_use]
    pub fn freq_hz(&self, registry: &TuningRegistry) -> Option<f32> {
        self.try_freq_hz(registry).ok()
    }

    /// Resolve to a frequency, returning a descriptive error when the system is missing.
    ///
    /// # Errors
    ///
    /// Returns [`PitchError::UnknownSystem`] if the tuning system cannot be found.
    pub fn try_freq_hz(&self, registry: &TuningRegistry) -> Result<f32, PitchError> {
        match self {
            Self::Frequency(f) => validate_literal_frequency(*f),
            Self::Abstract(AbstractPitch { index, system }) => registry
                .resolve_frequency(system, *index)
                .map_err(PitchError::from),
        }
    }

    /// Consume the pitch while resolving it into a literal frequency.
    ///
    /// # Errors
    ///
    /// Propagates [`PitchError::UnknownSystem`] and [`PitchError::InvalidLiteralFrequency`].
    pub fn into_freq_hz(self, registry: &TuningRegistry) -> Result<f32, PitchError> {
        match self {
            Self::Frequency(f) => validate_literal_frequency(f),
            Self::Abstract(_) => self.try_freq_hz(registry),
        }
    }

    /// Human-friendly label if the tuning system provides one (falls back to literal frequency).
    #[must_use]
    pub fn name(&self, registry: &TuningRegistry) -> Option<PitchLabel> {
        self.try_label(registry).ok()
    }

    /// Return detailed label metadata, including numeric fallback when symbolic names are absent.
    ///
    /// # Errors
    ///
    /// Returns [`PitchError::UnknownSystem`] when the tuning registry does not contain the system
    /// backing an abstract pitch, or [`PitchError::InvalidLiteralFrequency`] for malformed
    /// literal pitches.
    pub fn try_label(&self, registry: &TuningRegistry) -> Result<PitchLabel, PitchError> {
        match self {
            Self::Frequency(freq) => Ok(PitchLabel::Frequency(validate_literal_frequency(*freq)?)),
            Self::Abstract(AbstractPitch { index, system }) => {
                let system_ref = registry.resolve_system(system).map_err(PitchError::from)?;
                if let Some(name) = system_ref.name_of(*index) {
                    return Ok(PitchLabel::Named(name));
                }
                let freq = system_ref.to_frequency(*index);
                Ok(PitchLabel::Frequency(freq))
            }
        }
    }

    /// Human-friendly name with error awareness that only succeeds when a symbolic label exists.
    ///
    /// # Errors
    ///
    /// Returns [`PitchError::UnknownSystem`] when the tuning system cannot be found or
    /// [`PitchError::NameUnavailable`] / [`PitchError::LiteralHasNoName`] when no symbolic name is
    /// available for the target pitch.
    pub fn try_name(&self, registry: &TuningRegistry) -> Result<String, PitchError> {
        match self {
            Self::Frequency(_) => Err(PitchError::LiteralHasNoName),
            Self::Abstract(AbstractPitch { index, system }) => {
                let system_ref = registry.resolve_system(system).map_err(PitchError::from)?;
                system_ref
                    .name_of(*index)
                    .ok_or_else(|| PitchError::NameUnavailable {
                        system: system.clone(),
                        index: *index,
                    })
            }
        }
    }

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

    /// Compute the cents offset between two pitches (positive when `self` is sharper than `reference`).
    /// Compute the cents offset between two pitches (positive when `self` is sharper than
    /// `reference`).
    ///
    /// # Errors
    ///
    /// Returns [`PitchError::UnknownSystem`] or [`PitchError::InvalidLiteralFrequency`] when
    /// either pitch cannot be resolved.
    #[allow(clippy::cast_possible_truncation)]
    pub fn cents_offset(
        &self,
        reference: &Self,
        registry: &TuningRegistry,
    ) -> Result<f32, PitchError> {
        let lhs = self.try_freq_hz(registry)?;
        let rhs = reference.try_freq_hz(registry)?;
        let ratio = f64::from(lhs) / f64::from(rhs);
        Ok((ratio.log2() * 1200.0) as f32)
    }
}

impl fmt::Display for Pitch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Frequency(freq) => write!(f, "{freq:.3} Hz"),
            Self::Abstract(abstract_pitch) => abstract_pitch.fmt(f),
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

impl From<AbstractPitch> for Pitch {
    fn from(pitch: AbstractPitch) -> Self {
        Self::Abstract(pitch)
    }
}

#[allow(clippy::missing_const_for_fn)]
fn validate_literal_frequency(freq: f32) -> Result<f32, PitchError> {
    if freq.is_finite() && freq.is_sign_positive() {
        Ok(freq)
    } else {
        Err(PitchError::InvalidLiteralFrequency(freq))
    }
}
