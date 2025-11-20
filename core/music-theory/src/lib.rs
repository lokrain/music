//! Pure music theory primitives: temperament-aware pitch classes/pitches, interval math,
//! diatonic scales, chord constructors, tonal keys, and harmonic functions.
//!
//! The crate is layered so higher-level concepts build on top of lower-level acoustics:
//! - [`PitchClass`] and [`Pitch`] wrap `music_acoustic::Temperament` markers so any tuning
//!   (12-TET, 24-TET, etc.) can share the same APIs.
//! - [`Interval`] describes abstract step distances that drive transposition and scale construction.
//! - [`Scale`] derives ordered pitch collections from step patterns and feeds degree lookups for
//!   chords and keys.
//! - [`Chord`] composes tertian/extended sonorities from root intervals.
//! - [`Key`] and [`HarmonicFunction`] tie everything together by mapping scale degrees to tonal
//!   functions (tonic, subdominant, dominant, ...).
//!
//! # Example
//! Build a I–V–I cadence in C major and classify the dominant function:
//!
//! ```
//! use music_theory::{
//!     Chord12, FunctionKind, Key12, PitchClass12,
//! };
//! let c = PitchClass12::from_semitones(0);
//! let g = PitchClass12::from_semitones(7);
//! let key = Key12::major(c);
//! let scale = key.scale12();
//! assert_eq!(scale.degree_of(c), Some(1));
//! assert_eq!(scale.degree_of(g), Some(5));
//!
//! let tonic = Chord12::major_triad(c);
//! let dominant = Chord12::dominant_seventh(g);
//! let cadence = vec![tonic.clone(), dominant.clone(), tonic.clone()];
//! assert_eq!(cadence.len(), 3);
//! assert!(dominant.contains(g));
//!
//! let dominant_function = key.function_for_pitch_class(g).unwrap();
//! assert_eq!(dominant_function.kind, FunctionKind::Dominant);
//! assert_eq!(dominant_function.degree, 5);
//! ```

pub mod chord;
pub mod function;
pub mod interval;
pub mod key;
pub mod pitch;
pub mod scale;

pub use chord::{Chord, Chord12, ChordKind, ExtendedKind, SeventhKind, TriadKind};
pub use function::{FunctionKind, HarmonicFunction};
pub use interval::{GenericInterval, Interval};
pub use key::{Key, Key12, Mode};
pub use pitch::{Pitch, Pitch12, PitchClass, PitchClass12};
pub use scale::{Scale, Scale12};
