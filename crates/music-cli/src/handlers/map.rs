use std::collections::HashSet;

use anyhow::{Context, Result, anyhow};
use music_engine::prelude::*;

use crate::{
    cli::{MapCommand, MapScaleArgs},
    format::OutputFormat,
    responses::{
        ModulatoryPathSummary, PitchClassMapEntry, ScaleMapReport, ScalePitchClassProjection,
    },
};

pub fn handle_map(engine: &MusicEngine, format: OutputFormat, command: MapCommand) -> Result<()> {
    match command {
        MapCommand::Scale(args) => map_scale(engine, format, args),
    }
}

fn map_scale(engine: &MusicEngine, format: OutputFormat, args: MapScaleArgs) -> Result<()> {
    let registry = engine.registry();
    let system_id = PitchSystemId::from(args.system.clone());
    let scale = args
        .scale
        .build_scale(args.root, &system_id, registry)
        .with_context(|| format!("failed to build {:?} scale", args.scale))?;
    let step_count = scale.step_count().max(1);

    let mut members = Vec::with_capacity(step_count);
    let mut pitch_class_map: [Option<ScalePitchClassProjection>; 12] =
        std::array::from_fn(|_| None);

    for degree in 0..step_count {
        let pitch = scale
            .degree_pitch(degree, registry)
            .with_context(|| format!("failed to resolve degree {degree}"))?;
        let index = pitch
            .index()
            .ok_or_else(|| anyhow!("scale pitch has no concrete index"))?;
        let pitch_class = ((index % 12) + 12) % 12;
        let label = pitch
            .try_label(registry)
            .map(|value| value.to_string_lossy())
            .ok();
        let frequency_hz = pitch.try_freq_hz(registry)?;
        let projection = ScalePitchClassProjection {
            degree: degree + 1,
            pitch_class: pitch_class as u8,
            index,
            label,
            frequency_hz,
        };
        if pitch_class_map[pitch_class as usize].is_none() {
            pitch_class_map[pitch_class as usize] = Some(projection.clone());
        }
        members.push(projection);
    }

    let pitch_class_rows: Vec<PitchClassMapEntry> = pitch_class_map
        .iter()
        .enumerate()
        .map(|(pc, projection)| PitchClassMapEntry {
            pitch_class: pc as u8,
            occupied: projection.is_some(),
            degree: projection.as_ref().map(|p| p.degree),
            label: projection.as_ref().and_then(|p| p.label.clone()),
        })
        .collect();

    let base_pitch_classes: HashSet<u8> = members.iter().map(|m| m.pitch_class).collect();

    let mut modulatory_paths = Vec::new();
    let max_rotations = step_count.saturating_sub(1);
    let requested = args.modulations.min(max_rotations);
    for rotation in 1..=requested {
        let mode_scale = scale
            .mode(rotation, registry)
            .with_context(|| format!("failed to rotate {:?} scale by {rotation}", args.scale))?;
        let root_pitch = mode_scale.root().clone();
        let root_index = root_pitch
            .index()
            .ok_or_else(|| anyhow!("mode root lacks index"))?;
        let root_label = root_pitch
            .try_label(registry)
            .map(|value| value.to_string_lossy())
            .ok();
        let mut rotated_pitch_classes = HashSet::new();
        for degree in 0..step_count {
            let pitch = mode_scale.degree_pitch(degree, registry)?;
            let index = pitch
                .index()
                .ok_or_else(|| anyhow!("mode pitch lacks index"))?;
            let pc = ((index % 12) + 12) % 12;
            rotated_pitch_classes.insert(pc as u8);
        }
        let mut pivots: Vec<u8> = base_pitch_classes
            .intersection(&rotated_pitch_classes)
            .copied()
            .collect();
        pivots.sort();
        modulatory_paths.push(ModulatoryPathSummary {
            rotation,
            mode_name: args
                .scale
                .mode_name_for_rotation(rotation)
                .map(str::to_string),
            root_index,
            root_label,
            pivot_pitch_classes: pivots,
        });
    }

    let root_label = scale
        .root()
        .try_label(registry)
        .map(|label| label.to_string_lossy())
        .ok();

    let report = ScaleMapReport {
        system: args.system,
        root_index: args.root,
        root_label,
        scale: format!("{:?}", args.scale),
        members,
        pitch_class_map: pitch_class_rows,
        modulatory_paths,
    };

    format.emit(&report, ScaleMapReport::render_text)
}
