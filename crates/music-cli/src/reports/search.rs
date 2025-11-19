//! Chord and scale search report types.

use std::fmt::Write as FmtWrite;

use serde::Serialize;

#[cfg(feature = "schema")]
use schemars::JsonSchema;

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct ChordSearchReport {
    pub system: String,
    pub criteria: Vec<u8>,
    pub voicing: String,
    pub match_count: usize,
    pub matches: Vec<ChordSearchMatch>,
}

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct ChordSearchMatch {
    pub scale: String,
    pub degree: usize,
    pub numeral: String,
    pub root_label: String,
    pub pitch_classes: Vec<u8>,
}

impl ChordSearchReport {
    pub fn render_text(&self) -> String {
        let mut out = String::new();
        let _ = writeln!(
            &mut out,
            "Chord search ({voicing}) in {system}: {count} match(es) for pcs {pcs:?}.",
            voicing = self.voicing,
            system = self.system,
            count = self.match_count,
            pcs = self.criteria
        );
        for entry in &self.matches {
            let _ = writeln!(
                &mut out,
                "  - {scale} degree {degree} ({numeral}) root {root}, pcs {pcs:?}.",
                scale = entry.scale,
                degree = entry.degree,
                numeral = entry.numeral,
                root = entry.root_label,
                pcs = entry.pitch_classes
            );
        }
        out
    }
}
