use core::ops::Deref;

use crate::system::{EqualTemperament, PitchSystem};

/// 24-tone equal temperament (quarter-tone steps).
#[derive(Debug, Clone)]
pub struct TwentyFourTET {
    inner: EqualTemperament,
}

impl TwentyFourTET {
    /// Build a 24-TET temperament with a custom base pitch reference.
    #[must_use]
    pub fn new(base_freq: f32, base_index: i32) -> Self {
        Self {
            inner: EqualTemperament::new(24, base_freq, base_index).with_label("24-TET"),
        }
    }

    /// Conventional setup matching 440 Hz at index 69.
    #[must_use]
    pub fn a4_440() -> Self {
        Self::new(440.0, 69)
    }

    /// Access the shared equal-temperament parameters.
    #[must_use]
    pub const fn inner(&self) -> &EqualTemperament {
        &self.inner
    }
}

impl Default for TwentyFourTET {
    fn default() -> Self {
        Self::a4_440()
    }
}

impl Deref for TwentyFourTET {
    type Target = EqualTemperament;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl PitchSystem for TwentyFourTET {
    fn to_frequency(&self, index: i32) -> f32 {
        self.inner.to_frequency(index)
    }

    fn name_of(&self, index: i32) -> Option<String> {
        self.inner.name_of(index)
    }
}
