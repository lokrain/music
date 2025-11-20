//! notation/music-articulation/src/articulation.rs
//! Articulation symbols and ornaments shared across the workspace.
//!
//! The enums below intentionally derive [`PartialOrd`] / [`Ord`] and use `#[repr(u8)]`
//! so their discriminant ordering is stable for persistence/serialization.

/// Basic articulation kinds applied to noteheads/stems.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum ArticulationKind {
    /// Extreme shortening; typically notated with a wedge.
    Staccatissimo = 0,
    /// Detached but connected; dot articulation.
    Staccato = 1,
    /// Full duration; line articulation.
    Tenuto = 2,
    /// Standard accent (>).
    Accent = 3,
    /// Heavy accent (^).
    Marcato = 4,
    /// Sustain beyond written value.
    Fermata = 5,
}

impl ArticulationKind {
    /// Articulations in their canonical order; must remain stable.
    pub const ORDERED: [Self; 6] = [
        Self::Staccatissimo,
        Self::Staccato,
        Self::Tenuto,
        Self::Accent,
        Self::Marcato,
        Self::Fermata,
    ];
}

/// Melodic ornaments that embellish a pitch.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum OrnamentKind {
    /// Rapid alternation with the upper neighbor.
    Trill = 0,
    /// Single alternation with upper then main pitch.
    Turn = 1,
    /// Rapid upper-neighbor execution preceding the beat.
    UpperMordent = 2,
    /// Rapid lower-neighbor execution preceding the beat.
    LowerMordent = 3,
    /// Grace/appoggiatura style lead-in notes.
    Appoggiatura = 4,
    /// Short crushed grace note (acciaccatura).
    Acciaccatura = 5,
}

impl OrnamentKind {
    /// Ornaments in canonical order.
    pub const ORDERED: [Self; 6] = [
        Self::Trill,
        Self::Turn,
        Self::UpperMordent,
        Self::LowerMordent,
        Self::Appoggiatura,
        Self::Acciaccatura,
    ];
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn articulation_order_stable() {
        for (expected, variant) in ArticulationKind::ORDERED.iter().enumerate() {
            assert_eq!(*variant as u8, expected as u8);
        }
    }

    #[test]
    fn ornament_order_stable() {
        for (expected, variant) in OrnamentKind::ORDERED.iter().enumerate() {
            assert_eq!(*variant as u8, expected as u8);
        }
    }
}
