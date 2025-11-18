use anyhow::{Context as _, Result, bail};
use music_analysis::{profile_melody, profile_midi};
use music_engine::prelude::*;

use crate::{
    cli::{ProfileCommand, ProfileMelodyArgs, ProfileMidiArgs},
    format::OutputFormat,
    responses::{PitchRangeReport, ProfileReport, TimingReport},
};

pub fn handle_profile(
    _engine: &MusicEngine,
    format: OutputFormat,
    command: ProfileCommand,
) -> Result<()> {
    match command {
        ProfileCommand::Melody(args) => profile_melody_handler(format, args),
        ProfileCommand::Midi(args) => profile_midi_handler(format, args),
    }
}

fn profile_melody_handler(format: OutputFormat, args: ProfileMelodyArgs) -> Result<()> {
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

    let stats = profile_melody(&pitches);

    let report = ProfileReport {
        input_type: "melody".to_string(),
        event_count: stats.event_count,
        total_duration_sec: stats.total_duration_sec,
        density_events_per_sec: stats.density_events_per_sec,
        pitch_range: PitchRangeReport {
            min_pitch: stats.pitch_range.min_pitch,
            max_pitch: stats.pitch_range.max_pitch,
            median_pitch: stats.pitch_range.median_pitch,
            p25_pitch: stats.pitch_range.p25_pitch,
            p75_pitch: stats.pitch_range.p75_pitch,
        },
        timing: TimingReport {
            min_ioi_sec: stats.timing.min_ioi_sec,
            max_ioi_sec: stats.timing.max_ioi_sec,
            median_ioi_sec: stats.timing.median_ioi_sec,
            p25_ioi_sec: stats.timing.p25_ioi_sec,
            p75_ioi_sec: stats.timing.p75_ioi_sec,
            swing_ratio: stats.timing.swing_ratio,
        },
    };

    format.emit(&report, |r| r.render_text())
}

fn profile_midi_handler(format: OutputFormat, args: ProfileMidiArgs) -> Result<()> {
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
            if let midly::TrackEventKind::Midi {
                channel: _,
                message,
            } = event.kind
                && let midly::MidiMessage::NoteOn { key, vel } = message
                && vel > 0
            {
                let pitch = key.as_int();
                let onset_sec = current_time as f64 * seconds_per_tick;
                events.push((pitch, onset_sec));
            }
        }
    }

    if events.is_empty() {
        bail!("no note events found in MIDI file");
    }

    let stats = profile_midi(&events);

    let report = ProfileReport {
        input_type: format!("MIDI file: {:?}", args.file.file_name().unwrap_or_default()),
        event_count: stats.event_count,
        total_duration_sec: stats.total_duration_sec,
        density_events_per_sec: stats.density_events_per_sec,
        pitch_range: PitchRangeReport {
            min_pitch: stats.pitch_range.min_pitch,
            max_pitch: stats.pitch_range.max_pitch,
            median_pitch: stats.pitch_range.median_pitch,
            p25_pitch: stats.pitch_range.p25_pitch,
            p75_pitch: stats.pitch_range.p75_pitch,
        },
        timing: TimingReport {
            min_ioi_sec: stats.timing.min_ioi_sec,
            max_ioi_sec: stats.timing.max_ioi_sec,
            median_ioi_sec: stats.timing.median_ioi_sec,
            p25_ioi_sec: stats.timing.p25_ioi_sec,
            p75_ioi_sec: stats.timing.p75_ioi_sec,
            swing_ratio: stats.timing.swing_ratio,
        },
    };

    format.emit(&report, |r| r.render_text())
}
