//! Scale mapping and modulatory path analysis report types.

use std::fmt::Write as FmtWrite;

use serde::Serialize;

#[cfg(feature = "schema")]
use schemars::JsonSchema;

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
