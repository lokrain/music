use anyhow::{bail, Context as _, Result};
use music_analysis::{estimate_from_melody, estimate_from_midi};
use music_engine::prelude::*;

use crate::{
    cli::{EstimateCommand, EstimateMelodyArgs, EstimateMidiArgs},
    format::OutputFormat,
    responses::EstimateReport,
};

pub fn handle_estimate(
    _engine: &MusicEngine,
    format: OutputFormat,
    command: EstimateCommand,
) -> Result<()> {
    match command {
        EstimateCommand::Melody(args) => estimate_melody_handler(format, args),
        EstimateCommand::Midi(args) => estimate_midi_handler(format, args),
    }
}

fn estimate_melody_handler(format: OutputFormat, args: EstimateMelodyArgs) -> Result<()> {
    if args.notes.is_empty() {
        bail!("provide at least one note via --notes");
    }

    // Convert to u8 pitch indices
    let pitches: Vec<u8> = args
        .notes
        .iter()
        .map(|&index| {
            let mut pc = index % 128;
            if pc < 0 {
                pc += 128;
            }
            pc as u8
        })
        .collect();

    let estimate = estimate_from_melody(&pitches);

    let key_name = estimate
        .key_estimate
        .map(pitch_class_name)
        .or(Some("Unknown".to_string()));

    let report = EstimateReport {
        input_type: "melody".to_string(),
        tempo_bpm: estimate.tempo_bpm,
        tempo_confidence: Some(estimate.tempo_confidence),
        key_estimate: key_name,
        key_confidence: Some(estimate.key_confidence),
        meter: estimate.meter,
        meter_confidence: Some(estimate.meter_confidence),
    };

    format.emit(&report, |r| r.render_text())
}

fn estimate_midi_handler(format: OutputFormat, args: EstimateMidiArgs) -> Result<()> {
    // Read MIDI file
    let midi_data = std::fs::read(&args.file)
        .with_context(|| format!("failed to read MIDI file: {:?}", args.file))?;

    let smf = midly::Smf::parse(&midi_data)
        .with_context(|| format!("failed to parse MIDI file: {:?}", args.file))?;

    // Extract note events with timing
    let mut events: Vec<(u8, f64)> = Vec::new();
    let ticks_per_beat = match smf.header.timing {
        midly::Timing::Metrical(tpb) => tpb.as_int() as f64,
        midly::Timing::Timecode(_, _) => 480.0, // Default fallback
    };

    // Simple tempo assumption: 120 BPM = 0.5 seconds per beat
    let seconds_per_tick = 0.5 / ticks_per_beat;

    for track in &smf.tracks {
        let mut current_time = 0u64;
        for event in track {
            current_time += event.delta.as_int() as u64;
            if let midly::TrackEventKind::Midi { channel: _, message } = event.kind
                && let midly::MidiMessage::NoteOn { key, vel } = message
                    && vel > 0 {
                        let pitch = key.as_int();
                        let onset_sec = current_time as f64 * seconds_per_tick;
                        events.push((pitch, onset_sec));
                    }
            }
    }

    if events.is_empty() {
        bail!("no note events found in MIDI file");
    }

    let estimate = estimate_from_midi(&events);

    let key_name = estimate
        .key_estimate
        .map(pitch_class_name)
        .or(Some("Unknown".to_string()));

    let report = EstimateReport {
        input_type: format!("MIDI file: {:?}", args.file.file_name().unwrap_or_default()),
        tempo_bpm: estimate.tempo_bpm,
        tempo_confidence: Some(estimate.tempo_confidence),
        key_estimate: key_name,
        key_confidence: Some(estimate.key_confidence),
        meter: estimate.meter,
        meter_confidence: Some(estimate.meter_confidence),
    };

    format.emit(&report, |r| r.render_text())
}

/// Convert pitch class (0-11) to note name.
fn pitch_class_name(pc: u8) -> String {
    match pc {
        0 => "C",
        1 => "C♯/D♭",
        2 => "D",
        3 => "D♯/E♭",
        4 => "E",
        5 => "F",
        6 => "F♯/G♭",
        7 => "G",
        8 => "G♯/A♭",
        9 => "A",
        10 => "A♯/B♭",
        11 => "B",
        _ => "?",
    }
    .to_string()
}
