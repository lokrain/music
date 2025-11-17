use core::ops::Deref;

use crate::system::{EqualTemperament, PitchSystem};

/// Classic 12-tone equal temperament helper built atop [`EqualTemperament`].
#[derive(Debug, Clone)]
pub struct TwelveTET {
    inner: EqualTemperament,
}

impl TwelveTET {
    /// Build a 12-TET with custom base frequency/index.
    #[must_use]
    pub fn new(base_freq: f32, base_index: i32) -> Self {
        Self {
            inner: EqualTemperament::new(12, base_freq, base_index).with_label("12-TET"),
        }
    }

    /// Standard tuning where index 69 corresponds to 440 Hz.
    #[must_use]
    pub fn a4_440() -> Self {
        Self::new(440.0, 69)
    }

    /// Access the underlying [`EqualTemperament`] instance.
    #[must_use]
    pub const fn inner(&self) -> &EqualTemperament {
        &self.inner
    }
}

impl Default for TwelveTET {
    fn default() -> Self {
        Self::a4_440()
    }
}

impl Deref for TwelveTET {
    type Target = EqualTemperament;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl PitchSystem for TwelveTET {
    fn to_frequency(&self, index: i32) -> f32 {
        self.inner.to_frequency(index)
    }

    fn name_of(&self, index: i32) -> Option<String> {
        self.inner.name_of(index)
    }
}
