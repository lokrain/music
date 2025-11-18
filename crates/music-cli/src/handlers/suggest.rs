use anyhow::{Context, Result, bail};
use chord::{diatonic_sevenths, diatonic_triads};
use music_engine::prelude::*;

use crate::{
    cli::{ReharmArgs, SuggestCommand},
    format::OutputFormat,
    responses::{ModeReharm, ReharmListing, borrowed_chord},
    theory::ChordVoicing,
};

pub fn handle_suggest(
    engine: &MusicEngine,
    format: OutputFormat,
    command: SuggestCommand,
) -> Result<()> {
    match command {
        SuggestCommand::Reharm(args) => reharmonize(engine, format, args),
    }
}

fn reharmonize(engine: &MusicEngine, format: OutputFormat, args: ReharmArgs) -> Result<()> {
    let registry = engine.registry();
    let system_id = PitchSystemId::from(args.system.clone());
    let base_scale = args
        .scale
        .build_scale(args.root, &system_id, registry)
        .with_context(|| format!("failed to build {:?} scale", args.scale))?;
    let step_count = base_scale.step_count();
    if step_count == 0 {
        bail!(
            "scale {:?} has no steps; cannot build reharmonization table",
            args.scale
        );
    }

    if let Some(degree) = args
        .degree
        .filter(|degree| !(1..=step_count).contains(degree))
    {
        bail!(
            "degree must be between 1 and {step_count}, received {degree}",
            step_count = step_count,
            degree = degree
        );
    }

    let target_pitch = if let Some(degree) = args.degree {
        Some(
            base_scale
                .degree_pitch(degree - 1, registry)
                .with_context(|| {
                    format!("failed to resolve degree {degree} for {:?}", args.scale)
                })?,
        )
    } else {
        None
    };

    let target_index = target_pitch.as_ref().and_then(|pitch| pitch.index());
    let target_label = target_pitch.as_ref().and_then(|pitch| {
        pitch
            .try_label(registry)
            .ok()
            .map(|label| label.to_string_lossy())
    });
    let target_frequency_hz = target_pitch
        .as_ref()
        .and_then(|pitch| pitch.try_freq_hz(registry).ok());

    let mut modes = Vec::with_capacity(step_count);
    for rotation in 0..step_count {
        let mode_scale = base_scale.mode(rotation, registry).with_context(|| {
            format!(
                "failed to rotate {:?} scale by {rotation} degrees",
                args.scale
            )
        })?;
        let chords = match args.voicing {
            ChordVoicing::Triads => diatonic_triads(&mode_scale, registry)?,
            ChordVoicing::Sevenths => diatonic_sevenths(&mode_scale, registry)?,
        };

        let mut borrowed = Vec::with_capacity(chords.len());
        for chord in chords {
            let chord_root_index = chord.chord.root().index();
            if target_index
                .map(|target| chord_root_index != Some(target))
                .unwrap_or(false)
            {
                continue;
            }
            borrowed.push(borrowed_chord(chord, registry, target_index)?);
        }

        let root_pitch = mode_scale.root().clone();
        let root_label = root_pitch
            .try_label(registry)
            .map(|label| label.to_string_lossy())
            .ok();

        modes.push(ModeReharm {
            mode_index: rotation + 1,
            rotation_degree: rotation,
            mode_name: args
                .scale
                .mode_name_for_rotation(rotation)
                .map(str::to_string),
            root_index: root_pitch.index(),
            root_label,
            borrowed_chords: borrowed,
        });
    }

    let listing = ReharmListing {
        system: args.system,
        root_index: args.root,
        scale: format!("{:?}", args.scale),
        voicing: format!("{:?}", args.voicing),
        target_degree: args.degree,
        target_label,
        target_frequency_hz,
        mode_count: modes.len(),
        modes,
    };

    format.emit(&listing, ReharmListing::render_text)
}
