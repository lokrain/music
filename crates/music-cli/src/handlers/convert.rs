use std::{
    collections::{BTreeMap, HashMap, VecDeque},
    fs,
    path::Path,
};

use anyhow::{Context, Result, anyhow, bail};
use midly::{
    Format, Header, MetaMessage, MidiMessage, Smf, Timing, TrackEvent, TrackEventKind,
    num::{u4, u7, u15, u28},
};
use music_engine::prelude::*;

use crate::{
    cli::{
        ConvertCommand, ConvertCsvToMidiArgs, ConvertFrequencyArgs, ConvertMidiToCsvArgs,
        ConvertPitchArgs, ConvertTemperamentArgs, MidiConvertCommand, PitchConvertCommand,
    },
    format::OutputFormat,
    responses::{
        FrequencyToIndexReport, MidiConversionDirection, MidiCsvConversionReport, MidiCsvRow,
        PitchIndexToFrequency, TemperamentMappingRow, TemperamentRemapReport,
    },
};

const MAX_ALLOWED_SEARCH_SPAN: i32 = 1024;

pub fn handle_convert(
    engine: &MusicEngine,
    format: OutputFormat,
    command: ConvertCommand,
) -> Result<()> {
    match command {
        ConvertCommand::Pitch { command } => match command {
            PitchConvertCommand::ToFrequency(args) => {
                convert_pitch_to_frequency(engine, format, args)
            }
            PitchConvertCommand::ToIndex(args) => convert_frequency_to_index(engine, format, args),
        },
        ConvertCommand::Midi { command } => match command {
            MidiConvertCommand::ToCsv(args) => convert_midi_to_csv(format, args),
            MidiConvertCommand::FromCsv(args) => convert_csv_to_midi(format, args),
        },
        ConvertCommand::Temperament(args) => remap_temperament(engine, format, args),
    }
}

fn convert_pitch_to_frequency(
    engine: &MusicEngine,
    format: OutputFormat,
    args: ConvertPitchArgs,
) -> Result<()> {
    let system = PitchSystemId::try_new(&args.system)
        .with_context(|| format!("invalid pitch system id '{}'", args.system))?;
    let pitch = Pitch::abstract_pitch(args.index, system.clone());
    let frequency_hz = pitch
        .try_freq_hz(engine.registry())
        .with_context(|| format!("failed to resolve {pitch}"))?;
    let label = pitch
        .try_label(engine.registry())
        .ok()
        .map(|label| label.to_string_lossy());

    let report = PitchIndexToFrequency {
        system: system.to_string(),
        index: args.index,
        frequency_hz,
        label,
    };

    format.emit(&report, PitchIndexToFrequency::render_text)
}

fn convert_frequency_to_index(
    engine: &MusicEngine,
    format: OutputFormat,
    args: ConvertFrequencyArgs,
) -> Result<()> {
    if !args.frequency_hz.is_finite() || args.frequency_hz <= 0.0 {
        bail!("--frequency must be a positive finite value in Hz");
    }

    let system = PitchSystemId::try_new(&args.system)
        .with_context(|| format!("invalid pitch system id '{}'", args.system))?;
    let (index, resolved_frequency_hz) = find_closest_index(
        engine.registry(),
        &system,
        args.frequency_hz,
        args.center,
        args.search_span,
    )?;

    let pitch = Pitch::abstract_pitch(index, system.clone());
    let label = pitch
        .try_label(engine.registry())
        .ok()
        .map(|label| label.to_string_lossy());
    let cents_error = cents_delta(args.frequency_hz, resolved_frequency_hz);

    let report = FrequencyToIndexReport {
        system: system.to_string(),
        input_frequency_hz: args.frequency_hz,
        resolved_index: index,
        resolved_frequency_hz,
        cents_error,
        search_center: args.center,
        search_span: args.search_span,
        label,
    };

    format.emit(&report, FrequencyToIndexReport::render_text)
}

fn remap_temperament(
    engine: &MusicEngine,
    format: OutputFormat,
    args: ConvertTemperamentArgs,
) -> Result<()> {
    if args.indices.is_empty() {
        bail!("provide at least one pitch index via --indices");
    }

    let from_system = PitchSystemId::try_new(&args.from_system)
        .with_context(|| format!("invalid pitch system id '{}'", args.from_system))?;
    let to_system = PitchSystemId::try_new(&args.to_system)
        .with_context(|| format!("invalid pitch system id '{}'", args.to_system))?;

    let registry = engine.registry();
    let mut mappings = Vec::with_capacity(args.indices.len());
    for &index in &args.indices {
        let freq = registry
            .resolve_frequency(&from_system, index)
            .with_context(|| format!("failed to resolve {index} in {from_system}"))?;
        let source_pitch = Pitch::abstract_pitch(index, from_system.clone());
        let source_label = source_pitch
            .try_label(registry)
            .ok()
            .map(|label| label.to_string_lossy());
        let (target_index, target_freq) =
            find_closest_index(registry, &to_system, freq, index, args.search_span)?;
        let target_pitch = Pitch::abstract_pitch(target_index, to_system.clone());
        let target_label = target_pitch
            .try_label(registry)
            .ok()
            .map(|label| label.to_string_lossy());
        let cents_offset = cents_delta(target_freq, freq);
        mappings.push(TemperamentMappingRow {
            source_index: index,
            source_label,
            source_frequency_hz: freq,
            target_index,
            target_label,
            target_frequency_hz: target_freq,
            cents_delta: cents_offset,
        });
    }

    let report = TemperamentRemapReport {
        from_system: from_system.to_string(),
        to_system: to_system.to_string(),
        mapping_count: mappings.len(),
        search_span: args.search_span,
        mappings,
    };

    format.emit(&report, TemperamentRemapReport::render_text)
}

fn find_closest_index(
    registry: &TuningRegistry,
    system: &PitchSystemId,
    target_freq: f32,
    center: i32,
    span: i32,
) -> Result<(i32, f32)> {
    if !target_freq.is_finite() || target_freq <= 0.0 {
        bail!("frequency must be positive and finite");
    }
    let clamped_span = span.clamp(1, MAX_ALLOWED_SEARCH_SPAN);
    let start = center - clamped_span;
    let end = center + clamped_span;

    let mut best_index = None;
    let mut best_diff = f32::MAX;
    let mut best_freq = None;

    for index in start..=end {
        let freq = registry
            .resolve_frequency(system, index)
            .with_context(|| format!("failed to resolve {index} in {system}"))?;
        let diff = (freq - target_freq).abs();
        if diff < best_diff {
            best_diff = diff;
            best_index = Some(index);
            best_freq = Some(freq);
        }
    }

    match (best_index, best_freq) {
        (Some(index), Some(freq)) => Ok((index, freq)),
        _ => bail!("no candidate pitches found during search"),
    }
}

fn convert_midi_to_csv(format: OutputFormat, args: ConvertMidiToCsvArgs) -> Result<()> {
    let path = &args.file;
    let data =
        fs::read(path).with_context(|| format!("failed to read MIDI file {}", path.display()))?;
    let smf = Smf::parse(&data).context("failed to parse MIDI file")?;
    let ticks_per_quarter = match smf.header.timing {
        Timing::Metrical(value) => Some(value.as_int()),
        Timing::Timecode(_, _) => None,
    };
    let mut rows = extract_midi_rows(&smf);
    rows.sort_by_key(|row| (row.track, row.start_tick, row.note));
    let mut emitted_rows = rows.clone();
    let mut truncated = false;
    if let Some(limit) = args.max_rows.filter(|limit| emitted_rows.len() > *limit) {
        emitted_rows.truncate(limit);
        truncated = true;
    }

    let report = MidiCsvConversionReport {
        direction: MidiConversionDirection::MidiToCsv,
        source: path.display().to_string(),
        destination: None,
        note_count: rows.len(),
        emitted_rows: emitted_rows.len(),
        truncated,
        ticks_per_quarter,
        rows: emitted_rows,
    };

    format.emit(&report, MidiCsvConversionReport::render_text)
}

fn convert_csv_to_midi(format: OutputFormat, args: ConvertCsvToMidiArgs) -> Result<()> {
    let rows = parse_csv_rows(&args.csv)?;
    if rows.is_empty() {
        bail!("CSV file does not contain any note rows");
    }

    let mut grouped = rows.clone();
    grouped.sort_by_key(|row| (row.track, row.start_tick, row.note));
    let smf = build_smf(&grouped, args.ticks_per_quarter);
    let mut buffer = Vec::new();
    smf.write(&mut buffer)
        .map_err(|err| anyhow!("failed to serialize MIDI data: {err}"))?;
    fs::write(&args.out, &buffer)
        .with_context(|| format!("failed to write {}", args.out.display()))?;

    let report = MidiCsvConversionReport {
        direction: MidiConversionDirection::CsvToMidi,
        source: args.csv.display().to_string(),
        destination: Some(args.out.display().to_string()),
        note_count: rows.len(),
        emitted_rows: rows.len(),
        truncated: false,
        ticks_per_quarter: Some(args.ticks_per_quarter),
        rows: rows.into_iter().take(16).collect(),
    };

    format.emit(&report, MidiCsvConversionReport::render_text)
}

fn extract_midi_rows(smf: &Smf<'_>) -> Vec<MidiCsvRow> {
    let mut rows = Vec::new();
    for (track_idx, track) in smf.tracks.iter().enumerate() {
        let mut time: u64 = 0;
        let mut pending: HashMap<(u8, u8), VecDeque<PendingNote>> = HashMap::new();
        for event in track {
            time += u64::from(event.delta.as_int());
            if let TrackEventKind::Midi { channel, message } = event.kind {
                match message {
                    MidiMessage::NoteOn { key, vel } if vel.as_int() > 0 => {
                        let entry = pending.entry((channel.as_int(), key.as_int())).or_default();
                        entry.push_back(PendingNote {
                            start: clamp_tick(time),
                            velocity: vel.as_int(),
                        });
                    }
                    MidiMessage::NoteOn { key, .. } | MidiMessage::NoteOff { key, .. } => {
                        close_pending_note(
                            &mut rows,
                            track_idx as u16,
                            channel.as_int(),
                            key.as_int(),
                            clamp_tick(time),
                            &mut pending,
                        );
                    }
                    _ => {}
                }
            }
        }

        let final_time = clamp_tick(time);
        for ((channel, note), mut queue) in pending {
            while let Some(active) = queue.pop_front() {
                rows.push(MidiCsvRow {
                    track: track_idx as u16,
                    channel,
                    note,
                    start_tick: active.start,
                    end_tick: final_time,
                    duration_tick: final_time.saturating_sub(active.start),
                    velocity: active.velocity,
                });
            }
        }
    }

    rows
}

fn close_pending_note(
    rows: &mut Vec<MidiCsvRow>,
    track: u16,
    channel: u8,
    note: u8,
    end_tick: u32,
    pending: &mut HashMap<(u8, u8), VecDeque<PendingNote>>,
) {
    if let Some(queue) = pending.get_mut(&(channel, note)) {
        if let Some(active) = queue.pop_front() {
            rows.push(MidiCsvRow {
                track,
                channel,
                note,
                start_tick: active.start,
                end_tick,
                duration_tick: end_tick.saturating_sub(active.start),
                velocity: active.velocity,
            });
        }
        if queue.is_empty() {
            pending.remove(&(channel, note));
        }
    }
}

fn parse_csv_rows(path: &Path) -> Result<Vec<MidiCsvRow>> {
    let contents = fs::read_to_string(path)
        .with_context(|| format!("failed to read CSV {}", path.display()))?;
    parse_csv_rows_str(&contents)
}

fn parse_csv_rows_str(contents: &str) -> Result<Vec<MidiCsvRow>> {
    let mut rows = Vec::new();
    for (line_number, line) in contents.lines().enumerate() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        if line_number == 0 && trimmed.to_ascii_lowercase().starts_with("track") {
            continue;
        }
        let columns: Vec<&str> = trimmed.split(',').map(|value| value.trim()).collect();
        if columns.len() < 6 {
            bail!(
                "line {}: expected at least 6 columns (track,channel,note,start_tick,end_tick,velocity)",
                line_number + 1
            );
        }

        let track = columns[0]
            .parse::<u16>()
            .with_context(|| format!("line {}: invalid track", line_number + 1))?;
        let channel = columns[1]
            .parse::<u8>()
            .with_context(|| format!("line {}: invalid channel", line_number + 1))?;
        let note = columns[2]
            .parse::<u8>()
            .with_context(|| format!("line {}: invalid note", line_number + 1))?;
        let start_tick = columns[3]
            .parse::<u32>()
            .with_context(|| format!("line {}: invalid start_tick", line_number + 1))?;
        let end_tick = columns[4]
            .parse::<u32>()
            .with_context(|| format!("line {}: invalid end_tick", line_number + 1))?;
        if end_tick < start_tick {
            bail!(
                "line {}: end_tick must be greater than or equal to start_tick",
                line_number + 1
            );
        }
        let velocity = columns[5]
            .parse::<u8>()
            .with_context(|| format!("line {}: invalid velocity", line_number + 1))?;
        rows.push(MidiCsvRow {
            track,
            channel,
            note,
            start_tick,
            end_tick,
            duration_tick: end_tick - start_tick,
            velocity,
        });
    }

    Ok(rows)
}

fn build_smf(rows: &[MidiCsvRow], ticks_per_quarter: u16) -> Smf<'static> {
    let mut track_map: BTreeMap<u16, Vec<TimedMidiEvent>> = BTreeMap::new();
    for row in rows {
        track_map.entry(row.track).or_default().extend_from_slice(&[
            TimedMidiEvent {
                tick: row.start_tick,
                channel: row.channel,
                note: row.note,
                velocity: row.velocity,
                kind: MidiEventKind::On,
            },
            TimedMidiEvent {
                tick: row.end_tick,
                channel: row.channel,
                note: row.note,
                velocity: 0,
                kind: MidiEventKind::Off,
            },
        ]);
    }

    let mut tracks = Vec::with_capacity(track_map.len());
    for (_track, mut events) in track_map {
        events.sort_by(|a, b| {
            a.tick
                .cmp(&b.tick)
                .then(a.kind.order().cmp(&b.kind.order()))
        });
        let mut prev_tick = 0u32;
        let mut track_events = Vec::new();
        for event in events {
            let delta = event.tick.saturating_sub(prev_tick);
            prev_tick = event.tick;
            let message = match event.kind {
                MidiEventKind::On => MidiMessage::NoteOn {
                    key: u7::from(event.note.min(127)),
                    vel: u7::from(event.velocity.min(127)),
                },
                MidiEventKind::Off => MidiMessage::NoteOff {
                    key: u7::from(event.note.min(127)),
                    vel: u7::from(0),
                },
            };
            track_events.push(TrackEvent {
                delta: u28::from(delta),
                kind: TrackEventKind::Midi {
                    channel: u4::from(event.channel.min(15)),
                    message,
                },
            });
        }
        track_events.push(TrackEvent {
            delta: u28::from(0),
            kind: TrackEventKind::Meta(MetaMessage::EndOfTrack),
        });
        tracks.push(track_events);
    }

    if tracks.is_empty() {
        tracks.push(vec![TrackEvent {
            delta: u28::from(0),
            kind: TrackEventKind::Meta(MetaMessage::EndOfTrack),
        }]);
    }

    Smf {
        header: Header {
            format: if tracks.len() > 1 {
                Format::Parallel
            } else {
                Format::SingleTrack
            },
            timing: Timing::Metrical(u15::from(ticks_per_quarter.max(1))),
        },
        tracks,
    }
}

fn cents_delta(a: f32, b: f32) -> f32 {
    1200.0 * (a / b).log2()
}

fn clamp_tick(value: u64) -> u32 {
    value.min(u32::MAX as u64) as u32
}

#[derive(Debug)]
struct PendingNote {
    start: u32,
    velocity: u8,
}

#[derive(Clone)]
struct TimedMidiEvent {
    tick: u32,
    channel: u8,
    note: u8,
    velocity: u8,
    kind: MidiEventKind,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum MidiEventKind {
    On,
    Off,
}

impl MidiEventKind {
    fn order(&self) -> u8 {
        match self {
            MidiEventKind::Off => 0,
            MidiEventKind::On => 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cents_delta_zero_when_equal() {
        assert!((cents_delta(440.0, 440.0)).abs() < 1.0e-6);
    }

    #[test]
    fn parse_csv_rows_str_handles_header_and_values() {
        let rows = parse_csv_rows_str(
            "track,channel,note,start_tick,end_tick,velocity\n0,1,60,0,480,100\n",
        )
        .unwrap();
        assert_eq!(rows.len(), 1);
        let row = &rows[0];
        assert_eq!(row.track, 0);
        assert_eq!(row.channel, 1);
        assert_eq!(row.note, 60);
        assert_eq!(row.duration_tick, 480);
    }

    #[test]
    fn build_smf_emits_parallel_format_when_multiple_tracks() {
        let rows = vec![
            MidiCsvRow {
                track: 0,
                channel: 0,
                note: 60,
                start_tick: 0,
                end_tick: 120,
                duration_tick: 120,
                velocity: 100,
            },
            MidiCsvRow {
                track: 1,
                channel: 1,
                note: 64,
                start_tick: 0,
                end_tick: 120,
                duration_tick: 120,
                velocity: 90,
            },
        ];
        let smf = build_smf(&rows, 480);
        assert_eq!(smf.tracks.len(), 2);
        assert_eq!(smf.header.format, Format::Parallel);
    }
}
