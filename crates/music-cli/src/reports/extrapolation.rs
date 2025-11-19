//! Melody and chord extrapolation reports using Markov/n-gram models.

use std::fmt::Write;

#[cfg(feature = "schema")]
use schemars::JsonSchema;
use serde::Serialize;

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct MelodyExtrapolationReport {
    pub system: String,
    pub input_count: usize,
    pub input_notes: Vec<i32>,
    pub model_order: usize,
    pub context_pitch_classes: Vec<u8>,
    pub predictions: Vec<MelodyPrediction>,
    pub key_hint: Option<String>,
}

impl MelodyExtrapolationReport {
    pub fn render_text(&self) -> String {
        let mut out = String::new();
        let _ = writeln!(
            &mut out,
            "Melody extrapolation (n-gram order {order}) in {system}.",
            order = self.model_order,
            system = self.system
        );
        let _ = writeln!(
            &mut out,
            "Input: {count} notes → pitch classes {pcs:?}",
            count = self.input_count,
            pcs = self.context_pitch_classes
        );
        if let Some(key) = &self.key_hint {
            let _ = writeln!(&mut out, "Context key: {key}.");
        }
        if self.predictions.is_empty() {
            let _ = writeln!(
                &mut out,
                "No predictions available (context not seen in training)."
            );
            return out;
        }
        let _ = writeln!(&mut out, "Predicted continuations:");
        for (idx, pred) in self.predictions.iter().enumerate() {
            let label = pred.label.as_deref().unwrap_or("—");
            let _ = writeln!(
                &mut out,
                "  {rank:>2}. pc {pc:>2} → index {index:>3} ({label}) — {prob:.1}% ({freq:.3} Hz)",
                rank = idx + 1,
                pc = pred.pitch_class,
                index = pred.suggested_index,
                label = label,
                prob = pred.probability * 100.0,
                freq = pred.frequency_hz
            );
        }
        out
    }
}

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct MelodyPrediction {
    pub pitch_class: u8,
    pub suggested_index: i32,
    pub label: Option<String>,
    pub frequency_hz: f32,
    pub probability: f32,
}

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct ChordExtrapolationReport {
    pub input_count: usize,
    pub input_progression: Vec<String>,
    pub model_order: usize,
    pub context_chords: Vec<String>,
    pub predictions: Vec<ChordPrediction>,
    pub key_hint: Option<String>,
}

impl ChordExtrapolationReport {
    pub fn render_text(&self) -> String {
        let mut out = String::new();
        let _ = writeln!(
            &mut out,
            "Chord extrapolation (n-gram order {order}).",
            order = self.model_order
        );
        let _ = writeln!(
            &mut out,
            "Input progression ({count} chords): {prog}",
            count = self.input_count,
            prog = self.input_progression.join(" → ")
        );
        let _ = writeln!(
            &mut out,
            "Context (last {order} chord(s)): {ctx}",
            order = self.model_order,
            ctx = self.context_chords.join(" → ")
        );
        if let Some(key) = &self.key_hint {
            let _ = writeln!(&mut out, "Context key: {key}.");
        }
        if self.predictions.is_empty() {
            let _ = writeln!(
                &mut out,
                "No predictions available (context not seen in training)."
            );
            return out;
        }
        let _ = writeln!(&mut out, "Predicted next chords:");
        for (idx, pred) in self.predictions.iter().enumerate() {
            let _ = writeln!(
                &mut out,
                "  {rank:>2}. {chord:<6} — {prob:.1}%",
                rank = idx + 1,
                chord = pred.chord,
                prob = pred.probability * 100.0
            );
        }
        out
    }
}

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct ChordPrediction {
    pub chord: String,
    pub probability: f32,
}
