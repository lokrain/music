#![forbid(unsafe_code)]

pub mod ngram;

pub use ngram::{ChordTransitionModel, MelodyTransitionModel, TransitionModel};
