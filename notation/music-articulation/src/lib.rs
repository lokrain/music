//! notation/music-articulation/src/lib.rs
//! Articulation and dynamics primitives.

pub mod articulation;
pub mod dynamics;

pub use articulation::{ArticulationKind, OrnamentKind};
pub use dynamics::{DynamicMark, Hairpin};
