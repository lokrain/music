//! core/music-time/src/lib.rs
//! Time grid primitives: beat, meter, tempo, time points and spans.

pub mod beat;
pub mod meter;
pub mod tempo;
pub mod timegrid;
pub mod timespan;

pub use beat::Beat;
pub use meter::Meter;
pub use tempo::Tempo;
pub use timegrid::{GridConfig, TimeGrid};
pub use timespan::{TimePoint, TimeSpan};
