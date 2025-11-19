use serde::Serialize;
use std::fmt::Write as FmtWrite;

#[cfg(feature = "schema")]
use schemars::JsonSchema;

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct VoiceResolution {
    pub from_index: Option<i32>,
    pub from_pc: u8,
    pub to_pc: u8,
    pub semitones: i8,
    pub direction: String,
}

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct ChordResolutionReport {
    pub key: String,
    pub system: String,
    pub input_description: String,
    pub target_description: String,
    pub resolutions: Vec<VoiceResolution>,
}

impl ChordResolutionReport {
    pub fn render_text(&self) -> String {
        let mut out = String::new();
        let _ = writeln!(
            &mut out,
            "Resolution in {key} ({system}) — {input} → {target}:",
            key = self.key,
            system = self.system,
            input = self.input_description,
            target = self.target_description
        );
        if self.resolutions.is_empty() {
            let _ = writeln!(&mut out, "No stepwise resolutions available.");
            return out;
        }
        let _ = writeln!(&mut out, "Suggested voice-leading:");
        for vr in &self.resolutions {
            let from = format!("pc {}", vr.from_pc);
            let to = format!("pc {}", vr.to_pc);
            let delta = vr.semitones;
            let _ = writeln!(
                &mut out,
                "  {from:<6} → {to:<6}  ({dir} {delta:+} st)",
                from = from,
                to = to,
                dir = vr.direction,
                delta = delta
            );
        }
        out
    }
}
