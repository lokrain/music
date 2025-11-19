//! Profile and estimation reports for timing, density, and musical features.

use std::fmt::Write;

#[cfg(feature = "schema")]
use schemars::JsonSchema;
use serde::Serialize;

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct ProfileReport {
    pub input_type: String,
    pub event_count: usize,
    pub total_duration_sec: Option<f64>,
    pub density_events_per_sec: Option<f64>,
    pub pitch_range: PitchRangeReport,
    pub timing: TimingReport,
}

impl ProfileReport {
    pub fn render_text(&self) -> String {
        let mut out = String::new();
        let _ = writeln!(
            &mut out,
            "Profile for {} ({} events).",
            self.input_type, self.event_count
        );

        if let Some(duration) = self.total_duration_sec {
            let _ = writeln!(&mut out, "Total duration: {:.2} seconds.", duration);
        }

        if let Some(density) = self.density_events_per_sec {
            let _ = writeln!(&mut out, "Density: {:.2} events/second.", density);
        }

        let _ = writeln!(&mut out, "\nPitch Range:");
        if let Some(min) = self.pitch_range.min_pitch {
            let _ = writeln!(&mut out, "  Min pitch: {}", min);
        }
        if let Some(max) = self.pitch_range.max_pitch {
            let _ = writeln!(&mut out, "  Max pitch: {}", max);
        }
        if let Some(median) = self.pitch_range.median_pitch {
            let _ = writeln!(&mut out, "  Median pitch: {:.1}", median);
        }
        if let Some(p25) = self.pitch_range.p25_pitch {
            let _ = writeln!(&mut out, "  25th percentile: {:.1}", p25);
        }
        if let Some(p75) = self.pitch_range.p75_pitch {
            let _ = writeln!(&mut out, "  75th percentile: {:.1}", p75);
        }

        let _ = writeln!(&mut out, "\nTiming:");
        if let Some(min) = self.timing.min_ioi_sec {
            let _ = writeln!(&mut out, "  Min IOI: {:.3} seconds.", min);
        }
        if let Some(max) = self.timing.max_ioi_sec {
            let _ = writeln!(&mut out, "  Max IOI: {:.3} seconds.", max);
        }
        if let Some(median) = self.timing.median_ioi_sec {
            let _ = writeln!(&mut out, "  Median IOI: {:.3} seconds.", median);
        }
        if let Some(p25) = self.timing.p25_ioi_sec {
            let _ = writeln!(&mut out, "  25th percentile IOI: {:.3} seconds.", p25);
        }
        if let Some(p75) = self.timing.p75_ioi_sec {
            let _ = writeln!(&mut out, "  75th percentile IOI: {:.3} seconds.", p75);
        }
        if let Some(swing) = self.timing.swing_ratio {
            let _ = writeln!(&mut out, "  Detected swing ratio: {:.2}", swing);
        }

        out
    }
}

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct PitchRangeReport {
    pub min_pitch: Option<u8>,
    pub max_pitch: Option<u8>,
    pub median_pitch: Option<f64>,
    pub p25_pitch: Option<f64>,
    pub p75_pitch: Option<f64>,
}

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct TimingReport {
    pub min_ioi_sec: Option<f64>,
    pub max_ioi_sec: Option<f64>,
    pub median_ioi_sec: Option<f64>,
    pub p25_ioi_sec: Option<f64>,
    pub p75_ioi_sec: Option<f64>,
    pub swing_ratio: Option<f64>,
}
