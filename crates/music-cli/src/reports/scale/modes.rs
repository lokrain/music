//! Mode listing and enumeration report types with utilities.

use std::fmt::Write as FmtWrite;

use music_engine::prelude::{Pitch, Scale, TuningRegistry};
use serde::Serialize;

#[cfg(feature = "schema")]
use schemars::JsonSchema;

use crate::reports::pitch::PitchResult;

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct ModeListing {
    pub system: String,
    pub root_index: i32,
    pub scale: String,
    pub mode_count: usize,
    pub modes: Vec<ModeSummary>,
}

impl ModeListing {
    pub fn render_text(&self) -> String {
        let mut out = String::new();
        let _ = writeln!(
            &mut out,
            "{count} mode(s) derived from the {scale} scale (root {root}) in {system}.",
            count = self.mode_count,
            scale = self.scale,
            root = self.root_index,
            system = self.system
        );
        let _ = writeln!(&mut out);

        for mode in &self.modes {
            let title = mode
                .mode_name
                .as_deref()
                .map(str::to_string)
                .unwrap_or_else(|| format!("Mode {}", mode.mode_index));
            let root_label = mode.root_label.as_deref().unwrap_or("—");
            let _ = writeln!(
                &mut out,
                "{idx:>2}. {title} (rotation {rotation}, root {root_label})",
                idx = mode.mode_index,
                title = title,
                rotation = mode.rotation_degree + 1,
                root_label = root_label
            );
            for pitch in &mode.pitches {
                let label = pitch.label.as_deref().unwrap_or("—");
                let _ = writeln!(
                    &mut out,
                    "    • degree {degree}: {label} ({freq:.3} Hz)",
                    degree = pitch.degree + 1,
                    label = label,
                    freq = pitch.frequency_hz
                );
            }
            let _ = writeln!(&mut out);
        }

        out
    }
}

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct ModeSummary {
    pub mode_index: usize,
    pub rotation_degree: usize,
    pub mode_name: Option<String>,
    pub root_index: Option<i32>,
    pub root_label: Option<String>,
    pub pitches: Vec<ModePitch>,
}

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct ModePitch {
    pub degree: usize,
    pub index: Option<i32>,
    pub label: Option<String>,
    pub frequency_hz: f32,
}

/// Collect mode pitch summaries from a scale.
pub fn collect_mode_pitches(
    scale: &Scale,
    registry: &TuningRegistry,
    step_count: usize,
) -> PitchResult<Vec<ModePitch>> {
    if step_count == 0 {
        return Ok(Vec::new());
    }

    let limit = step_count - 1;
    let mut summaries = Vec::with_capacity(step_count);
    for (degree, pitch) in scale
        .degree_pitches(limit, registry)?
        .into_iter()
        .enumerate()
    {
        summaries.push(summarize_mode_pitch(degree, pitch, registry)?);
    }
    Ok(summaries)
}

fn summarize_mode_pitch(
    degree: usize,
    pitch: Pitch,
    registry: &TuningRegistry,
) -> PitchResult<ModePitch> {
    let label = pitch
        .try_label(registry)
        .map(|value| value.to_string_lossy())
        .ok();
    let frequency_hz = pitch.try_freq_hz(registry)?;
    Ok(ModePitch {
        degree,
        index: pitch.index(),
        label,
        frequency_hz,
    })
}
