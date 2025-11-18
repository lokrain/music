#![forbid(unsafe_code)]

pub mod estimate;
pub mod ngram;
pub mod profile;

pub use estimate::{MusicEstimate, estimate_from_melody, estimate_from_midi};
pub use ngram::{ChordTransitionModel, MelodyTransitionModel, TransitionModel};
pub use profile::{PitchRangeStats, ProfileStats, TimingStats, profile_melody, profile_midi};
