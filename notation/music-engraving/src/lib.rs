//! notation/music-engraving/src/lib.rs
//! Engraving primitives: positions, glyphs, spacing hints.

pub mod geometry;
pub mod glyph;

pub use geometry::{LayoutBox, LayoutPosition, PageIndex, SystemIndex};
pub use glyph::{GlyphId, StemDirection};
