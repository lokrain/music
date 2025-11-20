//! core/music-theory/src/chord.rs
//! Chord definitions and canonical chord-type helpers.
//!
//! Chords combine [`PitchClass`] degrees from a [`Scale`](crate::scale::Scale) using stacks of
//! thirds (or extended tensions). Keys leverage chords to describe harmonic functions and to form
//! cadential progressions.

use crate::interval::Interval;
use crate::pitch::{PitchClass, PitchClass12};
use music_acoustic::{T12, Temperament};

/// Triad quality (root + third + fifth).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TriadKind {
    Major,
    Minor,
    Diminished,
    Augmented,
}

impl TriadKind {
    #[must_use]
    pub const fn intervals(self) -> &'static [i32] {
        match self {
            Self::Major => &[0, 4, 7],
            Self::Minor => &[0, 3, 7],
            Self::Diminished => &[0, 3, 6],
            Self::Augmented => &[0, 4, 8],
        }
    }
}

/// Seventh-chord quality (root + third + fifth + seventh).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SeventhKind {
    Major7,
    Dominant7,
    Minor7,
    HalfDiminished7,
    Diminished7,
}

impl SeventhKind {
    #[must_use]
    pub const fn intervals(self) -> &'static [i32] {
        match self {
            Self::Major7 => &[0, 4, 7, 11],
            Self::Dominant7 => &[0, 4, 7, 10],
            Self::Minor7 => &[0, 3, 7, 10],
            Self::HalfDiminished7 => &[0, 3, 6, 10],
            Self::Diminished7 => &[0, 3, 6, 9],
        }
    }
}

/// Common extended chords (add and 9/11/13 structures).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ExtendedKind {
    Add9,
    Add11,
    Add13,
    SixNine,
    Major9,
    Dominant9,
    Minor9,
    Major11,
    Dominant11,
    Minor11,
    Major13,
    Dominant13,
    Minor13,
}

impl ExtendedKind {
    #[must_use]
    pub const fn intervals(self) -> &'static [i32] {
        match self {
            Self::Add9 => &[0, 4, 7, 14],
            Self::Add11 => &[0, 4, 7, 17],
            Self::Add13 => &[0, 4, 7, 21],
            Self::SixNine => &[0, 4, 7, 9, 14],
            Self::Major9 => &[0, 4, 7, 11, 14],
            Self::Dominant9 => &[0, 4, 7, 10, 14],
            Self::Minor9 => &[0, 3, 7, 10, 14],
            Self::Major11 => &[0, 4, 7, 11, 14, 17],
            Self::Dominant11 => &[0, 4, 7, 10, 14, 17],
            Self::Minor11 => &[0, 3, 7, 10, 14, 17],
            Self::Major13 => &[0, 4, 7, 11, 14, 17, 21],
            Self::Dominant13 => &[0, 4, 7, 10, 14, 17, 21],
            Self::Minor13 => &[0, 3, 7, 10, 14, 17, 21],
        }
    }
}

/// Unified chord-kind wrapper.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChordKind {
    Triad(TriadKind),
    Seventh(SeventhKind),
    Extended(ExtendedKind),
}

impl ChordKind {
    #[must_use]
    pub const fn intervals(self) -> &'static [i32] {
        match self {
            Self::Triad(kind) => kind.intervals(),
            Self::Seventh(kind) => kind.intervals(),
            Self::Extended(kind) => kind.intervals(),
        }
    }
}

/// Chord as an ordered collection of pitch-classes in a temperament.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Chord<T: Temperament> {
    pub tones: Vec<PitchClass<T>>,
}

impl<T: Temperament> Chord<T> {
    #[must_use]
    pub fn new(tones: Vec<PitchClass<T>>) -> Self {
        Self { tones }
    }

    /// Build a chord from root + explicit interval offsets (in temperament steps).
    #[must_use]
    pub fn from_intervals(root: PitchClass<T>, intervals: &[i32]) -> Self {
        let tones = intervals.iter().map(|steps| root.transpose(Interval::new(*steps))).collect();
        Self::new(tones)
    }

    /// Build a chord from a canonical [`ChordKind`].
    #[must_use]
    pub fn from_kind(root: PitchClass<T>, kind: ChordKind) -> Self {
        Self::from_intervals(root, kind.intervals())
    }

    /// Number of unique tones in the chord.
    #[must_use]
    pub fn len(&self) -> usize {
        self.tones.len()
    }

    /// Whether the chord contains no tones.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.tones.is_empty()
    }

    /// Check whether a pitch-class is part of the chord.
    #[must_use]
    pub fn contains(&self, pitch_class: PitchClass<T>) -> bool {
        self.tones.contains(&pitch_class)
    }
}

/// Convenience alias for 12-TET chords.
pub type Chord12 = Chord<T12>;

impl Chord12 {
    #[must_use]
    pub fn major_triad(root: PitchClass12) -> Self {
        Self::from_kind(root, ChordKind::Triad(TriadKind::Major))
    }

    #[must_use]
    pub fn minor_triad(root: PitchClass12) -> Self {
        Self::from_kind(root, ChordKind::Triad(TriadKind::Minor))
    }

    #[must_use]
    pub fn diminished_triad(root: PitchClass12) -> Self {
        Self::from_kind(root, ChordKind::Triad(TriadKind::Diminished))
    }

    #[must_use]
    pub fn augmented_triad(root: PitchClass12) -> Self {
        Self::from_kind(root, ChordKind::Triad(TriadKind::Augmented))
    }

    #[must_use]
    pub fn major_seventh(root: PitchClass12) -> Self {
        Self::from_kind(root, ChordKind::Seventh(SeventhKind::Major7))
    }

    #[must_use]
    pub fn dominant_seventh(root: PitchClass12) -> Self {
        Self::from_kind(root, ChordKind::Seventh(SeventhKind::Dominant7))
    }

    #[must_use]
    pub fn minor_seventh(root: PitchClass12) -> Self {
        Self::from_kind(root, ChordKind::Seventh(SeventhKind::Minor7))
    }

    #[must_use]
    pub fn half_diminished(root: PitchClass12) -> Self {
        Self::from_kind(root, ChordKind::Seventh(SeventhKind::HalfDiminished7))
    }

    #[must_use]
    pub fn diminished_seventh(root: PitchClass12) -> Self {
        Self::from_kind(root, ChordKind::Seventh(SeventhKind::Diminished7))
    }

    #[must_use]
    pub fn extended(root: PitchClass12, kind: ExtendedKind) -> Self {
        Self::from_kind(root, ChordKind::Extended(kind))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn semitone(pc: u16) -> PitchClass12 {
        PitchClass12::from_semitones(pc)
    }

    #[test]
    fn c_major_triad() {
        let chord = Chord12::major_triad(semitone(0));
        let expected = vec![semitone(0), semitone(4), semitone(7)];
        assert_eq!(chord.tones, expected);
        assert_eq!(chord.len(), 3);
    }

    #[test]
    fn g_dominant_seventh_matches_expectation() {
        let chord = Chord12::dominant_seventh(semitone(7));
        let expected = vec![semitone(7), semitone(11), semitone(2), semitone(5)];
        assert_eq!(chord.tones, expected);
    }

    #[test]
    fn a_minor_seventh_includes_correct_pitches() {
        let chord = Chord12::minor_seventh(semitone(9));
        let pcs = [9, 0, 4, 7];
        for semis in pcs {
            assert!(chord.contains(semitone(semis)));
        }
        assert_eq!(chord.len(), 4);
    }

    #[test]
    fn half_diminished_on_b_matches_locrian_expectation() {
        let chord = Chord12::half_diminished(semitone(11));
        let expected = vec![semitone(11), semitone(2), semitone(5), semitone(9)];
        assert_eq!(chord.tones, expected);
    }

    #[test]
    fn extended_chord_builds_from_kind() {
        let chord = Chord12::extended(semitone(0), ExtendedKind::Major9);
        let expected = vec![semitone(0), semitone(4), semitone(7), semitone(11), semitone(2)];
        assert_eq!(chord.tones, expected);
    }
}
