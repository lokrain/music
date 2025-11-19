//! Interpolation envelope report types.

use std::fmt::Write as FmtWrite;

use serde::Serialize;

#[cfg(feature = "schema")]
use schemars::JsonSchema;

#[derive(Debug, Serialize, Clone)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct InterpolatedPoint {
    pub time: f32,
    pub value: f32,
}

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct InterpolationContext {
    pub curve: String,
    pub samples: usize,
}

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct InterpolatedEnvelopeReport {
    pub context: InterpolationContext,
    pub unit: String,
    pub anchors: Vec<InterpolatedPoint>,
    pub samples: Vec<InterpolatedPoint>,
}

impl InterpolatedEnvelopeReport {
    pub fn render_text(&self) -> String {
        let mut out = String::new();
        let _ = writeln!(
            &mut out,
            "Interpolation ({unit}) — curve {curve}, {count} anchors, {samples} samples.",
            unit = self.unit,
            curve = self.context.curve,
            count = self.anchors.len(),
            samples = self.samples.len()
        );
        let _ = writeln!(&mut out, "Anchors:");
        for anchor in &self.anchors {
            let _ = writeln!(
                &mut out,
                "  t={time:>5.2} → {value:>7.3}",
                time = anchor.time,
                value = anchor.value
            );
        }
        let _ = writeln!(&mut out, "Samples:");
        for point in &self.samples {
            let _ = writeln!(
                &mut out,
                "  t={time:>5.2} → {value:>7.3}",
                time = point.time,
                value = point.value
            );
        }
        out
    }
}

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct VelocityEnvelopeReport {
    pub context: InterpolationContext,
    pub anchors: Vec<InterpolatedPoint>,
    pub samples: Vec<InterpolatedPoint>,
    pub min_value: i32,
    pub max_value: i32,
}

impl VelocityEnvelopeReport {
    pub fn render_text(&self) -> String {
        let mut out = String::new();
        let _ = writeln!(
            &mut out,
            "Velocity interpolation [{min}..{max}] — curve {curve}, {count} anchors.",
            min = self.min_value,
            max = self.max_value,
            curve = self.context.curve,
            count = self.anchors.len()
        );
        let _ = writeln!(&mut out, "Anchors:");
        for anchor in &self.anchors {
            let _ = writeln!(
                &mut out,
                "  t={time:>5.2} → {value:>6.1}",
                time = anchor.time,
                value = anchor.value
            );
        }
        let _ = writeln!(&mut out, "Samples:");
        for point in &self.samples {
            let _ = writeln!(
                &mut out,
                "  t={time:>5.2} → {value:>6.1}",
                time = point.time,
                value = point.value
            );
        }
        out
    }
}
