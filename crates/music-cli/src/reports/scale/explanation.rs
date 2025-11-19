//! Scale explanation report types.

use std::fmt::Write as FmtWrite;

use serde::Serialize;

#[cfg(feature = "schema")]
use schemars::JsonSchema;

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
