use std::path::Path;

use anyhow::{Context, Result, anyhow, bail};
use music_score::planner::{SectionTemplate, builtin_template, load_template_from_path};
use music_theory::{Key12, PitchClass12, key::Mode};

#[derive(Clone, Debug, Default)]
pub struct TemplateLocator {
    pub builtin_id: Option<String>,
    pub file_path: Option<String>,
}

impl TemplateLocator {
    #[must_use]
    pub fn builtin(id: impl Into<String>) -> Self {
        Self { builtin_id: Some(id.into()), file_path: None }
    }

    #[must_use]
    pub fn file(path: impl Into<String>) -> Self {
        Self { builtin_id: None, file_path: Some(path.into()) }
    }

    #[must_use]
    pub fn describe(&self) -> String {
        match (&self.builtin_id, &self.file_path) {
            (Some(id), None) => format!("builtin:{id}"),
            (None, Some(path)) => format!("file:{path}"),
            (Some(id), Some(path)) => format!("builtin:{id} + file:{path}"),
            (None, None) => "unknown".to_string(),
        }
    }
}

pub fn resolve_template(
    builtin_id: Option<&str>,
    template_path: Option<&Path>,
) -> Result<(SectionTemplate, TemplateLocator)> {
    match (builtin_id, template_path) {
        (Some(_), Some(_)) => {
            bail!("specify either --template or --template-path (not both)");
        }
        (Some(id), None) => {
            let template = builtin_template(id)
                .with_context(|| format!("unknown built-in template '{id}'"))?;
            Ok((template, TemplateLocator::builtin(id)))
        }
        (None, Some(path)) => {
            let template = load_template_from_path(path)
                .with_context(|| format!("failed to load template from {}", path.display()))?;
            let locator = TemplateLocator::file(path.display().to_string());
            Ok((template, locator))
        }
        (None, None) => {
            bail!("template not specified (use --template or --template-path)");
        }
    }
}

pub fn parse_key(tonic: &str, mode: Mode) -> Result<Key12> {
    let pitch_class = parse_pitch_class(tonic)
        .with_context(|| format!("invalid tonic '{tonic}' (expected pitch like C, F#, Bb)"))?;
    Ok(Key12::new(pitch_class, mode))
}

fn parse_pitch_class(input: &str) -> Result<PitchClass12> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        bail!("tonic cannot be empty");
    }
    let mut chars = trimmed.chars();
    let letter = chars
        .next()
        .ok_or_else(|| anyhow!("tonic must start with A–G letter"))?
        .to_ascii_uppercase();
    let base = match letter {
        'C' => 0,
        'D' => 2,
        'E' => 4,
        'F' => 5,
        'G' => 7,
        'A' => 9,
        'B' => 11,
        _ => bail!("tonic must begin with A, B, C, D, E, F, or G"),
    };
    let mut offset: i32 = 0;
    for ch in chars {
        offset += match ch {
            '#' | '♯' => 1,
            'b' | '♭' => -1,
            _ => bail!("unrecognized accidental '{ch}' (use # or b)"),
        };
    }
    let semitone = (base + offset).rem_euclid(12) as u16;
    Ok(PitchClass12::from_semitones(semitone))
}

const PITCH_CLASS_LABELS: [&str; 12] =
    ["C", "C#", "D", "Eb", "E", "F", "F#", "G", "Ab", "A", "Bb", "B"];

#[must_use]
pub fn pitch_class_label(pc: PitchClass12) -> &'static str {
    let index = pc.to_semitones() as usize % 12;
    PITCH_CLASS_LABELS[index]
}

#[must_use]
pub fn format_key_label(key: Key12) -> String {
    let tonic = pitch_class_label(key.tonic);
    let mode_label = match key.mode {
        Mode::Major => "major",
        Mode::Minor => "minor",
    };
    format!("{tonic} {mode_label}")
}
