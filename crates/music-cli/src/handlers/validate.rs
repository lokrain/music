use std::collections::HashSet;

use anyhow::{Context, Result, bail};
use music_engine::{
    MusicEngine,
    prelude::{Pitch, PitchSystemId},
};

use crate::{
    cli::{ValidateCommand, ValidateMelodyArgs, ValidateProgressionArgs, ValidateTuningArgs},
    format::OutputFormat,
    handlers::analyze::{FunctionRole, classify_function, detect_cadence, normalize_roman},
    reports::{
        analysis::Ambitus,
        validation::{
            InvalidMelodyNote, InvalidProgressionToken, LeapViolation, MelodyValidationReport,
            MonotonicViolation, ProgressionValidationReport, TuningSampleRow,
            TuningValidationReport,
        },
    },
    responses::FunctionCounts,
};

pub fn handle_validate(
    engine: &MusicEngine,
    format: OutputFormat,
    command: ValidateCommand,
) -> Result<()> {
    match command {
        ValidateCommand::Melody(args) => validate_melody(engine, format, args),
        ValidateCommand::Progression(args) => validate_progression(format, args),
        ValidateCommand::Tuning(args) => validate_tuning(engine, format, args),
    }
}

fn validate_melody(
    engine: &MusicEngine,
    format: OutputFormat,
    args: ValidateMelodyArgs,
) -> Result<()> {
    if args.notes.is_empty() {
        bail!("provide at least one note via --notes");
    }
    if args.max_interval <= 0 {
        bail!("--max-interval must be positive");
    }

    let system_id = PitchSystemId::try_new(&args.system)
        .with_context(|| format!("invalid pitch system id '{}'", args.system))?;
    if !engine.registry().contains(&system_id) {
        bail!("pitch system '{}' is not registered", args.system);
    }
    let allowed_offsets = args.scale.pitch_classes();
    let allowed_set: HashSet<u8> = allowed_offsets.iter().copied().collect();
    let root_pc = normalize_pitch_class(args.root);

    let mut out_of_scale = Vec::new();
    for (position, note) in args.notes.iter().enumerate() {
        let pc = normalize_pitch_class(*note);
        let relative = ((12 + pc as i32 - root_pc as i32) % 12) as u8;
        if !allowed_set.contains(&relative) {
            out_of_scale.push(InvalidMelodyNote {
                position,
                note_index: *note,
                pitch_class: pc,
            });
        }
    }

    let mut leap_violations = Vec::new();
    for (position, window) in args.notes.windows(2).enumerate() {
        let interval = window[1] - window[0];
        if interval.abs() > args.max_interval {
            leap_violations.push(LeapViolation {
                position,
                from: window[0],
                to: window[1],
                interval,
            });
        }
    }

    let lowest = *args
        .notes
        .iter()
        .min()
        .expect("non-empty notes already validated");
    let highest = *args
        .notes
        .iter()
        .max()
        .expect("non-empty notes already validated");

    let allowed_pitch_classes = allowed_offsets
        .iter()
        .map(|offset| ((*offset as i32 + root_pc as i32) % 12) as u8)
        .collect();

    let report = MelodyValidationReport {
        system: args.system,
        scale: args.scale.to_string(),
        root_index: args.root,
        note_count: args.notes.len(),
        ambitus: Ambitus {
            lowest,
            highest,
            span: highest - lowest,
        },
        max_interval: args.max_interval,
        allowed_pitch_classes,
        out_of_scale_notes: out_of_scale,
        leap_violations,
    };

    format.emit(&report, MelodyValidationReport::render_text)
}

fn validate_progression(format: OutputFormat, args: ValidateProgressionArgs) -> Result<()> {
    let progression: Vec<String> = args
        .progression
        .into_iter()
        .map(|token| token.trim().to_string())
        .filter(|token| !token.is_empty())
        .collect();

    if progression.is_empty() {
        bail!("provide at least one chord via --progression");
    }

    let normalized: Vec<String> = progression.iter().map(|t| normalize_roman(t)).collect();
    let mut invalid = Vec::new();
    for (idx, norm) in normalized.iter().enumerate() {
        if !is_valid_roman(norm) {
            invalid.push(InvalidProgressionToken {
                position: idx,
                token: progression[idx].clone(),
                normalized: norm.clone(),
            });
        }
    }

    let mut duplicates = Vec::new();
    for (idx, window) in normalized.windows(2).enumerate() {
        if window[0] == window[1] {
            duplicates.push(idx);
        }
    }

    let mut counts = FunctionCounts::default();
    for token in &progression {
        match classify_function(token) {
            FunctionRole::Tonic => counts.tonic += 1,
            FunctionRole::Predominant => counts.predominant += 1,
            FunctionRole::Dominant => counts.dominant += 1,
            FunctionRole::Other => counts.other += 1,
        }
    }

    let cadence = detect_cadence(&progression);

    let report = ProgressionValidationReport {
        progression,
        chord_count: normalized.len(),
        key_hint: args.key_hint,
        invalid_chords: invalid,
        duplicate_positions: duplicates,
        function_counts: counts,
        cadence,
    };

    format.emit(&report, ProgressionValidationReport::render_text)
}

fn validate_tuning(
    engine: &MusicEngine,
    format: OutputFormat,
    args: ValidateTuningArgs,
) -> Result<()> {
    let system_id = PitchSystemId::try_new(&args.system)
        .with_context(|| format!("invalid pitch system id '{}'", args.system))?;
    let registry = engine.registry();

    let mut resolved = Vec::new();
    let mut failed = Vec::new();
    for index in &args.indices {
        match registry.resolve_frequency(&system_id, *index) {
            Ok(freq) => {
                let label = Pitch::abstract_pitch(*index, system_id.clone())
                    .try_label(registry)
                    .ok()
                    .map(|label| label.to_string_lossy());
                resolved.push(TuningSampleRow {
                    index: *index,
                    frequency_hz: freq,
                    label,
                });
            }
            Err(_) => failed.push(*index),
        }
    }

    resolved.sort_by_key(|row| row.index);
    let mut monotonic = Vec::new();
    for window in resolved.windows(2) {
        if window[0].frequency_hz >= window[1].frequency_hz {
            monotonic.push(MonotonicViolation {
                lower_index: window[0].index,
                lower_frequency_hz: window[0].frequency_hz,
                higher_index: window[1].index,
                higher_frequency_hz: window[1].frequency_hz,
            });
        }
    }

    let report = TuningValidationReport {
        system: args.system,
        requested_indices: args.indices,
        resolved_samples: resolved,
        failed_indices: failed,
        monotonic_violations: monotonic,
    };

    format.emit(&report, TuningValidationReport::render_text)
}

fn normalize_pitch_class(value: i32) -> u8 {
    let mut pc = value % 12;
    if pc < 0 {
        pc += 12;
    }
    pc as u8
}

fn is_valid_roman(token: &str) -> bool {
    matches!(token, "I" | "II" | "III" | "IV" | "V" | "VI" | "VII")
}
