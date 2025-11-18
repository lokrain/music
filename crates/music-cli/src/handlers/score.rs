use std::collections::HashSet;

use anyhow::{Result, bail};
use music_engine::prelude::*;

use crate::{
    cli::{ScoreChordArgs, ScoreCommand, ScoreMelodyArgs, ScoreProgressionArgs},
    format::OutputFormat,
    responses::{
        Ambitus, ChordScoreReport, FunctionCounts, MelodyScoreReport, ProgressionScoreReport,
    },
};

use super::analyze::{FunctionRole, classify_function, detect_cadence};

pub fn handle_score(
    engine: &MusicEngine,
    format: OutputFormat,
    command: ScoreCommand,
) -> Result<()> {
    match command {
        ScoreCommand::Progression(args) => score_progression(format, args),
        ScoreCommand::Melody(args) => score_melody(engine, format, args),
        ScoreCommand::Chord(args) => score_chord(engine, format, args),
    }
}

fn score_progression(format: OutputFormat, args: ScoreProgressionArgs) -> Result<()> {
    let progression: Vec<String> = args
        .progression
        .into_iter()
        .map(|token| token.trim().to_string())
        .filter(|token| !token.is_empty())
        .collect();
    if progression.is_empty() {
        bail!("provide at least one chord via --progression");
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

    let total = progression.len();
    let unique_chords = progression
        .iter()
        .map(|token| token.to_ascii_uppercase())
        .collect::<HashSet<_>>()
        .len();
    let cadence = detect_cadence(&progression);
    let coverage_hits = [
        counts.tonic > 0,
        counts.predominant > 0,
        counts.dominant > 0,
    ]
    .into_iter()
    .filter(|hit| *hit)
    .count();
    let coverage_score = (coverage_hits as f32 / 3.0).clamp(0.0, 1.0);
    let cadence_score = cadence.as_ref().map(|c| c.confidence).unwrap_or(0.0);
    let variety_score = if total == 0 {
        0.0
    } else {
        (unique_chords as f32 / total as f32).clamp(0.0, 1.0)
    };
    let total_score = (coverage_score * 0.45 + cadence_score * 0.35 + variety_score * 0.2) * 100.0;

    let mut commentary = Vec::new();
    if coverage_hits == 3 {
        commentary.push("Balanced tonic→predominant→dominant flow detected".into());
    } else if coverage_hits == 2 {
        commentary.push(
            "Progression touches two primary functions; consider completing the cycle".into(),
        );
    } else {
        commentary.push("Functional coverage is narrow; add supporting chords for motion".into());
    }
    match cadence {
        Some(ref cad) if cad.confidence > 0.85 => {
            commentary.push(format!(
                "Cadence {pattern} lands convincingly",
                pattern = cad.pattern
            ));
        }
        Some(_) => commentary.push("Cadence present but relatively soft".into()),
        None => commentary.push("No terminal cadence detected".into()),
    }
    if variety_score < 0.5 {
        commentary.push("Consider introducing more harmonic variety".into());
    }

    let report = ProgressionScoreReport {
        progression,
        total_chords: total,
        unique_chords,
        function_counts: counts,
        cadence,
        coverage_score,
        cadence_score,
        variety_score,
        total_score,
        commentary,
        key_hint: args.key_hint,
    };

    format.emit(&report, ProgressionScoreReport::render_text)
}

fn score_melody(_engine: &MusicEngine, format: OutputFormat, args: ScoreMelodyArgs) -> Result<()> {
    if args.notes.is_empty() {
        bail!("provide at least one note via --notes");
    }
    let notes = args.notes;
    let mut sorted = notes.clone();
    sorted.sort_unstable();
    let lowest = *sorted.first().unwrap();
    let highest = *sorted.last().unwrap();
    let span = highest - lowest;
    let ambitus = Ambitus {
        lowest,
        highest,
        span,
    };
    let interval_count = notes.len().saturating_sub(1);
    let mut leaps = 0usize;
    let mut steps = 0usize;
    let mut direction_changes = 0usize;
    let mut prev_dir = 0i32;
    for window in notes.windows(2) {
        let diff = window[1] - window[0];
        if (diff > 0 && prev_dir < 0) || (diff < 0 && prev_dir > 0) {
            direction_changes += 1;
        }
        if diff != 0 {
            prev_dir = diff.signum();
        }
        let dist = diff.abs();
        if dist <= 2 {
            steps += 1;
        }
        if dist > 7 {
            leaps += 1;
        }
    }

    let leap_ratio = if interval_count == 0 {
        0.0
    } else {
        leaps as f32 / interval_count as f32
    };
    let stepwise_ratio = if interval_count == 0 {
        1.0
    } else {
        steps as f32 / interval_count as f32
    };
    let contour_score = if interval_count <= 1 {
        0.5
    } else {
        (direction_changes as f32 / (interval_count - 1) as f32).clamp(0.0, 1.0)
    };
    let range_score = if span <= 12 {
        1.0
    } else if span <= 19 {
        0.75
    } else {
        0.5
    };
    let motion_score = (stepwise_ratio * 0.7 + (1.0 - leap_ratio) * 0.3).clamp(0.0, 1.0);
    let closure_interval = notes.last().unwrap() - notes.first().unwrap();
    let closure_score = if closure_interval.abs() <= 2 {
        1.0
    } else if closure_interval.abs() <= 5 {
        0.7
    } else {
        0.4
    };
    let total_score =
        (range_score * 0.3 + motion_score * 0.4 + closure_score * 0.2 + contour_score * 0.1)
            * 100.0;

    let mut commentary = Vec::new();
    if leap_ratio < 0.2 {
        commentary.push("Mostly conjunct motion keeps tension low".into());
    } else if leap_ratio > 0.4 {
        commentary.push("Frequent leaps add excitement but increase tension".into());
    }
    if span > 19 {
        commentary.push("Wide range — consider narrowing for tighter focus".into());
    } else if span < 7 {
        commentary.push("Compact range; add larger gestures for contrast".into());
    }
    if closure_interval.abs() <= 2 {
        commentary.push("Closing near the opening pitch provides resolution".into());
    } else {
        commentary.push("Ending away from the opening pitch leaves tension unresolved".into());
    }

    let report = MelodyScoreReport {
        note_count: notes.len(),
        ambitus,
        leap_ratio,
        stepwise_ratio,
        direction_changes,
        closure_interval,
        range_score,
        motion_score,
        contour_score,
        total_score,
        commentary,
        key_hint: args.key_hint,
    };

    format.emit(&report, MelodyScoreReport::render_text)
}

fn score_chord(engine: &MusicEngine, format: OutputFormat, args: ScoreChordArgs) -> Result<()> {
    if args.notes.is_empty() {
        bail!("provide at least one note via --notes");
    }
    let mut notes = args.notes;
    notes.sort_unstable();
    let pitch_span = notes.last().unwrap() - notes.first().unwrap();
    let unique_pitch_classes = notes
        .iter()
        .map(|note| note.rem_euclid(12))
        .collect::<HashSet<_>>()
        .len();
    let extensions = notes
        .iter()
        .map(|note| note - notes[0])
        .filter(|interval| *interval >= 12)
        .count();

    let mut tension: f32 = 0.0;
    if has_interval_mod(&notes, 6) {
        tension += 0.5;
    }
    if has_interval_mod(&notes, 1) || has_interval_mod(&notes, 11) {
        tension += 0.3;
    }
    if has_interval_mod(&notes, 2) {
        tension += 0.2;
    }
    let tension_index = tension.clamp(0.0, 1.0);
    let mut color_score = (unique_pitch_classes as f32 / 5.0).min(1.0);
    color_score += (extensions as f32).min(2.0) * 0.1;
    color_score = color_score.min(1.0);
    let spread_score = if pitch_span <= 12 {
        0.65
    } else if pitch_span <= 24 {
        0.85
    } else {
        0.95
    };
    let stability_score = (((1.0f32 - tension_index) * 0.7) + spread_score * 0.3).clamp(0.0, 1.0);
    let total_score = (color_score * 0.5 + stability_score * 0.5) * 100.0;

    let mut commentary = Vec::new();
    if extensions > 0 {
        commentary.push(format!(
            "Includes {extensions} upper extension(s) for added color"
        ));
    }
    if tension_index > 0.6 {
        commentary.push("Strong dissonance present — resolve or sustain intentionally".into());
    } else if tension_index < 0.2 {
        commentary.push("Stable sonority suitable for cadential use".into());
    }
    if pitch_span > 24 {
        commentary.push("Very wide voicing; ensure ensemble can cover the span".into());
    }

    let system_id = PitchSystemId::from(args.system.clone());
    let note_labels = notes
        .iter()
        .map(|index| describe_pitch(engine, *index, &system_id))
        .collect::<Vec<_>>();

    let report = ChordScoreReport {
        system: args.system,
        note_count: notes.len(),
        pitch_span,
        unique_pitch_classes,
        extensions,
        color_score,
        stability_score,
        tension_index,
        total_score,
        note_labels,
        commentary,
    };

    format.emit(&report, ChordScoreReport::render_text)
}

fn has_interval_mod(notes: &[i32], target_mod: i32) -> bool {
    for (idx, &a) in notes.iter().enumerate() {
        for &b in &notes[idx + 1..] {
            let diff = (b - a).abs() % 12;
            if diff == target_mod {
                return true;
            }
        }
    }
    false
}

fn describe_pitch(engine: &MusicEngine, index: i32, system: &PitchSystemId) -> String {
    let pitch = Pitch::abstract_pitch(index, system.clone());
    engine
        .describe_pitch(&pitch)
        .unwrap_or_else(|_| format!("{system}({index})"))
}
