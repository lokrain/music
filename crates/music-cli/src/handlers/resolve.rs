use anyhow::{Result, bail};
use music_engine::prelude::*;

use crate::{
    cli::{ResolveChordArgs, ResolveCommand, ResolveNotesArgs},
    format::OutputFormat,
    responses::{ChordResolutionReport, VoiceResolution},
};

pub fn handle_resolve(
    _engine: &MusicEngine,
    format: OutputFormat,
    command: ResolveCommand,
) -> Result<()> {
    match command {
        ResolveCommand::Chord(args) => resolve_chord(format, args),
        ResolveCommand::Notes(args) => resolve_notes(format, args),
    }
}

fn parse_key_to_pc(key: &str) -> Option<(u8, bool)> {
    // Very small parser: e.g., Cmaj, Amin
    if key.len() < 2 {
        return None;
    }
    let (name, qual) = key.split_at(key.len().saturating_sub(3));
    let qual = qual.to_ascii_lowercase();
    let is_minor = qual.contains("min");
    let root = name.trim();
    let pc = match root {
        "C" => 0,
        "C#" | "C♯" | "Db" | "D♭" => 1,
        "D" => 2,
        "D#" | "D♯" | "Eb" | "E♭" => 3,
        "E" => 4,
        "F" => 5,
        "F#" | "F♯" | "Gb" | "G♭" => 6,
        "G" => 7,
        "G#" | "G♯" | "Ab" | "A♭" => 8,
        "A" => 9,
        "A#" | "A♯" | "Bb" | "B♭" => 10,
        "B" => 11,
        _ => return None,
    };
    Some((pc, is_minor))
}

fn major_triad_pcs(tonic_pc: u8) -> [u8; 3] {
    [tonic_pc % 12, (tonic_pc + 4) % 12, (tonic_pc + 7) % 12]
}
fn minor_triad_pcs(tonic_pc: u8) -> [u8; 3] {
    [tonic_pc % 12, (tonic_pc + 3) % 12, (tonic_pc + 7) % 12]
}

fn nearest_triad_pc(pc: u8, triad: &[u8; 3]) -> (u8, i8) {
    let mut best = (triad[0], distance_signed(pc, triad[0]));
    for &t in triad.iter().skip(1) {
        let d = distance_signed(pc, t);
        if d.abs() < best.1.abs() {
            best = (t, d);
        }
    }
    best
}

fn distance_signed(from: u8, to: u8) -> i8 {
    let from = from as i16;
    let to = to as i16;
    let mut d = (to - from) % 12;
    if d > 6 {
        d -= 12;
    }
    if d < -6 {
        d += 12;
    }
    d as i8
}

fn resolve_chord(format: OutputFormat, args: ResolveChordArgs) -> Result<()> {
    let (tonic_pc, is_minor) =
        parse_key_to_pc(&args.key).ok_or_else(|| anyhow::anyhow!("invalid key: {}", args.key))?;

    // Basic V / V7 handling; otherwise map every tone to nearest tonic triad tone
    let triad = if is_minor {
        minor_triad_pcs(tonic_pc)
    } else {
        major_triad_pcs(tonic_pc)
    };

    let pcs: Vec<u8> = match args.chord.trim().to_ascii_lowercase().as_str() {
        "v" | "v7" => {
            // Build V7 relative to key
            let dom = (tonic_pc + 7) % 12; // degree 5
            let third = (dom + 4) % 12; // major third of V
            let fifth = (dom + 7) % 12;
            let seventh = (dom + 10) % 12; // minor 7th
            vec![dom, third, fifth, seventh]
        }
        other => {
            // Fallback: try interpret numeral root; else treat as pitch-class list like 0,4,7
            if other.chars().all(|c| c.is_ascii_digit() || c == ',') {
                other
                    .split(',')
                    .filter_map(|s| s.parse::<u8>().ok().map(|v| v % 12))
                    .collect()
            } else {
                // Default to scale degrees 1-3-5 as input (no-op mapping)
                triad.to_vec()
            }
        }
    };

    let mut resolutions = Vec::new();
    for pc in pcs {
        // Special voice-leading for leading tone (7) and 4 → 3
        let rel = (pc + 12 - tonic_pc) % 12;
        let (to_pc, delta) = match rel {
            11 => (tonic_pc % 12, 1),                                  // 7→1 up
            5 => ((tonic_pc + if is_minor { 3 } else { 4 }) % 12, -1), // 4→3 down
            _ => nearest_triad_pc(pc, &triad),
        };
        resolutions.push(VoiceResolution {
            from_index: None,
            from_pc: pc,
            to_pc,
            semitones: delta,
            direction: if delta > 0 {
                "up".into()
            } else if delta < 0 {
                "down".into()
            } else {
                "stay".into()
            },
        });
    }

    let report = ChordResolutionReport {
        key: args.key,
        system: args.system,
        input_description: format!("Chord {}", args.chord),
        target_description: if is_minor { "i" } else { "I" }.to_string(),
        resolutions,
    };

    format.emit(&report, |r| r.render_text())
}

fn resolve_notes(format: OutputFormat, args: ResolveNotesArgs) -> Result<()> {
    if args.notes.is_empty() {
        bail!("provide at least one note via --notes");
    }
    let (tonic_pc, is_minor) =
        parse_key_to_pc(&args.key).ok_or_else(|| anyhow::anyhow!("invalid key: {}", args.key))?;
    let triad = if is_minor {
        minor_triad_pcs(tonic_pc)
    } else {
        major_triad_pcs(tonic_pc)
    };

    let mut resolutions = Vec::new();
    for &idx in &args.notes {
        let mut pc = idx % 12;
        if pc < 0 {
            pc += 12;
        }
        let pc = pc as u8;
        let (to_pc, delta) = nearest_triad_pc(pc, &triad);
        resolutions.push(VoiceResolution {
            from_index: Some(idx),
            from_pc: pc,
            to_pc,
            semitones: delta,
            direction: if delta > 0 {
                "up".into()
            } else if delta < 0 {
                "down".into()
            } else {
                "stay".into()
            },
        });
    }

    let report = ChordResolutionReport {
        key: args.key,
        system: args.system,
        input_description: format!(
            "Notes {}",
            args.notes
                .iter()
                .map(|n| n.to_string())
                .collect::<Vec<_>>()
                .join(",")
        ),
        target_description: if is_minor { "i" } else { "I" }.to_string(),
        resolutions,
    };

    format.emit(&report, |r| r.render_text())
}
