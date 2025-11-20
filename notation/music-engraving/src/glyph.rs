//! notation/music-engraving/src/glyph.rs
//! Glyph identifiers and stem directions used by engraving backends.

use core::fmt;

/// Identifier of a glyph (e.g. SMuFL code point).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GlyphId(pub u32);

impl GlyphId {
    /// Construct a glyph id from a Unicode/SMuFL code point.
    #[must_use]
    pub const fn from_codepoint(code: u32) -> Self {
        Self(code)
    }

    /// Format as an SMuFL-style string (e.g. `U+E0A4`).
    #[must_use]
    pub fn to_smufl(&self) -> String {
        format!("U+{:04X}", self.0)
    }
}

impl From<char> for GlyphId {
    fn from(value: char) -> Self {
        Self(value as u32)
    }
}

impl fmt::Display for GlyphId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.to_smufl())
    }
}

/// Stem direction enumeration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StemDirection {
    Up,
    Down,
}

impl StemDirection {
    /// Signed scalar (Up = `1.0`, Down = `-1.0`) useful for vertical offsets.
    #[must_use]
    pub const fn sign(self) -> f32 {
        match self {
            StemDirection::Up => 1.0,
            StemDirection::Down => -1.0,
        }
    }

    /// Flip the stem direction.
    #[must_use]
    pub const fn flipped(self) -> Self {
        match self {
            StemDirection::Up => StemDirection::Down,
            StemDirection::Down => StemDirection::Up,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn glyph_id_display_matches_smufl_format() {
        let glyph = GlyphId::from_codepoint(0xE0A4);
        assert_eq!(glyph.to_string(), "U+E0A4");
    }

    #[test]
    fn stem_direction_sign_and_flip() {
        assert_eq!(StemDirection::Up.sign(), 1.0);
        assert_eq!(StemDirection::Down.sign(), -1.0);
        assert_eq!(StemDirection::Up.flipped(), StemDirection::Down);
    }
}
