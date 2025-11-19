//! Chord symbol parsing utilities.
//!
//! Provides functions to parse chord symbols (e.g., "Cmaj7", "Fm", "Bdim") into structured
//! components (root note letter, accidental, quality).

use core::fmt;

use super::quality::ChordQuality;

/// Note letter (C through B).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NoteLetter {
    C,
    D,
    E,
    F,
    G,
    A,
    B,
}

impl NoteLetter {
    /// Semitone offset from C (C=0, D=2, E=4, F=5, G=7, A=9, B=11).
    #[must_use]
    pub const fn semitone_from_c(&self) -> i32 {
        match self {
            Self::C => 0,
            Self::D => 2,
            Self::E => 4,
            Self::F => 5,
            Self::G => 7,
            Self::A => 9,
            Self::B => 11,
        }
    }
}

impl fmt::Display for NoteLetter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::C => f.write_str("C"),
            Self::D => f.write_str("D"),
            Self::E => f.write_str("E"),
            Self::F => f.write_str("F"),
            Self::G => f.write_str("G"),
            Self::A => f.write_str("A"),
            Self::B => f.write_str("B"),
        }
    }
}

/// Accidental modifier (sharp, flat, natural).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Accidental {
    Flat,
    Natural,
    Sharp,
}

impl Accidental {
    /// Semitone offset applied by the accidental (-1 for flat, 0 for natural, +1 for sharp).
    #[must_use]
    pub const fn semitone_offset(&self) -> i32 {
        match self {
            Self::Flat => -1,
            Self::Natural => 0,
            Self::Sharp => 1,
        }
    }
}

impl fmt::Display for Accidental {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Flat => f.write_str("♭"),
            Self::Natural => Ok(()),
            Self::Sharp => f.write_str("♯"),
        }
    }
}

/// Parsed chord symbol components.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChordSymbol {
    pub root: NoteLetter,
    pub accidental: Accidental,
    pub quality: ChordQuality,
}

impl ChordSymbol {
    /// Absolute semitone position of the root (C=0, C#=1, D=2, ..., B=11, accounting for octave wrapping).
    #[must_use]
    pub const fn root_semitone(&self) -> i32 {
        (self.root.semitone_from_c() + self.accidental.semitone_offset()).rem_euclid(12)
    }
}

impl fmt::Display for ChordSymbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.root, self.accidental)?;
        match self.quality {
            ChordQuality::MajorTriad => Ok(()),
            ChordQuality::MinorTriad => f.write_str("m"),
            ChordQuality::DiminishedTriad => f.write_str("dim"),
            ChordQuality::AugmentedTriad => f.write_str("aug"),
            ChordQuality::SuspendedSecond => f.write_str("sus2"),
            ChordQuality::SuspendedFourth => f.write_str("sus4"),
            ChordQuality::DominantSeventh => f.write_str("7"),
            ChordQuality::MajorSeventh => f.write_str("maj7"),
            ChordQuality::MinorSeventh => f.write_str("m7"),
            ChordQuality::MinorMajorSeventh => f.write_str("m(maj7)"),
            ChordQuality::HalfDiminishedSeventh => f.write_str("m7♭5"),
            ChordQuality::DiminishedSeventh => f.write_str("dim7"),
            ChordQuality::Add9 => f.write_str("add9"),
            ChordQuality::Dominant9 => f.write_str("9"),
            ChordQuality::Major9 => f.write_str("maj9"),
            ChordQuality::Minor9 => f.write_str("m9"),
            ChordQuality::Dominant11 => f.write_str("11"),
            ChordQuality::Major11 => f.write_str("maj11"),
            ChordQuality::Minor11 => f.write_str("m11"),
            ChordQuality::Dominant13 => f.write_str("13"),
            ChordQuality::Major13 => f.write_str("maj13"),
            ChordQuality::Minor13 => f.write_str("m13"),
        }
    }
}

/// Error parsing a chord symbol.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseChordError {
    /// Input string is empty.
    EmptyInput,
    /// Root note letter not recognized.
    InvalidRoot(char),
    /// Chord quality suffix not recognized.
    UnknownQuality(alloc::string::String),
}

impl fmt::Display for ParseChordError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyInput => f.write_str("chord symbol cannot be empty"),
            Self::InvalidRoot(c) => write!(f, "invalid root note: '{c}'"),
            Self::UnknownQuality(s) => write!(f, "unknown chord quality: '{s}'"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for ParseChordError {}

/// Parse a chord symbol string (e.g., "Cmaj7", "F#m", "Bb7") into its components.
///
/// # Errors
///
/// Returns [`ParseChordError`] if the input is empty, the root note is invalid, or the quality
/// suffix is unrecognized.
pub fn parse_chord_symbol(input: &str) -> Result<ChordSymbol, ParseChordError> {
    use alloc::string::ToString;

    if input.is_empty() {
        return Err(ParseChordError::EmptyInput);
    }

    let mut chars = input.chars();
    let root_char = chars.next().ok_or(ParseChordError::EmptyInput)?;
    let root = match root_char.to_ascii_uppercase() {
        'C' => NoteLetter::C,
        'D' => NoteLetter::D,
        'E' => NoteLetter::E,
        'F' => NoteLetter::F,
        'G' => NoteLetter::G,
        'A' => NoteLetter::A,
        'B' => NoteLetter::B,
        c => return Err(ParseChordError::InvalidRoot(c)),
    };

    let rest: alloc::string::String = chars.collect();
    let (accidental, quality_str) = if rest.starts_with('#') || rest.starts_with('♯') {
        (
            Accidental::Sharp,
            &rest[rest.chars().next().map_or(0, char::len_utf8)..],
        )
    } else if rest.starts_with('b') || rest.starts_with('♭') {
        (
            Accidental::Flat,
            &rest[rest.chars().next().map_or(0, char::len_utf8)..],
        )
    } else {
        (Accidental::Natural, rest.as_str())
    };

    let quality = match quality_str {
        "" | "maj" => ChordQuality::MajorTriad,
        "m" | "min" => ChordQuality::MinorTriad,
        "dim" | "°" => ChordQuality::DiminishedTriad,
        "aug" | "+" => ChordQuality::AugmentedTriad,
        "sus2" => ChordQuality::SuspendedSecond,
        "sus4" | "sus" => ChordQuality::SuspendedFourth,
        "7" => ChordQuality::DominantSeventh,
        "maj7" | "M7" | "Δ7" => ChordQuality::MajorSeventh,
        "m7" | "min7" => ChordQuality::MinorSeventh,
        "m(maj7)" | "m/maj7" | "mM7" => ChordQuality::MinorMajorSeventh,
        "m7b5" | "m7♭5" | "ø7" => ChordQuality::HalfDiminishedSeventh,
        "dim7" | "°7" => ChordQuality::DiminishedSeventh,
        "add9" => ChordQuality::Add9,
        "9" => ChordQuality::Dominant9,
        "maj9" | "M9" | "Δ9" => ChordQuality::Major9,
        "m9" | "min9" => ChordQuality::Minor9,
        "11" => ChordQuality::Dominant11,
        "maj11" | "M11" => ChordQuality::Major11,
        "m11" | "min11" => ChordQuality::Minor11,
        "13" => ChordQuality::Dominant13,
        "maj13" | "M13" => ChordQuality::Major13,
        "m13" | "min13" => ChordQuality::Minor13,
        _ => return Err(ParseChordError::UnknownQuality(quality_str.to_string())),
    };

    Ok(ChordSymbol {
        root,
        accidental,
        quality,
    })
}
