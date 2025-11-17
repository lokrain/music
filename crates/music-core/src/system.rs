use alloc::{borrow::Cow, string::String};
use core::{borrow::Borrow, fmt, ops::Deref, str::FromStr};

use num_traits::ToPrimitive;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Opaque identifier for a tuning system.
///
/// Identifiers must be non-empty (after trimming) and free of ASCII control characters. Use
/// [`PitchSystemId::try_new`] or [`core::str::FromStr`] when validating user input; the infallible
/// constructors are intended for trusted literals.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PitchSystemId(String);

/// Validation errors encountered while constructing [`PitchSystemId`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PitchSystemIdError {
    /// Identifier was empty or consisted solely of whitespace.
    Empty,
    /// Identifier contained ASCII control characters (newline, tab, etc.).
    ContainsControl(char),
}

impl fmt::Display for PitchSystemIdError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => f.write_str("pitch system identifier must not be empty"),
            Self::ContainsControl(ch) => {
                write!(
                    f,
                    "pitch system identifier contains control character {ch:?}"
                )
            }
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for PitchSystemIdError {}

impl PitchSystemId {
    /// Create a new identifier from an owned or borrowed string.
    ///
    /// This constructor does **not** validate the identifier. Prefer [`Self::try_new`] for
    /// user-provided data.
    #[must_use]
    pub fn new<S: Into<String>>(name: S) -> Self {
        Self(name.into())
    }

    /// Fallible constructor that validates identifier contents.
    ///
    /// # Errors
    ///
    /// Returns [`PitchSystemIdError`] when the provided identifier is empty or contains control
    /// characters.
    pub fn try_new(name: impl AsRef<str>) -> Result<Self, PitchSystemIdError> {
        let name_ref = name.as_ref();
        validate_identifier(name_ref)?;
        Ok(Self(name_ref.to_owned()))
    }

    /// Borrow the identifier as a string slice.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consume the identifier, yielding the underlying `String`.
    #[must_use]
    pub fn into_inner(self) -> String {
        self.0
    }
}

impl From<String> for PitchSystemId {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&str> for PitchSystemId {
    fn from(value: &str) -> Self {
        Self(value.to_owned())
    }
}

impl From<&String> for PitchSystemId {
    fn from(value: &String) -> Self {
        Self(value.clone())
    }
}

impl From<PitchSystemId> for String {
    fn from(value: PitchSystemId) -> Self {
        value.into_inner()
    }
}

impl FromStr for PitchSystemId {
    type Err = PitchSystemIdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_new(s)
    }
}

impl AsRef<str> for PitchSystemId {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl Borrow<str> for PitchSystemId {
    fn borrow(&self) -> &str {
        self.as_str()
    }
}

impl PartialEq<str> for PitchSystemId {
    fn eq(&self, other: &str) -> bool {
        self.as_str() == other
    }
}

impl PartialEq<PitchSystemId> for str {
    fn eq(&self, other: &PitchSystemId) -> bool {
        self == other.as_str()
    }
}

impl fmt::Display for PitchSystemId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl Deref for PitchSystemId {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

/// Behavior of any tuning system (12-TET, 24-TET, JI, Scala, etc.).
pub trait PitchSystem: Send + Sync {
    /// Convert abstract pitch index to frequency in Hz.
    fn to_frequency(&self, index: i32) -> f32;

    /// Optional symbolic name for the pitch.
    fn name_of(&self, _index: i32) -> Option<String> {
        None
    }
}

/// Generic equal-temperament helper storing shared configuration for ET systems.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct EqualTemperament {
    base_freq: f32,
    base_index: i32,
    steps_per_octave: u32,
    label: Option<Cow<'static, str>>,
}

impl EqualTemperament {
    /// Create an equal-temperament system.
    ///
    /// # Panics
    ///
    /// Panics if `steps_per_octave` is zero.
    #[must_use]
    pub fn new(steps_per_octave: u32, base_freq: f32, base_index: i32) -> Self {
        assert!(steps_per_octave > 0, "steps_per_octave must be non-zero");
        Self {
            base_freq,
            base_index,
            steps_per_octave,
            label: None,
        }
    }

    /// Attach a display label (e.g., "12-TET") used by [`PitchSystem::name_of`].
    #[must_use]
    pub fn with_label(mut self, label: impl Into<Cow<'static, str>>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Steps per octave for this temperament.
    #[must_use]
    pub const fn steps_per_octave(&self) -> u32 {
        self.steps_per_octave
    }

    /// Base frequency anchor (index = [`Self::base_index`]).
    #[must_use]
    pub const fn base_freq(&self) -> f32 {
        self.base_freq
    }

    /// Base index used as the frequency reference.
    #[must_use]
    pub const fn base_index(&self) -> i32 {
        self.base_index
    }
}

impl PitchSystem for EqualTemperament {
    fn to_frequency(&self, index: i32) -> f32 {
        let steps = f64::from(index - self.base_index);
        let step_ratio = steps / f64::from(self.steps_per_octave);
        let freq = f64::from(self.base_freq) * step_ratio.exp2();
        freq.to_f32()
            .unwrap_or_else(|| panic!("frequency exceeds f32 range"))
    }

    fn name_of(&self, index: i32) -> Option<String> {
        self.label.as_ref().map(|label| format!("{label}({index})"))
    }
}

fn validate_identifier(value: &str) -> Result<(), PitchSystemIdError> {
    if value.trim().is_empty() {
        return Err(PitchSystemIdError::Empty);
    }
    if let Some(ch) = value.chars().find(|ch| ch.is_control()) {
        return Err(PitchSystemIdError::ContainsControl(ch));
    }
    Ok(())
}
