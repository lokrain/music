//! Ergonomic registry utilities for named pitch systems.
//!
//! The [`TuningRegistry`] type tracks [`PitchSystem`](crate::system::PitchSystem) instances by
//! their [`PitchSystemId`](crate::system::PitchSystemId) and offers fallible registration,
//! borrowed lookups, and iterator helpers for in-place tweaks.
//!
//! # Examples
//!
//! ```rust
//! use music_core::{
//!     registry::TuningRegistry,
//!     systems::TwelveTET,
//!     PitchSystemId,
//! };
//!
//! # fn main() {
//! let mut registry = TuningRegistry::new();
//! let id = PitchSystemId::try_new("12tet").expect("valid id");
//! registry
//!     .try_register_system(id.clone(), TwelveTET::a4_440())
//!     .expect("unique id");
//!
//! // Borrowed lookup helpers accept `&str`.
//! let hz = registry
//!     .resolve_frequency_str("12tet", 69)
//!     .expect("registered system");
//! assert!((hz - 440.0).abs() < 1e-6);
//!
//! // Lazily insert or fetch systems.
//! let _shared = registry.get_or_insert_with("12tet", TwelveTET::a4_440);
//! assert_eq!(registry.ids().count(), 1);
//!
//! // Iterator helpers expose deterministic traversal.
//! assert_eq!(registry.iter().count(), 1);
//! assert_eq!(registry.contains_str("12tet"), true);
//! assert_eq!(registry.clone().into_iter().count(), 1);
//! # }
//! ```
//!
//! When user input is involved, prefer [`PitchSystemId::try_new`](crate::system::PitchSystemId::try_new)
//! or [`PitchSystemId::from_str`](core::str::FromStr::from_str) to validate identifiers before
//! inserting them into the registry.

mod errors;
mod tuning;

pub use errors::*;
pub use tuning::*;
