use anyhow::{Context, Result, bail};
use chord::{diatonic_seventh, diatonic_triad};
use music_engine::prelude::*;

use crate::{
    cli::{ExplainChordArgs, ExplainCommand, ExplainPitchArgs, ExplainScaleArgs},
    format::OutputFormat,
    handlers::analyze::{
        FunctionRole, apply_pattern, classify_function, describe_pitch_class, parse_key_hint,
    },
    responses::{
        ChordExplanation, PitchContext, PitchExplanation, PitchSummary, ScaleDegreeSummary,
        ScaleExplanation, summarize_chord,
    },
};

pub fn handle_explain(
    engine: &MusicEngine,
    format: OutputFormat,
    command: ExplainCommand,
) -> Result<()> {
    match command {
        ExplainCommand::Pitch(args) => explain_pitch(engine, format, args),
        ExplainCommand::Scale(args) => explain_scale(engine, format, args),
        ExplainCommand::Chord(args) => explain_chord(engine, format, args),
    }
}

fn explain_pitch(engine: &MusicEngine, format: OutputFormat, args: ExplainPitchArgs) -> Result<()> {
    let system_id = PitchSystemId::from(args.system.clone());
    let pitch = Pitch::abstract_pitch(args.index, system_id.clone());
    let freq = engine.resolve_pitch(&pitch)?;
    let label = engine.describe_pitch(&pitch)?;

    let summary = PitchSummary {
        system: args.system,
        index: args.index,
        label,
        frequency_hz: freq,
    };

    let pitch_class = ((args.index % 12) + 12) % 12;
    let octave = (args.index / 12) - 1;
    let semitone_delta = args.index - 69;
    let cents_offset = 1200.0 * (freq / 440.0).log2();
    let pitch_class_label = pitch_class_name(pitch_class as u8).to_string();

    let mut narrative = Vec::new();
    narrative.push(format!(
        "{label} sits in octave {octave} and resonates at {freq:.3} Hz inside {system}.",
        label = summary.label,
        octave = octave,
        freq = freq,
        system = summary.system
    ));
    if semitone_delta != 0 {
        narrative.push(format!(
            "It is {delta:+} semitone(s) away from concert A4, {cents:+.1} cents in tempered tuning.",
            delta = semitone_delta,
            cents = cents_offset
        ));
    } else {
        narrative.push("It matches the concert A4 reference exactly.".to_string());
    }

    let context = build_pitch_context(engine, &system_id, pitch_class as u8, &args.key_hint)?;

    let explanation = PitchExplanation {
        summary,
        octave,
        pitch_class: pitch_class as u8,
        pitch_class_label,
        semitone_delta_from_a4: semitone_delta,
        cents_offset_from_a4: cents_offset,
        context,
        narrative,
    };

    format.emit(&explanation, PitchExplanation::render_text)
}

fn explain_scale(engine: &MusicEngine, format: OutputFormat, args: ExplainScaleArgs) -> Result<()> {
    let registry = engine.registry();
    let system_id = PitchSystemId::from(args.system.clone());
    let scale = args
        .scale
        .build_scale(args.root, &system_id, registry)
        .with_context(|| format!("failed to build {:?} scale", args.scale))?;

    let root_label = scale
        .root()
        .try_label(registry)
        .map(|value| value.to_string_lossy())
        .or_else(|_| engine.describe_pitch(scale.root()))
        .unwrap_or_else(|_| format!("{}({})", system_id, args.root));

    let degrees_to_cover = if args.degrees == 0 { 1 } else { args.degrees };
    let highest_degree = degrees_to_cover.saturating_sub(1);

    let mut degrees = Vec::with_capacity(degrees_to_cover);
    for entry in scale.degrees_up_to(highest_degree, registry) {
        let (degree_idx, interval, pitch) = entry?;
        let label = pitch
            .try_label(registry)
            .map(|value| value.to_string_lossy())
            .or_else(|_| engine.describe_pitch(&pitch))
            .unwrap_or_else(|_| format!("{}", pitch));
        let frequency_hz = pitch.try_freq_hz(registry)?;
        let interval_cents = interval.cents();
        let semitone_offset = interval.steps().map(|(delta, _)| delta);
        let role = diatonic_degree_label(degree_idx);
        degrees.push(ScaleDegreeSummary {
            degree: degree_idx + 1,
            label,
            frequency_hz,
            interval_cents,
            semitone_offset,
            role: role.to_string(),
        });
    }

    let pattern_cents: Vec<f32> = scale
        .pattern()
        .steps()
        .iter()
        .map(|interval| interval.cents())
        .collect();
    let total_span: f32 = pattern_cents.iter().copied().sum();
    let octaves = total_span / 1200.0;

    let mut narrative = Vec::new();
    narrative.push(format!(
        "The {scale:?} template covers roughly {octaves:.2} octave(s) in {system}.",
        scale = args.scale,
        octaves = octaves,
        system = args.system
    ));
    if let Some(last_degree) = degrees.last() {
        narrative.push(format!(
            "Degree {deg} ({label}) stretches to {freq:.3} Hz and acts as {role} tension.",
            deg = last_degree.degree,
            label = last_degree.label,
            freq = last_degree.frequency_hz,
            role = last_degree.role
        ));
    }

    let explanation = ScaleExplanation {
        system: args.system,
        root_index: args.root,
        root_label,
        scale_name: format!("{:?}", args.scale),
        mode_alias: args.scale.mode_name_for_rotation(0).map(str::to_string),
        degrees,
        pattern_cents,
        narrative,
    };

    format.emit(&explanation, ScaleExplanation::render_text)
}

fn explain_chord(engine: &MusicEngine, format: OutputFormat, args: ExplainChordArgs) -> Result<()> {
    if args.degree == 0 {
        bail!("degree must be a positive integer (1-indexed)");
    }
    let registry = engine.registry();
    let system_id = PitchSystemId::from(args.system.clone());
    let scale = args
        .scale
        .build_scale(args.root, &system_id, registry)
        .with_context(|| format!("failed to build {:?} scale", args.scale))?;

    let degree_index = args.degree - 1;
    let chord = match args.voicing {
        crate::theory::ChordVoicing::Triads => diatonic_triad(&scale, degree_index, registry)?,
        crate::theory::ChordVoicing::Sevenths => diatonic_seventh(&scale, degree_index, registry)?,
    };

    let summary = summarize_chord(chord.clone(), registry)?;
    let function_role = describe_function_role(classify_function(&summary.numeral));

    let scale_root_label = scale
        .root()
        .try_label(registry)
        .map(|value| value.to_string_lossy())
        .ok();

    let mut narrative = Vec::new();
    if let Some(quality) = summary.quality.as_deref() {
        narrative.push(format!(
            "Stacked thirds produce a {quality} with {count} tone(s), emphasizing its {function} pull.",
            quality = quality,
            count = summary.tones.len(),
            function = function_role
        ));
    } else {
        narrative.push(format!(
            "The chord contains {count} tone(s) and functions as {function} in context.",
            count = summary.tones.len(),
            function = function_role
        ));
    }
    if let Some(root) = summary.tones.first() {
        let label = root.label.as_deref().unwrap_or("—");
        narrative.push(format!(
            "Root tone {label} anchors the chord before the remaining voices outline the interval profile.",
            label = label
        ));
    }

    let explanation = ChordExplanation {
        system: args.system,
        scale_name: format!("{:?}", args.scale),
        scale_root_index: args.root,
        scale_root_label,
        voicing: format!("{:?}", args.voicing),
        summary,
        function: function_role.to_string(),
        narrative,
    };

    format.emit(&explanation, ChordExplanation::render_text)
}

fn build_pitch_context(
    engine: &MusicEngine,
    system: &PitchSystemId,
    pitch_class: u8,
    key_hint: &Option<String>,
) -> Result<Option<PitchContext>> {
    let Some(hint) = key_hint else {
        return Ok(None);
    };
    let (root_pc, mode) = parse_key_hint(hint)?;
    let members = apply_pattern(root_pc, mode.pattern());
    if let Some(idx) = members.iter().position(|pc| *pc == pitch_class) {
        let degree = idx + 1;
        let degree_label = diatonic_degree_label(idx).to_string();
        let function_role = describe_function_role(classify_function(roman_for_degree(idx)));
        let function = function_role.to_string();
        let tonic_label = describe_pitch_class(engine, system, root_pc)?;
        return Ok(Some(PitchContext {
            key: tonic_label,
            mode: mode.name().into(),
            degree,
            degree_label,
            function,
        }));
    }
    Ok(None)
}

fn pitch_class_name(pc: u8) -> &'static str {
    const NAMES: [&str; 12] = [
        "C",
        "C♯/D♭",
        "D",
        "D♯/E♭",
        "E",
        "F",
        "F♯/G♭",
        "G",
        "G♯/A♭",
        "A",
        "A♯/B♭",
        "B",
    ];
    NAMES.get(pc as usize % 12).copied().unwrap_or("?")
}

fn diatonic_degree_label(degree_idx: usize) -> &'static str {
    match degree_idx % 7 {
        0 => "tonic",
        1 => "supertonic",
        2 => "mediant",
        3 => "subdominant",
        4 => "dominant",
        5 => "submediant",
        _ => "leading tone",
    }
}

fn describe_function_role(role: FunctionRole) -> &'static str {
    match role {
        FunctionRole::Tonic => "tonic",
        FunctionRole::Predominant => "predominant",
        FunctionRole::Dominant => "dominant",
        FunctionRole::Other => "color",
    }
}

fn roman_for_degree(degree_idx: usize) -> &'static str {
    const ROMANS: [&str; 7] = ["I", "II", "III", "IV", "V", "VI", "VII"];
    ROMANS
        .get(degree_idx % ROMANS.len())
        .copied()
        .unwrap_or("I")
}
