use music_engine::prelude::PitchError;
use serde::Serialize;
use std::fmt::Write as FmtWrite;

#[cfg(feature = "schema")]
use schemars::JsonSchema;

pub type PitchResult<T> = Result<T, PitchError>;

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct PitchSummary {
    pub system: String,
    pub index: i32,
    pub label: String,
    pub frequency_hz: f32,
}

impl PitchSummary {
    pub fn render_text(&self) -> String {
        format!(
            "Pitch {index} in {system}: {label} ({freq:.3} Hz)",
            index = self.index,
            system = self.system,
            label = self.label,
            freq = self.frequency_hz
        )
    }
}

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct PitchExplanation {
    pub summary: PitchSummary,
    pub octave: i32,
    pub pitch_class: u8,
    pub pitch_class_label: String,
    pub semitone_delta_from_a4: i32,
    pub cents_offset_from_a4: f32,
    pub context: Option<PitchContext>,
    pub narrative: Vec<String>,
}

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct PitchContext {
    pub key: String,
    pub mode: String,
    pub degree: usize,
    pub degree_label: String,
    pub function: String,
}

impl PitchExplanation {
    pub fn render_text(&self) -> String {
        let mut out = String::new();
        let summary = self.summary.render_text();
        let _ = writeln!(&mut out, "{summary}");
        let _ = writeln!(
            &mut out,
            "Octave {octave}, pitch class {pc} ({label}).",
            octave = self.octave,
            pc = self.pitch_class,
            label = self.pitch_class_label
        );
        let _ = writeln!(
            &mut out,
            "Relative to A4: {delta:+} semitone(s), {cents:+.1} cents.",
            delta = self.semitone_delta_from_a4,
            cents = self.cents_offset_from_a4
        );
        if let Some(context) = &self.context {
            let _ = writeln!(
                &mut out,
                "In {key} {mode}, this is degree {degree} ({label}), functioning as {function}.",
                key = context.key,
                mode = context.mode,
                degree = context.degree,
                label = context.degree_label,
                function = context.function
            );
        }
        for paragraph in &self.narrative {
            let _ = writeln!(&mut out);
            let _ = writeln!(&mut out, "{paragraph}");
        }
        out
    }
}
