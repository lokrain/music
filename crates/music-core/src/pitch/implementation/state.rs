use crate::{pitch::AbstractPitch, system::PitchSystemId};

use super::Pitch;

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
}
