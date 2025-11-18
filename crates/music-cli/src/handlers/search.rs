use std::collections::HashSet;

use anyhow::{Context, Result, anyhow, bail};
use chord::{DiatonicChord, diatonic_sevenths, diatonic_triads};
use music_engine::prelude::*;

use crate::{
    cli::{SearchChordArgs, SearchCommand, SearchScaleArgs},
    format::OutputFormat,
    responses::{
        ChordSearchMatch, ChordSearchReport, ScaleSearchMatch, ScaleSearchReport, summarize_chord,
    },
    theory::{ChordVoicing, ScaleKind},
};

pub fn handle_search(
    engine: &MusicEngine,
    format: OutputFormat,
    command: SearchCommand,
) -> Result<()> {
    match command {
        SearchCommand::Scale(args) => search_scales(engine, format, args),
        SearchCommand::Chord(args) => search_chords(engine, format, args),
    }
}

fn search_scales(engine: &MusicEngine, format: OutputFormat, args: SearchScaleArgs) -> Result<()> {
    if args.notes.is_empty() {
        bail!("provide at least one note via --notes");
    }
    let targets = normalize_pitch_classes(&args.notes);
    let mut matches = Vec::new();

    'roots: for root_pc in 0..12u8 {
        for kind in ScaleKind::all() {
            let pattern = shifted_pattern(kind.pitch_classes(), root_pc);
            if targets.iter().all(|pc| pattern.contains(pc)) {
                let index = 60 + i32::from(root_pc);
                let pitch = Pitch::abstract_pitch(index, PitchSystemId::from(args.system.clone()));
                let label = engine.describe_pitch(&pitch).unwrap_or_else(|_| {
                    format!("{system}({index})", system = args.system, index = index)
                });
                matches.push(ScaleSearchMatch {
                    scale: format!("{:?}", kind),
                    root_index: index,
                    root_label: label,
                });
                if matches.len() >= args.limit {
                    break 'roots;
                }
            }
        }
    }

    let report = ScaleSearchReport {
        system: args.system,
        criteria: targets,
        match_count: matches.len(),
        matches,
    };

    format.emit(&report, ScaleSearchReport::render_text)
}

fn search_chords(engine: &MusicEngine, format: OutputFormat, args: SearchChordArgs) -> Result<()> {
    if args.notes.is_empty() {
        bail!("provide at least one note via --notes");
    }
    let targets = normalize_pitch_classes(&args.notes);
    let registry = engine.registry();
    let mut matches = Vec::new();

    'roots: for root_pc in 0..12u8 {
        let root_index = 60 + i32::from(root_pc);
        let system_id = PitchSystemId::from(args.system.clone());
        for kind in ScaleKind::all() {
            let scale = kind
                .build_scale(root_index, &system_id, registry)
                .with_context(|| format!("failed to build {:?} scale", kind))?;
            let chords = match args.voicing {
                ChordVoicing::Triads => diatonic_triads(&scale, registry)?,
                ChordVoicing::Sevenths => diatonic_sevenths(&scale, registry)?,
            };
            for chord in chords {
                let pcs = chord_pitch_classes(&chord, registry)?;
                if matches_pitch_classes(&pcs, &targets) {
                    let summary = summarize_chord(chord.clone(), registry)?;
                    let root = chord.chord.root().clone();
                    let label = root
                        .try_label(registry)
                        .map(|value| value.to_string_lossy())
                        .unwrap_or_else(|_| {
                            format!(
                                "{system}({index})",
                                system = args.system,
                                index = root.index().unwrap_or(root_index)
                            )
                        });
                    matches.push(ChordSearchMatch {
                        scale: format!("{:?}", kind),
                        degree: summary.degree + 1,
                        numeral: summary.numeral.clone(),
                        root_label: label,
                        pitch_classes: pcs.clone(),
                    });
                    if matches.len() >= args.limit {
                        break 'roots;
                    }
                }
            }
        }
    }

    let report = ChordSearchReport {
        system: args.system,
        criteria: targets,
        match_count: matches.len(),
        voicing: format!("{:?}", args.voicing),
        matches,
    };

    format.emit(&report, ChordSearchReport::render_text)
}

fn normalize_pitch_classes(notes: &[i32]) -> Vec<u8> {
    let mut seen = HashSet::new();
    let mut order = Vec::with_capacity(notes.len());
    for note in notes {
        let mut pc = note % 12;
        if pc < 0 {
            pc += 12;
        }
        let pc = pc as u8;
        if seen.insert(pc) {
            order.push(pc);
        }
    }
    order
}

fn shifted_pattern(pattern: &[u8; 7], root_pc: u8) -> Vec<u8> {
    pattern.iter().map(|step| (root_pc + step) % 12).collect()
}

fn chord_pitch_classes(chord: &DiatonicChord, registry: &TuningRegistry) -> Result<Vec<u8>> {
    let tones = chord.chord.tones(registry)?;
    let mut pcs = Vec::with_capacity(tones.len());
    for pitch in tones {
        let idx = pitch
            .index()
            .ok_or_else(|| anyhow!("chord pitch lacks index"))?;
        let pc = ((idx % 12) + 12) % 12;
        pcs.push(pc as u8);
    }
    pcs.sort();
    pcs.dedup();
    Ok(pcs)
}

fn matches_pitch_classes(chord_pcs: &[u8], targets: &[u8]) -> bool {
    targets.iter().all(|pc| chord_pcs.contains(pc))
}
