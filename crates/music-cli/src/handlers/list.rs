use anyhow::{Context, Result, bail};
use chord::{diatonic_sevenths, diatonic_triads};
use music_engine::prelude::*;

use crate::{
    cli::{ChordArgs, ExposeCommand, ListCommand, ModesArgs, SystemsArgs},
    format::OutputFormat,
    responses::{
        ChordListing, ModeListing, ModeSummary, SystemSummary, SystemsListing,
        collect_mode_pitches, summarize_chord,
    },
};

pub fn handle_list(engine: &MusicEngine, format: OutputFormat, command: ListCommand) -> Result<()> {
    match command {
        ListCommand::Systems(args) => list_systems(engine, format, args),
        ListCommand::Chords(args) => list_chords(engine, format, args),
        ListCommand::Modes(args) => list_modes(engine, format, args),
    }
}

pub fn handle_expose(
    engine: &MusicEngine,
    format: OutputFormat,
    command: ExposeCommand,
) -> Result<()> {
    match command {
        ExposeCommand::Tunings(args) => list_systems(engine, format, args),
        ExposeCommand::Modes(args) => list_modes(engine, format, args),
    }
}

fn list_systems(engine: &MusicEngine, format: OutputFormat, args: SystemsArgs) -> Result<()> {
    let reference_index = args.reference_index;
    let registry = engine.registry();
    let mut systems = Vec::with_capacity(registry.len());
    for id in registry.ids() {
        let freq = registry
            .resolve_frequency(id, reference_index)
            .with_context(|| format!("failed to resolve frequency for system {id}"))?;
        let label = registry.resolve_name(id, reference_index).ok().flatten();
        systems.push(SystemSummary {
            id: id.to_string(),
            reference_index,
            frequency_hz: freq,
            label,
        });
    }

    let listing = SystemsListing {
        reference_index,
        systems,
    };

    format.emit(&listing, SystemsListing::render_text)
}

fn list_chords(engine: &MusicEngine, format: OutputFormat, args: ChordArgs) -> Result<()> {
    let registry = engine.registry();
    let system_id = PitchSystemId::from(args.system.clone());
    let scale = args
        .scale
        .build_scale(args.root, &system_id, registry)
        .with_context(|| format!("failed to build {:?} scale", args.scale))?;

    let chords = match args.voicing {
        crate::theory::ChordVoicing::Triads => diatonic_triads(&scale, registry)?,
        crate::theory::ChordVoicing::Sevenths => diatonic_sevenths(&scale, registry)?,
    };

    let root_label = scale
        .root()
        .try_label(registry)
        .map(|label| label.to_string_lossy())
        .ok();

    let mut chord_summaries = Vec::with_capacity(chords.len());
    for chord in chords {
        chord_summaries.push(summarize_chord(chord, registry)?);
    }

    let listing = ChordListing {
        system: args.system,
        root_index: args.root,
        root_label,
        scale: format!("{:?}", args.scale),
        voicing: format!("{:?}", args.voicing),
        chord_count: chord_summaries.len(),
        chords: chord_summaries,
    };

    format.emit(&listing, ChordListing::render_text)
}

fn list_modes(engine: &MusicEngine, format: OutputFormat, args: ModesArgs) -> Result<()> {
    let registry = engine.registry();
    let system_id = PitchSystemId::from(args.system.clone());
    let base_scale = args
        .scale
        .build_scale(args.root, &system_id, registry)
        .with_context(|| format!("failed to build {:?} scale", args.scale))?;
    let step_count = base_scale.step_count();
    if step_count == 0 {
        bail!(
            "scale {:?} has no steps; cannot enumerate modes",
            args.scale
        );
    }

    let mut modes = Vec::with_capacity(step_count);
    for rotation in 0..step_count {
        let mode_scale = base_scale.mode(rotation, registry).with_context(|| {
            format!(
                "failed to rotate {:?} scale by {rotation} degrees",
                args.scale
            )
        })?;
        let root_pitch = mode_scale.root().clone();
        let root_index = root_pitch.index();
        let root_label = root_pitch
            .try_label(registry)
            .map(|label| label.to_string_lossy())
            .ok();
        let pitches = collect_mode_pitches(&mode_scale, registry, step_count)?;
        modes.push(ModeSummary {
            mode_index: rotation + 1,
            rotation_degree: rotation,
            mode_name: args
                .scale
                .mode_name_for_rotation(rotation)
                .map(str::to_string),
            root_index,
            root_label,
            pitches,
        });
    }

    let listing = ModeListing {
        system: args.system,
        root_index: args.root,
        scale: format!("{:?}", args.scale),
        mode_count: modes.len(),
        modes,
    };

    format.emit(&listing, ModeListing::render_text)
}
