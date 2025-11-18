use std::fmt;

use clap::ValueEnum;
use music_engine::prelude::{PitchSystemId, Scale, ScaleBuildError, TuningRegistry, scale};

#[derive(Clone, Copy, Debug, ValueEnum)]
pub enum ScaleKind {
    Ionian,
    Dorian,
    Phrygian,
    Lydian,
    Mixolydian,
    Aeolian,
    Locrian,
    Major,
    Minor,
    NaturalMinor,
    HarmonicMinor,
    MelodicMinor,
}

const DIATONIC_MODE_NAMES: [&str; 7] = [
    "Ionian",
    "Dorian",
    "Phrygian",
    "Lydian",
    "Mixolydian",
    "Aeolian",
    "Locrian",
];

const HARMONIC_MINOR_MODE_NAMES: [&str; 7] = [
    "Harmonic Minor",
    "Locrian ♮6",
    "Ionian ♯5",
    "Dorian ♯4",
    "Phrygian Dominant",
    "Lydian ♯2",
    "Super Locrian",
];

const MELODIC_MINOR_MODE_NAMES: [&str; 7] = [
    "Melodic Minor",
    "Dorian ♭2",
    "Lydian Augmented",
    "Lydian Dominant",
    "Mixolydian ♭6",
    "Locrian ♮2",
    "Altered",
];

impl ScaleKind {
    pub fn build_scale(
        &self,
        root_index: i32,
        system: &PitchSystemId,
        registry: &TuningRegistry,
    ) -> Result<Scale, ScaleBuildError> {
        match self {
            ScaleKind::Ionian | ScaleKind::Major => {
                scale::ionian_scale(root_index, system, registry)
            }
            ScaleKind::Dorian => scale::dorian_scale(root_index, system, registry),
            ScaleKind::Phrygian => scale::phrygian_scale(root_index, system, registry),
            ScaleKind::Lydian => scale::lydian_scale(root_index, system, registry),
            ScaleKind::Mixolydian => scale::mixolydian_scale(root_index, system, registry),
            ScaleKind::Aeolian | ScaleKind::Minor | ScaleKind::NaturalMinor => {
                scale::aeolian_scale(root_index, system, registry)
            }
            ScaleKind::Locrian => scale::locrian_scale(root_index, system, registry),
            ScaleKind::HarmonicMinor => scale::harmonic_minor_scale(root_index, system, registry),
            ScaleKind::MelodicMinor => scale::melodic_minor_scale(root_index, system, registry),
        }
    }

    pub fn mode_name_for_rotation(&self, rotation: usize) -> Option<&'static str> {
        if let Some(offset) = self.diatonic_rotation_offset() {
            return Some(DIATONIC_MODE_NAMES[(rotation + offset) % DIATONIC_MODE_NAMES.len()]);
        }

        match self {
            ScaleKind::HarmonicMinor => {
                Some(HARMONIC_MINOR_MODE_NAMES[rotation % HARMONIC_MINOR_MODE_NAMES.len()])
            }
            ScaleKind::MelodicMinor => {
                Some(MELODIC_MINOR_MODE_NAMES[rotation % MELODIC_MINOR_MODE_NAMES.len()])
            }
            _ => None,
        }
    }

    fn diatonic_rotation_offset(&self) -> Option<usize> {
        match self {
            ScaleKind::Ionian | ScaleKind::Major => Some(0),
            ScaleKind::Dorian => Some(1),
            ScaleKind::Phrygian => Some(2),
            ScaleKind::Lydian => Some(3),
            ScaleKind::Mixolydian => Some(4),
            ScaleKind::Aeolian | ScaleKind::Minor | ScaleKind::NaturalMinor => Some(5),
            ScaleKind::Locrian => Some(6),
            _ => None,
        }
    }
}

impl fmt::Display for ScaleKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl ScaleKind {
    #[must_use]
    pub fn pitch_classes(&self) -> &'static [u8; 7] {
        match self {
            ScaleKind::Ionian | ScaleKind::Major => &[0, 2, 4, 5, 7, 9, 11],
            ScaleKind::Dorian => &[0, 2, 3, 5, 7, 9, 10],
            ScaleKind::Phrygian => &[0, 1, 3, 5, 7, 8, 10],
            ScaleKind::Lydian => &[0, 2, 4, 6, 7, 9, 11],
            ScaleKind::Mixolydian => &[0, 2, 4, 5, 7, 9, 10],
            ScaleKind::Aeolian | ScaleKind::Minor | ScaleKind::NaturalMinor => {
                &[0, 2, 3, 5, 7, 8, 10]
            }
            ScaleKind::Locrian => &[0, 1, 3, 5, 6, 8, 10],
            ScaleKind::HarmonicMinor => &[0, 2, 3, 5, 7, 8, 11],
            ScaleKind::MelodicMinor => &[0, 2, 3, 5, 7, 9, 11],
        }
    }
}

#[derive(Clone, Copy, Debug, ValueEnum)]
pub enum ChordVoicing {
    Triads,
    Sevenths,
}

impl fmt::Display for ChordVoicing {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
