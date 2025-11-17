use alloc::{borrow::Cow, vec::Vec};
use core::convert::TryFrom;

use num_traits::ToPrimitive;

use crate::system::PitchSystem;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Describes a custom octave-repeating scale defined by cent offsets.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CentsScale {
    base_freq: f32,
    base_index: i32,
    cents: Vec<f32>,
    label: Option<Cow<'static, str>>,
}

impl CentsScale {
    /// Create a new cent-based scale. The list typically begins with 0.0 (unison)
    /// and must cover a single octave.
    ///
    /// # Panics
    /// - If `cents` is empty
    #[must_use]
    pub fn new(base_freq: f32, base_index: i32, cents: Vec<f32>) -> Self {
        assert!(!cents.is_empty(), "cents must contain at least one value");
        Self {
            base_freq,
            base_index,
            cents,
            label: None,
        }
    }

    /// Attach a display label (e.g., "24-EDO") used by [`PitchSystem::name_of`].
    #[must_use]
    pub fn with_label(mut self, label: impl Into<Cow<'static, str>>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Quarter-tone helper (24 notes per octave, 50 cents each).
    #[must_use]
    pub fn a4_440_quarter_tone() -> Self {
        let mut cents = Vec::with_capacity(24);
        let mut value = 0.0;
        for _ in 0..24 {
            cents.push(value);
            value += 50.0;
        }
        Self::new(440.0, 69, cents).with_label("24-EDO")
    }

    fn ratio_for_steps(&self, steps: i32) -> f32 {
        let Ok(len) = i32::try_from(self.cents.len()) else {
            panic!("scale length exceeds i32 range");
        };
        let octave = steps.div_euclid(len);
        let Ok(degree) = usize::try_from(steps.rem_euclid(len)) else {
            panic!("degree index must be non-negative");
        };
        let cents = self.cents[degree];
        let cents_total = f64::from(octave).mul_add(1200.0, f64::from(cents));
        let ratio = (cents_total / 1200.0).exp2();
        ratio
            .to_f32()
            .unwrap_or_else(|| panic!("ratio exceeds f32 range"))
    }
}

impl PitchSystem for CentsScale {
    fn to_frequency(&self, index: i32) -> f32 {
        let steps = index - self.base_index;
        self.base_freq * self.ratio_for_steps(steps)
    }

    fn name_of(&self, index: i32) -> Option<String> {
        self.label.as_ref().map(|label| format!("{label}({index})"))
    }
}
