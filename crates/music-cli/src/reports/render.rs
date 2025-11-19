//! Staff and piano-roll rendering reports.

use std::fmt::Write;

#[cfg(feature = "schema")]
use schemars::JsonSchema;
use serde::Serialize;

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct StaffRenderReport {
    pub system: String,
    pub unicode: bool,
    pub key_hint: Option<String>,
    pub note_count: usize,
    pub min_index: i32,
    pub max_index: i32,
    pub rows: Vec<String>,
    pub note_labels: Vec<String>,
}

impl StaffRenderReport {
    pub fn render_text(&self) -> String {
        let mut out = String::new();
        let mode = if self.unicode { "Unicode" } else { "ASCII" };
        let _ = writeln!(
            &mut out,
            "Staff render ({mode}) for {count} note(s) in {system} [{min}..{max}].",
            mode = mode,
            count = self.note_count,
            system = self.system,
            min = self.min_index,
            max = self.max_index
        );
        if let Some(key) = &self.key_hint {
            let _ = writeln!(&mut out, "Context key: {key}.");
        }
        for row in &self.rows {
            let _ = writeln!(&mut out, "{row}");
        }
        if !self.note_labels.is_empty() {
            let _ = writeln!(&mut out, "Notes:");
            for label in &self.note_labels {
                let _ = writeln!(&mut out, "  • {label}");
            }
        }
        out
    }
}

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct PianoRollRenderReport {
    pub system: String,
    pub width: usize,
    pub height: usize,
    pub note_count: usize,
    pub min_index: i32,
    pub max_index: i32,
    pub rows: Vec<String>,
    pub note_labels: Vec<String>,
}

impl PianoRollRenderReport {
    pub fn render_text(&self) -> String {
        let mut out = String::new();
        let _ = writeln!(
            &mut out,
            "Piano-roll render ({width}×{height}) for {count} note(s) in {system} [{min}..{max}].",
            width = self.width,
            height = self.height,
            count = self.note_count,
            system = self.system,
            min = self.min_index,
            max = self.max_index
        );
        for row in &self.rows {
            let _ = writeln!(&mut out, "{row}");
        }
        if !self.note_labels.is_empty() {
            let _ = writeln!(&mut out, "Notes:");
            for label in &self.note_labels {
                let _ = writeln!(&mut out, "  • {label}");
            }
        }
        out
    }
}
