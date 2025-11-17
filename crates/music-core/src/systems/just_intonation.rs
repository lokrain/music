use alloc::{borrow::Cow, vec::Vec};
use core::convert::TryFrom;

use crate::system::PitchSystem;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Simple ratio-based just intonation temperament with configurable scale degrees.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct JustIntonation {
    base_freq: f32,
    base_index: i32,
    ratios: Vec<f32>,
    label: Option<Cow<'static, str>>,
}

impl JustIntonation {
    /// Create a new just intonation mapping. `ratios` are relative to the base
    /// pitch (1.0 = unison) and represent one octave's worth of degrees.
    ///
    /// # Panics
    /// - If `ratios` is empty
    /// - If any ratio is non-positive
    #[must_use]
    pub fn new(base_freq: f32, base_index: i32, ratios: Vec<f32>) -> Self {
        assert!(!ratios.is_empty(), "ratios must contain at least one entry");
        assert!(
            ratios.iter().all(|ratio| *ratio > 0.0),
            "ratios must be positive"
        );
        Self {
            base_freq,
            base_index,
            ratios,
            label: None,
        }
    }

    /// Attach a label used by [`PitchSystem::name_of`].
    #[must_use]
    pub fn with_label(mut self, label: impl Into<Cow<'static, str>>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Canonical just-intonation major scale aligned to A4 = 440 Hz (index 69).
    #[must_use]
    pub fn a4_440_major() -> Self {
        Self::major_reference(440.0, 69)
    }

    /// Just-intonation major scale with a custom base reference.
    #[must_use]
    pub fn major_reference(base_freq: f32, base_index: i32) -> Self {
        Self::new(base_freq, base_index, Self::major_ratios()).with_label("JI-major")
    }

    fn major_ratios() -> Vec<f32> {
        vec![
            1.0,
            16.0 / 15.0,
            9.0 / 8.0,
            6.0 / 5.0,
            5.0 / 4.0,
            4.0 / 3.0,
            45.0 / 32.0,
            3.0 / 2.0,
            8.0 / 5.0,
            5.0 / 3.0,
            9.0 / 5.0,
            15.0 / 8.0,
        ]
    }

    fn ratio_for_steps(&self, steps: i32) -> f32 {
        let Ok(len) = i32::try_from(self.ratios.len()) else {
            panic!("ratio table exceeds i32 range");
        };
        let octave = steps.div_euclid(len);
        let Ok(degree) = usize::try_from(steps.rem_euclid(len)) else {
            panic!("degree index must be non-negative");
        };
        self.ratios[degree] * 2.0_f32.powi(octave)
    }
}

impl PitchSystem for JustIntonation {
    fn to_frequency(&self, index: i32) -> f32 {
        let steps = index - self.base_index;
        self.base_freq * self.ratio_for_steps(steps)
    }

    fn name_of(&self, index: i32) -> Option<String> {
        self.label.as_ref().map(|label| format!("{label}({index})"))
    }
}
