#![allow(dead_code)]
//! Scale-related report types.

use std::fmt::Write as FmtWrite;

use music_engine::prelude::{Pitch, Scale, TuningRegistry};
use serde::Serialize;

#[cfg(feature = "schema")]
use schemars::JsonSchema;

use crate::reports::pitch::PitchResult;

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct ScaleExplanation {
    pub system: String,
    pub root_index: i32,
    pub root_label: String,
    pub scale_name: String,
    pub mode_alias: Option<String>,
    pub degrees: Vec<ScaleDegreeSummary>,
    pub pattern_cents: Vec<f32>,
    pub narrative: Vec<String>,
}

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct ScaleDegreeSummary {
    pub degree: usize,
    pub label: String,
    pub frequency_hz: f32,
    pub interval_cents: f32,
    pub semitone_offset: Option<i32>,
    pub role: String,
}

impl ScaleExplanation {
    pub fn render_text(&self) -> String {
        let mut out = String::new();
        let _ = writeln!(
            &mut out,
            "Scale explanation: {scale} rooted at {label} ({root_index}) in {system}.",
            scale = self.scale_name,
            label = self.root_label,
            root_index = self.root_index,
            system = self.system
        );
        if let Some(alias) = &self.mode_alias {
            let _ = writeln!(&mut out, "Alias/rotation: {alias}.");
        }
        if !self.pattern_cents.is_empty() {
            let descriptions: Vec<String> = self
                .pattern_cents
                .iter()
                .enumerate()
                .map(|(idx, cents)| {
                    let step = idx + 1;
                    format!("step {step}: {cents:.1} cents")
                })
                .collect();
            let _ = writeln!(
                &mut out,
                "Step pattern: {pattern}.",
                pattern = descriptions.join(", ")
            );
        }
        for degree in &self.degrees {
            let offset = degree
                .semitone_offset
                .map(|value| format!(" {value:+} st"))
                .unwrap_or_default();
            let _ = writeln!(
                &mut out,
                "  • Degree {deg} ({label}) — {role}, {freq:.3} Hz, {cents:.1} cents{offset}.",
                deg = degree.degree,
                label = degree.label,
                role = degree.role,
                freq = degree.frequency_hz,
                cents = degree.interval_cents,
                offset = offset
            );
        }
        for paragraph in &self.narrative {
            let _ = writeln!(&mut out);
            let _ = writeln!(&mut out, "{paragraph}");
        }
        out
    }
}

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct SystemsListing {
    pub reference_index: i32,
    pub systems: Vec<SystemSummary>,
}

impl SystemsListing {
    pub fn render_text(&self) -> String {
        let mut out = String::new();
        let count = self.systems.len();
        let _ = writeln!(
            &mut out,
            "{count} tuning systems registered (reference index {reference}).",
            count = count,
            reference = self.reference_index
        );
        for system in &self.systems {
            let label = system
                .label
                .as_deref()
                .map(|value| format!(" — {value}"))
                .unwrap_or_default();
            let _ = writeln!(
                &mut out,
                "  • {id}: {freq:.3} Hz at index {index}{label}",
                id = system.id,
                freq = system.frequency_hz,
                index = system.reference_index,
                label = label
            );
        }
        out
    }
}

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct SystemSummary {
    pub id: String,
    pub reference_index: i32,
    pub frequency_hz: f32,
    pub label: Option<String>,
}

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

#[derive(Debug, Serialize, Clone)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct ScalePitchClassProjection {
    pub degree: usize,
    pub pitch_class: u8,
    pub index: i32,
    pub label: Option<String>,
    pub frequency_hz: f32,
}

#[derive(Debug, Serialize, Clone)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct PitchClassMapEntry {
    pub pitch_class: u8,
    pub occupied: bool,
    pub degree: Option<usize>,
    pub label: Option<String>,
}

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct ModulatoryPathSummary {
    pub rotation: usize,
    pub mode_name: Option<String>,
    pub root_index: i32,
    pub root_label: Option<String>,
    pub pivot_pitch_classes: Vec<u8>,
}

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct ScaleMapReport {
    pub system: String,
    pub root_index: i32,
    pub root_label: Option<String>,
    pub scale: String,
    pub members: Vec<ScalePitchClassProjection>,
    pub pitch_class_map: Vec<PitchClassMapEntry>,
    pub modulatory_paths: Vec<ModulatoryPathSummary>,
}

impl ScaleMapReport {
    pub fn render_text(&self) -> String {
        let mut out = String::new();
        let root_label = self
            .root_label
            .as_deref()
            .map(|label| format!(" ({label})"))
            .unwrap_or_default();
        let _ = writeln!(
            &mut out,
            "Scale map for {scale} in {system}, root {root}{label}.",
            scale = self.scale,
            system = self.system,
            root = self.root_index,
            label = root_label
        );
        let _ = writeln!(&mut out, "Pitch-class layout:");
        for entry in &self.pitch_class_map {
            let marker = if entry.occupied { '●' } else { '·' };
            if let Some(degree) = entry.degree {
                let lbl = entry.label.as_deref().unwrap_or("—");
                let _ = writeln!(
                    &mut out,
                    "  pc {pc:>2} {marker} degree {degree} ({lbl})",
                    pc = entry.pitch_class,
                    marker = marker,
                    degree = degree,
                    lbl = lbl
                );
            } else {
                let _ = writeln!(
                    &mut out,
                    "  pc {pc:>2} {marker}",
                    pc = entry.pitch_class,
                    marker = marker
                );
            }
        }
        if !self.modulatory_paths.is_empty() {
            let _ = writeln!(&mut out);
            let _ = writeln!(&mut out, "Modulatory paths:");
            for path in &self.modulatory_paths {
                let mode = path.mode_name.as_deref().unwrap_or("(unknown mode)");
                let pivots = if path.pivot_pitch_classes.is_empty() {
                    "none".into()
                } else {
                    format!("{:?}", path.pivot_pitch_classes)
                };
                let root_label = path
                    .root_label
                    .as_deref()
                    .map(|label| format!(" ({label})"))
                    .unwrap_or_default();
                let _ = writeln!(
                    &mut out,
                    "  Rotation {rotation}: {mode}, root {root}{label}, pivots {pivots}.",
                    rotation = path.rotation,
                    mode = mode,
                    root = path.root_index,
                    label = root_label,
                    pivots = pivots
                );
            }
        }
        out
    }
}

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct ScaleSearchReport {
    pub system: String,
    pub criteria: Vec<u8>,
    pub match_count: usize,
    pub matches: Vec<ScaleSearchMatch>,
}

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct ScaleSearchMatch {
    pub scale: String,
    pub root_index: i32,
    pub root_label: String,
}

impl ScaleSearchReport {
    pub fn render_text(&self) -> String {
        let mut out = String::new();
        let _ = writeln!(
            &mut out,
            "Scale search in {system}: {count} match(es) for pcs {pcs:?}.",
            system = self.system,
            count = self.match_count,
            pcs = self.criteria
        );
        for entry in &self.matches {
            let _ = writeln!(
                &mut out,
                "  - {scale} rooted at {label} ({index}).",
                scale = entry.scale,
                label = entry.root_label,
                index = entry.root_index
            );
        }
        out
    }
}

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
