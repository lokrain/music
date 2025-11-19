#![allow(dead_code)]
use serde::Serialize;
use std::fmt::Write as FmtWrite;

#[cfg(feature = "schema")]
use schemars::JsonSchema;

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct EstimateReport {
    pub input_type: String,
    pub tempo_bpm: Option<f64>,
    pub tempo_confidence: Option<f64>,
    pub key_estimate: Option<String>,
    pub key_confidence: Option<f64>,
    pub meter: Option<String>,
    pub meter_confidence: Option<f64>,
}

impl EstimateReport {
    pub fn render_text(&self) -> String {
        let mut out = String::new();
        let _ = writeln!(
            &mut out,
            "Musical feature estimation for {}.",
            self.input_type
        );

        if let Some(key) = &self.key_estimate {
            let conf = self.key_confidence.unwrap_or(0.0);
            let _ = writeln!(
                &mut out,
                "\nEstimated key: {} (confidence: {:.1}%)",
                key,
                conf * 100.0
            );
        } else {
            let _ = writeln!(&mut out, "\nEstimated key: (not available)");
        }

        if let Some(tempo) = self.tempo_bpm {
            let conf = self.tempo_confidence.unwrap_or(0.0);
            let _ = writeln!(
                &mut out,
                "Estimated tempo: {:.1} BPM (confidence: {:.1}%)",
                tempo,
                conf * 100.0
            );
        } else {
            let _ = writeln!(
                &mut out,
                "Estimated tempo: (not available - no timing data)"
            );
        }

        if let Some(meter) = &self.meter {
            let conf = self.meter_confidence.unwrap_or(0.0);
            let _ = writeln!(
                &mut out,
                "Estimated meter: {} (confidence: {:.1}%)",
                meter,
                conf * 100.0
            );
        } else {
            let _ = writeln!(
                &mut out,
                "Estimated meter: (not available - no timing data)"
            );
        }

        out
    }
}
