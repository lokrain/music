#![allow(dead_code)]
use serde::Serialize;
use std::fmt::Write as FmtWrite;

#[cfg(feature = "schema")]
use schemars::JsonSchema;

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct PatternContext {
    pub system: String,
    pub scale: String,
    pub root_index: i32,
    pub root_label: Option<String>,
    pub density: String,
}

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct GeneratedNote {
    pub degree: usize,
    pub octave: i32,
    pub index: i32,
    pub label: Option<String>,
    pub frequency_hz: f32,
}

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct MotifGeneration {
    pub context: PatternContext,
    pub note_count: usize,
    pub contour: Vec<i32>,
    pub notes: Vec<GeneratedNote>,
    pub description: String,
}

impl MotifGeneration {
    pub fn render_text(&self) -> String {
        let mut out = String::new();
        let root_label = self
            .context
            .root_label
            .as_deref()
            .map(|label| format!(" ({label})"))
            .unwrap_or_default();
        let _ = writeln!(
            &mut out,
            "Motif ({density}) in {scale} / {system}, root {root}{root_label}.",
            density = self.context.density,
            scale = self.context.scale,
            system = self.context.system,
            root = self.context.root_index,
            root_label = root_label
        );
        if !self.description.is_empty() {
            let _ = writeln!(&mut out, "{desc}", desc = self.description);
        }
        if !self.contour.is_empty() {
            let markings: Vec<String> = self
                .contour
                .iter()
                .enumerate()
                .map(|(idx, delta)| format!("Δ{step:02} {delta:+}", step = idx + 1, delta = delta))
                .collect();
            let _ = writeln!(&mut out, "Contour: {values}.", values = markings.join(", "));
        }
        for (idx, note) in self.notes.iter().enumerate() {
            let label = note.label.as_deref().unwrap_or("—");
            let _ = writeln!(
                &mut out,
                "  {order:>2}. degree {degree}{octave:+}, index {index}, {label} ({freq:.3} Hz)",
                order = idx + 1,
                degree = note.degree,
                octave = note.octave,
                index = note.index,
                label = label,
                freq = note.frequency_hz
            );
        }
        out
    }
}

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct ArpeggioStep {
    pub order: usize,
    pub direction: String,
    #[serde(flatten)]
    pub note: GeneratedNote,
}

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct ArpeggioGeneration {
    pub context: PatternContext,
    pub register_span: i32,
    pub steps: Vec<ArpeggioStep>,
    pub description: String,
}

impl ArpeggioGeneration {
    pub fn render_text(&self) -> String {
        let mut out = String::new();
        let root_label = self
            .context
            .root_label
            .as_deref()
            .map(|label| format!(" ({label})"))
            .unwrap_or_default();
        let _ = writeln!(
            &mut out,
            "Arpeggio ({density}) in {scale} / {system}, root {root}{root_label}. Span {span} semitones.",
            density = self.context.density,
            scale = self.context.scale,
            system = self.context.system,
            root = self.context.root_index,
            root_label = root_label,
            span = self.register_span
        );
        if !self.description.is_empty() {
            let _ = writeln!(&mut out, "{desc}", desc = self.description);
        }
        for step in &self.steps {
            let label = step.note.label.as_deref().unwrap_or("—");
            let _ = writeln!(
                &mut out,
                "  {order:>2}. [{direction:^4}] degree {degree}{octave:+}, index {index}, {label} ({freq:.3} Hz)",
                order = step.order,
                direction = step.direction,
                degree = step.note.degree,
                octave = step.note.octave,
                index = step.note.index,
                label = label,
                freq = step.note.frequency_hz
            );
        }
        out
    }
}

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct RhythmEvent {
    pub beat: f32,
    pub duration_beats: f32,
    pub accent: bool,
    pub suggested_degree: usize,
}

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct RhythmCellGeneration {
    pub context: PatternContext,
    pub meter: String,
    pub length_beats: f32,
    pub events: Vec<RhythmEvent>,
    pub description: String,
}

impl RhythmCellGeneration {
    pub fn render_text(&self) -> String {
        let mut out = String::new();
        let root_label = self
            .context
            .root_label
            .as_deref()
            .map(|label| format!(" ({label})"))
            .unwrap_or_default();
        let _ = writeln!(
            &mut out,
            "Rhythm cell ({density}) for {scale} / {system}, root {root}{root_label}, meter {meter}, {beats:.1} beats.",
            density = self.context.density,
            scale = self.context.scale,
            system = self.context.system,
            root = self.context.root_index,
            root_label = root_label,
            meter = self.meter,
            beats = self.length_beats
        );
        if !self.description.is_empty() {
            let _ = writeln!(&mut out, "{desc}", desc = self.description);
        }
        if self.events.is_empty() {
            return out;
        }

        let _ = writeln!(&mut out, "  beat  dur  acc  degree");
        for event in &self.events {
            let accent = if event.accent { "*" } else { "." };
            let _ = writeln!(
                &mut out,
                "  {beat:>4.1}  {dur:>3.1}   {accent}     {degree}",
                beat = event.beat,
                dur = event.duration_beats,
                accent = accent,
                degree = event.suggested_degree
            );
        }
        out
    }
}
