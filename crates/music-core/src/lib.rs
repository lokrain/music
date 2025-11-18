//! Core primitives for expressing musical pitches and tuning systems.
//!
//! ```rust
//! use music_core::{
//!     pitch::Pitch,
//!     systems::TwelveTET,
//!     PitchSystemId,
//!     TuningRegistry,
//! };
//!
//! let registry = TuningRegistry::new()
//!     .with_system(PitchSystemId::from("12tet"), TwelveTET::a4_440());
//!
//! let a4 = Pitch::abstract_pitch(69, PitchSystemId::from("12tet"));
//! assert_eq!(a4.freq_hz(&registry), Some(440.0));
//! ```
#![forbid(unsafe_code)]
#![deny(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::unwrap_used,
    clippy::expect_used
)]
#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

pub mod chord;
pub mod interval;
pub mod pitch;
pub mod prelude;
pub mod registry;
pub mod scale;
pub mod system;
pub mod systems;

pub use chord::*;
pub use interval::*;
pub use pitch::*;
pub use registry::*;
pub use scale::*;
pub use system::*;
pub use systems::*;
