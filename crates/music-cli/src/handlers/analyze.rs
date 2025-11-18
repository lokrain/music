use std::{collections::HashSet, fs};

use anyhow::{Context, Result, bail};
use music_engine::prelude::*;

use crate::{
    cli::{AnalyzeChordsArgs, AnalyzeCommand, AnalyzeMelodyArgs, AnalyzeMidiArgs},
    format::OutputFormat,
    responses::{
        Ambitus, CadenceSummary, ChordAnalysisReport, FunctionCounts, KeyHypothesis,
        MelodyAnalysisReport, MidiAnalysisReport, PitchClassBin, TensionMetrics,
    },
};

pub(crate) const MAJOR_PATTERN: [u8; 7] = [0, 2, 4, 5, 7, 9, 11];
pub(crate) const MINOR_PATTERN: [u8; 7] = [0, 2, 3, 5, 7, 8, 10];

pub fn handle_analyze(
    engine: &MusicEngine,
    format: OutputFormat,
    command: AnalyzeCommand,
) -> Result<()> {
    match command {
        AnalyzeCommand::Melody(args) => analyze_melody(engine, format, args),
        AnalyzeCommand::Chords(args) => analyze_chords(format, args),
        AnalyzeCommand::Midi(args) => analyze_midi(format, &args),
    }
}

fn analyze_melody(
    engine: &MusicEngine,
    format: OutputFormat,
    args: AnalyzeMelodyArgs,
) -> Result<()> {
    if args.notes.is_empty() {
        bail!("provide at least one note via --notes");
    }

    let histogram = build_histogram(&args.notes);
    let total = args.notes.len();
    let system_id = PitchSystemId::from(args.system.clone());
    let key_estimate = infer_key(&histogram, total, args.key_hint.as_deref())?;
    let tonic_index = 60 + i32::from(key_estimate.root_pc);
    let tonic_label = describe_pitch_class(engine, &system_id, key_estimate.root_pc)?;

    let mut scale_members = HashSet::with_capacity(7);
    for member in key_estimate.scale {
        scale_members.insert(member);
    }

    let out_of_scale: usize = histogram
        .iter()
        .enumerate()
        .filter(|(pc, _)| !scale_members.contains(&(*pc as u8)))
        .map(|(_, count)| *count)
        .sum();
    let percent_out = (out_of_scale as f32 / total as f32) * 100.0;

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

    let report = MelodyAnalysisReport {
        note_count: total,
        distinct_pitch_classes: histogram.iter().filter(|count| **count > 0).count(),
        ambitus: Ambitus {
            lowest,
            highest,
            span: highest - lowest,
        },
        best_key: KeyHypothesis {
            tonic_label,
            tonic_pitch_class: key_estimate.root_pc,
            tonic_index,
            mode: key_estimate.mode.name().to_string(),
            match_ratio: key_estimate.match_ratio(),
            enforced: key_estimate.enforced,
        },
        histogram: histogram
            .iter()
            .enumerate()
            .filter(|(_, count)| **count > 0)
            .map(|(pc, count)| PitchClassBin {
                pitch_class: pc as u8,
                count: *count,
            })
            .collect(),
        tension: TensionMetrics {
            out_of_scale,
            percent_out_of_scale: percent_out,
        },
    };

    format.emit(&report, MelodyAnalysisReport::render_text)
}

fn analyze_chords(format: OutputFormat, args: AnalyzeChordsArgs) -> Result<()> {
    let progression: Vec<String> = args
        .progression
        .into_iter()
        .map(|token| token.trim().to_string())
        .filter(|token| !token.is_empty())
        .collect();

    if progression.is_empty() {
        bail!("provide at least one chord via --progression");
    }

    let functions = classify_functions(&progression);
    let cadence = detect_cadence(&progression);
    let unique_chords = progression
        .iter()
        .map(|token| token.to_ascii_uppercase())
        .collect::<HashSet<_>>()
        .len();

    let report = ChordAnalysisReport {
        progression,
        chord_count: functions.total,
        unique_chords,
        function_counts: functions.counts,
        cadence,
        key_hint: args.key_hint,
    };

    format.emit(&report, ChordAnalysisReport::render_text)
}

fn analyze_midi(format: OutputFormat, args: &AnalyzeMidiArgs) -> Result<()> {
    let path = &args.file;
    let metadata = fs::metadata(path)
        .with_context(|| format!("failed to read metadata for {}", path.display()))?;
    let bytes =
        fs::read(path).with_context(|| format!("failed to read MIDI file {}", path.display()))?;

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

    let report = MidiAnalysisReport {
        file: path.display().to_string(),
        size_bytes: metadata.len(),
        header_format,
        declared_tracks,
        detected_tracks,
        ticks_per_quarter,
        key_hint: args.key_hint.clone(),
        is_standard_midi,
    };

    format.emit(&report, MidiAnalysisReport::render_text)
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

fn infer_key(histogram: &[usize; 12], total: usize, key_hint: Option<&str>) -> Result<KeyEstimate> {
    if let Some(hint) = key_hint {
        let (root_pc, mode) = parse_key_hint(hint)?;
        return Ok(KeyEstimate::new(root_pc, mode, histogram, total, true));
    }

    let mut best: Option<KeyEstimate> = None;
    for root in 0..12u8 {
        for mode in [ModeFlavor::Major, ModeFlavor::Minor] {
            let candidate = KeyEstimate::new(root, mode, histogram, total, false);
            match &best {
                Some(current) if !candidate.is_better_than(current) => {}
                _ => best = Some(candidate),
            }
        }
    }

    if let Some(best) = best {
        Ok(best)
    } else {
        bail!("failed to infer key")
    }
}

pub(crate) fn parse_key_hint(input: &str) -> Result<(u8, ModeFlavor)> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        bail!("--in expects a key like Cmaj or Amin");
    }
    let lower = trimmed.to_ascii_lowercase();
    let mut chars = lower.chars();
    let first = match chars.next() {
        Some(ch) => ch,
        None => bail!("invalid key: {trimmed}"),
    };
    let mut root = match first {
        'c' => 0,
        'd' => 2,
        'e' => 4,
        'f' => 5,
        'g' => 7,
        'a' => 9,
        'b' => 11,
        _ => bail!("unable to parse tonic in '{trimmed}'"),
    };

    let mut remainder = chars.as_str();
    if remainder.starts_with('#') || remainder.starts_with('♯') {
        root = (root + 1) % 12;
        remainder = &remainder[1..];
    } else if remainder.starts_with('b') || remainder.starts_with('♭') {
        root = (root + 11) % 12;
        remainder = &remainder[1..];
    }

    remainder = remainder.trim_start_matches(['-', '_', ' ']);
    let remainder = remainder.trim();
    let looks_minor = (remainder.starts_with('m') && !remainder.starts_with("maj"))
        || remainder.contains("min")
        || remainder.contains("aeol")
        || (remainder.is_empty() && trimmed.ends_with('m'));
    let mode = if looks_minor {
        ModeFlavor::Minor
    } else {
        ModeFlavor::Major
    };

    Ok((root as u8, mode))
}

pub(crate) fn describe_pitch_class(
    engine: &MusicEngine,
    system: &PitchSystemId,
    pitch_class: u8,
) -> Result<String> {
    let base_index = 60 + i32::from(pitch_class);
    let pitch = Pitch::abstract_pitch(base_index, system.clone());
    Ok(engine
        .describe_pitch(&pitch)
        .unwrap_or_else(|_| format!("{system}({base_index})")))
}

fn classify_functions(tokens: &[String]) -> ClassifiedFunctions {
    let mut counts = FunctionCounts::default();
    for token in tokens {
        match classify_function(token) {
            FunctionRole::Tonic => counts.tonic += 1,
            FunctionRole::Predominant => counts.predominant += 1,
            FunctionRole::Dominant => counts.dominant += 1,
            FunctionRole::Other => counts.other += 1,
        }
    }

    ClassifiedFunctions {
        counts,
        total: tokens.len(),
    }
}

pub(crate) fn classify_function(token: &str) -> FunctionRole {
    let normalized = normalize_roman(token);
    match normalized.as_str() {
        "I" | "III" | "VI" => FunctionRole::Tonic,
        "II" | "IV" => FunctionRole::Predominant,
        "V" | "VII" => FunctionRole::Dominant,
        _ => FunctionRole::Other,
    }
}

fn normalize_roman(token: &str) -> String {
    let mut result = String::new();
    for ch in token.chars() {
        if matches!(ch, '#' | '♯' | 'b' | '♭' | '+' | '°' | 'ø') {
            continue;
        }
        if ch == '/' {
            break;
        }
        if ch.is_ascii_digit() {
            continue;
        }
        if ch.is_ascii_alphabetic() {
            result.push(ch.to_ascii_uppercase());
        }
    }
    if result.is_empty() {
        "UNK".to_string()
    } else {
        result
    }
}

fn detect_cadence(tokens: &[String]) -> Option<CadenceSummary> {
    if tokens.len() < 2 {
        return None;
    }
    let normalized: Vec<String> = tokens.iter().map(|t| normalize_roman(t)).collect();
    let last = normalized[normalized.len() - 1].as_str();
    let prev = normalized[normalized.len() - 2].as_str();

    if prev == "V" && last == "I" {
        return Some(CadenceSummary {
            pattern: "V–I".into(),
            confidence: 0.92,
            description: "Authentic cadence".into(),
        });
    }
    if prev == "IV" && last == "I" {
        return Some(CadenceSummary {
            pattern: "IV–I".into(),
            confidence: 0.8,
            description: "Plagal cadence".into(),
        });
    }
    if prev == "V" && last == "VI" {
        return Some(CadenceSummary {
            pattern: "V–VI".into(),
            confidence: 0.75,
            description: "Deceptive cadence".into(),
        });
    }
    if normalized.len() >= 3 {
        let third_last = normalized[normalized.len() - 3].as_str();
        if third_last == "II" && prev == "V" && last == "I" {
            return Some(CadenceSummary {
                pattern: "II–V–I".into(),
                confidence: 0.95,
                description: "Extended authentic cadence".into(),
            });
        }
    }

    None
}

struct ClassifiedFunctions {
    counts: FunctionCounts,
    total: usize,
}

#[derive(Clone, Copy, Debug)]
pub(crate) enum FunctionRole {
    Tonic,
    Predominant,
    Dominant,
    Other,
}

struct KeyEstimate {
    root_pc: u8,
    mode: ModeFlavor,
    matches: usize,
    total: usize,
    enforced: bool,
    scale: [u8; 7],
}

impl KeyEstimate {
    fn new(
        root_pc: u8,
        mode: ModeFlavor,
        histogram: &[usize; 12],
        total: usize,
        enforced: bool,
    ) -> Self {
        let scale = apply_pattern(root_pc, mode.pattern());
        let matches = scale.iter().map(|pc| histogram[*pc as usize]).sum();
        Self {
            root_pc,
            mode,
            matches,
            total,
            enforced,
            scale,
        }
    }

    fn match_ratio(&self) -> f32 {
        if self.total == 0 {
            0.0
        } else {
            self.matches as f32 / self.total as f32
        }
    }

    fn is_better_than(&self, other: &Self) -> bool {
        if self.matches == other.matches {
            self.mode.priority() > other.mode.priority()
                || (self.mode.priority() == other.mode.priority() && self.root_pc < other.root_pc)
        } else {
            self.matches > other.matches
        }
    }
}

#[derive(Clone, Copy)]
pub(crate) enum ModeFlavor {
    Major,
    Minor,
}

impl ModeFlavor {
    pub(crate) fn name(&self) -> &'static str {
        match self {
            Self::Major => "Major",
            Self::Minor => "Minor",
        }
    }

    pub(crate) fn pattern(&self) -> &'static [u8; 7] {
        match self {
            Self::Major => &MAJOR_PATTERN,
            Self::Minor => &MINOR_PATTERN,
        }
    }

    fn priority(&self) -> u8 {
        match self {
            Self::Major => 1,
            Self::Minor => 0,
        }
    }
}

pub(crate) fn apply_pattern(root_pc: u8, pattern: &[u8; 7]) -> [u8; 7] {
    let mut out = [0u8; 7];
    for (idx, step) in pattern.iter().enumerate() {
        out[idx] = (root_pc + step) % 12;
    }
    out
}
