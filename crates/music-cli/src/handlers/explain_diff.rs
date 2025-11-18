use std::{collections::HashSet, fs, path::Path};

use anyhow::{Context, Result, bail};
use music_engine::prelude::*;

use crate::{
    cli::{
        ExplainDiffCommand, ExplainDiffMelodyArgs, ExplainDiffMidiArgs,
        ExplainDiffProgressionArgs,
    },
    format::OutputFormat,
    responses::{
        Ambitus, CadenceSummary, FunctionCounts, MelodyDiffReport, MelodyProfileSummary,
        MidiDiffReport, MidiFileSummary, PitchClassBin, ProgressionDiffReport,
        ProgressionProfileSummary,
    },
};

use super::analyze::{classify_function, detect_cadence, normalize_roman, FunctionRole};

pub fn handle_explain_diff(
    engine: &MusicEngine,
    format: OutputFormat,
    command: ExplainDiffCommand,
) -> Result<()> {
    match command {
        ExplainDiffCommand::Melody(args) => explain_diff_melody(format, args),
        ExplainDiffCommand::Progression(args) => explain_diff_progression(format, args),
        ExplainDiffCommand::Midi(args) => explain_diff_midi(format, args),
    }
}

fn explain_diff_melody(format: OutputFormat, args: ExplainDiffMelodyArgs) -> Result<()> {
    if args.left_notes.is_empty() || args.right_notes.is_empty() {
        bail!("provide --left-notes and --right-notes with at least one entry each");
    }
    let left_profile = build_melody_profile(&args.left_notes);
    let right_profile = build_melody_profile(&args.right_notes);

    let shared = intersect_pitch_classes(&left_profile.histogram, &right_profile.histogram);
    let left_only = diff_pitch_classes(&left_profile.histogram, &right_profile.histogram);
    let right_only = diff_pitch_classes(&right_profile.histogram, &left_profile.histogram);
    let distance = histogram_distance(&left_profile.histogram, &right_profile.histogram);

    let mut commentary = Vec::new();
    if distance < 0.2 {
        commentary.push("Histograms are closely aligned".into());
    } else if distance > 0.5 {
        commentary.push("Melodies have markedly different pitch distributions".into());
    }
    if left_profile.ambitus.span != right_profile.ambitus.span {
        commentary.push(format!(
            "Ambitus differs: left {left} st, right {right} st",
            left = left_profile.ambitus.span,
            right = right_profile.ambitus.span
        ));
    }
    if shared.is_empty() {
        commentary.push("No shared pitch classes — likely contrasting keys".into());
    }

    let report = MelodyDiffReport {
        system: args.system,
        key_hint: args.key_hint,
        left: left_profile.into_summary(),
        right: right_profile.into_summary(),
        shared_pitch_classes: shared,
        left_only_pitch_classes: left_only,
        right_only_pitch_classes: right_only,
        histogram_distance: distance,
        commentary,
    };

    format.emit(&report, MelodyDiffReport::render_text)
}

fn explain_diff_progression(
    format: OutputFormat,
    args: ExplainDiffProgressionArgs,
) -> Result<()> {
    if args.left.is_empty() || args.right.is_empty() {
        bail!("provide --left and --right progressions with at least one chord each");
    }

    let left = sanitize_progression(&args.left);
    let right = sanitize_progression(&args.right);

    let left_profile = build_progression_profile(&left);
    let right_profile = build_progression_profile(&right);

    let (shared, left_only, right_only) = compare_progressions(&left, &right);
    let mut commentary = Vec::new();
    if left_profile.function_counts.dominant > right_profile.function_counts.dominant {
        commentary.push("Left progression leans more on dominant function".into());
    } else if right_profile.function_counts.dominant > left_profile.function_counts.dominant {
        commentary.push("Right progression emphasizes dominant function more".into());
    }
    match (&left_profile.cadence, &right_profile.cadence) {
        (Some(left_cad), Some(right_cad)) if left_cad.pattern != right_cad.pattern => {
            commentary.push(format!(
                "Cadence shift: left {left} vs right {right}",
                left = left_cad.pattern,
                right = right_cad.pattern
            ));
        }
        (Some(_), None) => commentary.push("Left progression cadences while right does not".into()),
        (None, Some(_)) => commentary.push("Right progression cadences while left does not".into()),
        _ => {}
    }
    if shared.is_empty() {
        commentary.push("No shared chords — sequences are fully distinct".into());
    }

    let report = ProgressionDiffReport {
        key_hint: args.key_hint,
        left: left_profile,
        right: right_profile,
        shared_chords: shared,
        left_unique: left_only,
        right_unique: right_only,
        commentary,
    };

    format.emit(&report, ProgressionDiffReport::render_text)
}

fn explain_diff_midi(format: OutputFormat, args: ExplainDiffMidiArgs) -> Result<()> {
    let left = midi_summary(args.left_file.as_path())?;
    let right = midi_summary(args.right_file.as_path())?;
    let size_delta = left.size_bytes as i64 - right.size_bytes as i64;
    let track_delta = left.detected_tracks as i32 - right.detected_tracks as i32;

    let mut commentary = Vec::new();
    if left.header_format != right.header_format {
        commentary.push("Header format differs".into());
    }
    if left.ticks_per_quarter != right.ticks_per_quarter {
        commentary.push("Ticks-per-quarter resolution differs".into());
    }
    if left.is_standard_midi != right.is_standard_midi {
        commentary.push("One file advertises a Standard MIDI header, the other does not".into());
    }

    let report = MidiDiffReport {
        left,
        right,
        size_delta,
        track_delta,
        commentary,
    };

    format.emit(&report, MidiDiffReport::render_text)
}

struct MelodyProfile {
    note_count: usize,
    histogram: [usize; 12],
    ambitus: Ambitus,
}

impl MelodyProfile {
    fn into_summary(self) -> MelodyProfileSummary {
        MelodyProfileSummary {
            note_count: self.note_count,
            distinct_pitch_classes: self
                .histogram
                .iter()
                .filter(|count| **count > 0)
                .count(),
            ambitus: self.ambitus,
            histogram: histogram_bins(&self.histogram),
        }
    }
}

fn build_melody_profile(notes: &[i32]) -> MelodyProfile {
    let histogram = build_histogram(notes);
    let (lowest, highest) = melody_range(notes);
    MelodyProfile {
        note_count: notes.len(),
        histogram,
        ambitus: Ambitus {
            lowest,
            highest,
            span: highest - lowest,
        },
    }
}

fn melody_range(notes: &[i32]) -> (i32, i32) {
    let mut lowest = i32::MAX;
    let mut highest = i32::MIN;
    for note in notes {
        lowest = lowest.min(*note);
        highest = highest.max(*note);
    }
    (lowest, highest)
}

fn build_histogram(notes: &[i32]) -> [usize; 12] {
    let mut histogram = [0usize; 12];
    for note in notes {
        let mut pc = note % 12;
        if pc < 0 {
            pc += 12;
        }
        histogram[pc as usize] += 1;
    }
    histogram
}

fn histogram_bins(histogram: &[usize; 12]) -> Vec<PitchClassBin> {
    histogram
        .iter()
        .enumerate()
        .filter(|(_, count)| **count > 0)
        .map(|(pc, count)| PitchClassBin {
            pitch_class: pc as u8,
            count: *count,
        })
        .collect()
}

fn intersect_pitch_classes(left: &[usize; 12], right: &[usize; 12]) -> Vec<u8> {
    let mut pcs = Vec::new();
    for pc in 0..12 {
        if left[pc] > 0 && right[pc] > 0 {
            pcs.push(pc as u8);
        }
    }
    pcs
}

fn diff_pitch_classes(left: &[usize; 12], right: &[usize; 12]) -> Vec<u8> {
    let mut pcs = Vec::new();
    for pc in 0..12 {
        if left[pc] > 0 && right[pc] == 0 {
            pcs.push(pc as u8);
        }
    }
    pcs
}

fn histogram_distance(left: &[usize; 12], right: &[usize; 12]) -> f32 {
    let mut total = 0usize;
    let mut delta = 0usize;
    for pc in 0..12 {
        total += left[pc] + right[pc];
        delta += left[pc].abs_diff(right[pc]);
    }
    if total == 0 {
        0.0
    } else {
        delta as f32 / total as f32
    }
}

fn sanitize_progression(tokens: &[String]) -> Vec<String> {
    tokens
        .iter()
        .map(|token| token.trim().to_string())
        .filter(|token| !token.is_empty())
        .collect()
}

fn build_progression_profile(tokens: &[String]) -> ProgressionProfileSummary {
    let mut counts = FunctionCounts::default();
    for token in tokens {
        match classify_function(token) {
            FunctionRole::Tonic => counts.tonic += 1,
            FunctionRole::Predominant => counts.predominant += 1,
            FunctionRole::Dominant => counts.dominant += 1,
            FunctionRole::Other => counts.other += 1,
        }
    }
    let chord_count = tokens.len();
    let unique_chords = tokens
        .iter()
        .map(|token| token.to_ascii_uppercase())
        .collect::<HashSet<_>>()
        .len();

    ProgressionProfileSummary {
        progression: tokens.to_vec(),
        chord_count,
        unique_chords,
        function_counts: counts,
        cadence: detect_cadence(tokens),
    }
}

fn compare_progressions(
    left: &[String],
    right: &[String],
) -> (Vec<String>, Vec<String>, Vec<String>) {
    let left_set: HashSet<String> = left.iter().map(|t| normalize_roman(t)).collect();
    let right_set: HashSet<String> = right.iter().map(|t| normalize_roman(t)).collect();

    let mut shared: Vec<String> = left_set
        .intersection(&right_set)
        .cloned()
        .collect();
    shared.sort();
    let mut left_only: Vec<String> = left_set.difference(&right_set).cloned().collect();
    left_only.sort();
    let mut right_only: Vec<String> = right_set.difference(&left_set).cloned().collect();
    right_only.sort();

    (shared, left_only, right_only)
}

fn midi_summary(path: &Path) -> Result<MidiFileSummary> {
    let metadata = fs::metadata(path)
        .with_context(|| format!("failed to read metadata for {}", path.display()))?;
    let bytes = fs::read(path)
        .with_context(|| format!("failed to read MIDI file {}", path.display()))?;
    let (is_standard_midi, header_format, declared_tracks, ticks_per_quarter) =
        if bytes.len() >= 14 && &bytes[0..4] == b"MThd" {
            (
                true,
                Some(u16::from_be_bytes([bytes[8], bytes[9]])),
                Some(u16::from_be_bytes([bytes[10], bytes[11]])),
                Some(u16::from_be_bytes([bytes[12], bytes[13]])),
            )
        } else {
            (false, None, None, None)
        };
    let detected_tracks = bytes.windows(4).filter(|chunk| *chunk == b"MTrk").count();

    Ok(MidiFileSummary {
        file: path.display().to_string(),
        size_bytes: metadata.len(),
        header_format,
        declared_tracks,
        detected_tracks,
        ticks_per_quarter,
        is_standard_midi,
    })
}
