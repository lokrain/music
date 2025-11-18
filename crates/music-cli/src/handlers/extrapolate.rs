use anyhow::{Context as _, Result, bail};
use music_analysis::{ChordTransitionModel, MelodyTransitionModel, TransitionModel};
use music_engine::prelude::*;

use crate::{
    cli::{ExtrapolateChordsArgs, ExtrapolateCommand, ExtrapolateMelodyArgs},
    format::OutputFormat,
    responses::{
        ChordExtrapolationReport, ChordPrediction, MelodyExtrapolationReport, MelodyPrediction,
    },
};

pub fn handle_extrapolate(
    engine: &MusicEngine,
    format: OutputFormat,
    command: ExtrapolateCommand,
) -> Result<()> {
    match command {
        ExtrapolateCommand::Melody(args) => extrapolate_melody(engine, format, args),
        ExtrapolateCommand::Chords(args) => extrapolate_chords(format, args),
    }
}

fn extrapolate_melody(
    engine: &MusicEngine,
    format: OutputFormat,
    args: ExtrapolateMelodyArgs,
) -> Result<()> {
    if args.notes.is_empty() {
        bail!("provide at least one note via --notes");
    }

    if args.notes.len() <= args.order {
        bail!(
            "need at least {} notes for order-{} n-gram model",
            args.order + 1,
            args.order
        );
    }

    let system_id = PitchSystemId::from(args.system.clone());

    // Convert notes to pitch classes for the model
    let pitch_classes: Vec<u8> = args
        .notes
        .iter()
        .map(|&index| {
            let mut pc = index % 12;
            if pc < 0 {
                pc += 12;
            }
            pc as u8
        })
        .collect();

    // Train model on the input sequence
    let mut model = MelodyTransitionModel::new(args.order);
    model.train(&pitch_classes);

    // Extract context (last `order` pitch classes)
    let context = &pitch_classes[pitch_classes.len() - args.order..];

    // Generate predictions
    let raw_predictions = model.predict(context, args.count);

    // Convert predictions to full pitch information
    // We'll suggest indices in the same octave as the last input note
    let last_input_index = *args.notes.last().unwrap();
    let base_octave_index = (last_input_index / 12) * 12;

    let predictions: Result<Vec<_>> = raw_predictions
        .into_iter()
        .map(|pred| {
            let suggested_index = base_octave_index + i32::from(pred.token);
            let pitch = Pitch::abstract_pitch(suggested_index, system_id.clone());
            let label = pitch
                .try_label(engine.registry())
                .ok()
                .map(|l| l.to_string_lossy());
            let frequency_hz = pitch.try_freq_hz(engine.registry()).with_context(|| {
                format!("failed to resolve frequency for pitch {suggested_index}")
            })?;

            Ok(MelodyPrediction {
                pitch_class: pred.token,
                suggested_index,
                label,
                frequency_hz,
                probability: pred.probability,
            })
        })
        .collect();

    let report = MelodyExtrapolationReport {
        system: args.system,
        input_count: args.notes.len(),
        input_notes: args.notes,
        model_order: args.order,
        context_pitch_classes: context.to_vec(),
        predictions: predictions?,
        key_hint: args.key_hint,
    };

    format.emit(&report, MelodyExtrapolationReport::render_text)
}

fn extrapolate_chords(format: OutputFormat, args: ExtrapolateChordsArgs) -> Result<()> {
    let progression: Vec<String> = args
        .progression
        .into_iter()
        .map(|token| token.trim().to_string())
        .filter(|token| !token.is_empty())
        .collect();

    if progression.is_empty() {
        bail!("provide at least one chord via --progression");
    }

    if progression.len() <= args.order {
        bail!(
            "need at least {} chords for order-{} n-gram model",
            args.order + 1,
            args.order
        );
    }

    // Train model on the input progression
    let mut model = ChordTransitionModel::new(args.order);
    model.train_from_progression(&progression);

    // Extract context (last `order` chords)
    let context = &progression[progression.len() - args.order..];

    // Generate predictions
    let raw_predictions = model.predict_from_context(context, args.count);

    let predictions: Vec<ChordPrediction> = raw_predictions
        .into_iter()
        .map(|pred| ChordPrediction {
            chord: pred.token,
            probability: pred.probability,
        })
        .collect();

    let report = ChordExtrapolationReport {
        input_count: progression.len(),
        input_progression: progression.clone(),
        model_order: args.order,
        context_chords: context.to_vec(),
        predictions,
        key_hint: args.key_hint,
    };

    format.emit(&report, ChordExtrapolationReport::render_text)
}
