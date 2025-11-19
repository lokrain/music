//! Musical intervals representing the distance between two pitches.
//!
//! This module provides the [`Interval`] type, which captures both the frequency ratio and
//! optional step offset between pitches. When intervals are derived from pitches within the same
//! tuning system, they preserve the abstract step delta, allowing precise transposition that
//! maintains the original pitch-class structure.
//!
//! # Examples
//!
//! ```
//! use music_core::{
//!     interval::Interval,
//!     pitch::Pitch,
//!     TuningRegistry,
//!     PitchSystemId,
//!     systems::TwelveTET,
//! };
//!
//! let registry = TuningRegistry::new()
//!     .with_system(PitchSystemId::from("12tet"), TwelveTET::a4_440());
//!
//! // Create interval from explicit ratio
//! let octave = Interval::from_ratio(2.0).unwrap();
//! assert_eq!(octave.ratio(), 2.0);
//! assert_eq!(octave.cents(), 1200.0);
//!
//! // Derive interval from two pitches
//! let a4 = Pitch::abstract_pitch(69, PitchSystemId::from("12tet"));
//! let a5 = Pitch::abstract_pitch(81, PitchSystemId::from("12tet"));
//! let interval = Interval::between(&a4, &a5, &registry).unwrap();
//! assert_eq!(interval.ratio(), 2.0);
//! assert_eq!(interval.steps(), Some((12, &PitchSystemId::from("12tet"))));
//! ```

mod errors;
mod implementation;

pub use errors::{IntervalBetweenError, IntervalError};
pub use implementation::Interval;
