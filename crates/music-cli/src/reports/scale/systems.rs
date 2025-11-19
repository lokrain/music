//! Tuning system listing report types.

use std::fmt::Write as FmtWrite;

use serde::Serialize;

#[cfg(feature = "schema")]
use schemars::JsonSchema;

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
