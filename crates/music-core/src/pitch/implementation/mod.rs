mod core;
mod intervals;
mod labels;
mod resolution;
mod state;

pub use core::Pitch;

use super::PitchError;

/// Validate that a literal frequency is finite and positive.
///
/// # Errors
///
/// Returns [`PitchError::InvalidLiteralFrequency`] when `freq` is non-finite or not positive.
pub const fn validate_literal_frequency(freq: f32) -> Result<f32, PitchError> {
    if freq.is_finite() && freq.is_sign_positive() {
        Ok(freq)
    } else {
        Err(PitchError::InvalidLiteralFrequency(freq))
    }
}
