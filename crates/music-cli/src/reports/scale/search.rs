//! Scale search report types.

use std::fmt::Write as FmtWrite;

use serde::Serialize;

#[cfg(feature = "schema")]
use schemars::JsonSchema;

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
