use anyhow::{Context, Result, anyhow};
use music_engine::prelude::*;

use crate::{
    cli::{
        GenerateArpeggioArgs, GenerateCommand, GenerateCommonArgs, GenerateMotifArgs,
        GenerateRhythmArgs, PatternDensity,
    },
    format::OutputFormat,
    responses::{
        ArpeggioGeneration, ArpeggioStep, GeneratedNote, MotifGeneration, PatternContext,
        RhythmCellGeneration, RhythmEvent,
    },
};

pub fn handle_generate(
    engine: &MusicEngine,
    format: OutputFormat,
    command: GenerateCommand,
) -> Result<()> {
    match command {
        GenerateCommand::Motif(args) => generate_motif(engine, format, args),
        GenerateCommand::Arpeggio(args) => generate_arpeggio(engine, format, args),
        GenerateCommand::Rhythm(args) => generate_rhythm(engine, format, args),
    }
}

fn generate_motif(
    engine: &MusicEngine,
    format: OutputFormat,
    args: GenerateMotifArgs,
) -> Result<()> {
    let context = GenerationContext::new(engine, &args.common)?;
    let registry = engine.registry();
    let pattern = motif_pattern(context.density);
    let notes = collect_notes(&context, registry, pattern)?;
    let contour = contour(&notes);
    let response = MotifGeneration {
        context: context.pattern_context(),
        note_count: notes.len(),
        contour,
        notes,
        description: motif_description(context.density).to_string(),
    };

    format.emit(&response, MotifGeneration::render_text)
}

fn generate_arpeggio(
    engine: &MusicEngine,
    format: OutputFormat,
    args: GenerateArpeggioArgs,
) -> Result<()> {
    let context = GenerationContext::new(engine, &args.common)?;
    let registry = engine.registry();
    let pattern = arpeggio_pattern(context.density);
    let notes = collect_notes(&context, registry, pattern)?;
    let mut prev_index: Option<i32> = None;
    let mut min_index = i32::MAX;
    let mut max_index = i32::MIN;
    let mut steps = Vec::with_capacity(notes.len());

    for (idx, note) in notes.into_iter().enumerate() {
        min_index = min_index.min(note.index);
        max_index = max_index.max(note.index);
        let direction = match prev_index {
            None => "root",
            Some(prev) => direction_symbol(note.index - prev),
        };
        prev_index = Some(note.index);
        steps.push(ArpeggioStep {
            order: idx + 1,
            direction: direction.to_string(),
            note,
        });
    }

    let register_span = if min_index <= max_index {
        max_index - min_index
    } else {
        0
    };

    let response = ArpeggioGeneration {
        context: context.pattern_context(),
        register_span,
        steps,
        description: arpeggio_description(context.density).to_string(),
    };

    format.emit(&response, ArpeggioGeneration::render_text)
}

fn generate_rhythm(
    engine: &MusicEngine,
    format: OutputFormat,
    args: GenerateRhythmArgs,
) -> Result<()> {
    let context = GenerationContext::new(engine, &args.common)?;
    let durations = rhythm_durations(context.density);
    let degrees = rhythm_degree_hints(context.density);
    let mut beat = 0.0f32;
    let mut events = Vec::with_capacity(durations.len());

    for (idx, duration) in durations.iter().enumerate() {
        let accent = beat.fract().abs() < 1e-3
            || *duration >= 1.0
            || (context.density == PatternDensity::Dense && idx % 2 == 1);
        let degree = degrees[idx % degrees.len()];
        events.push(RhythmEvent {
            beat,
            duration_beats: *duration,
            accent,
            suggested_degree: degree,
        });
        beat += *duration;
    }

    let response = RhythmCellGeneration {
        context: context.pattern_context(),
        meter: "4/4".to_string(),
        length_beats: beat,
        events,
        description: rhythm_description(context.density).to_string(),
    };

    format.emit(&response, RhythmCellGeneration::render_text)
}

struct GenerationContext {
    system: String,
    root_index: i32,
    root_label: Option<String>,
    scale_kind: crate::theory::ScaleKind,
    scale: Scale,
    density: PatternDensity,
}

impl GenerationContext {
    fn new(engine: &MusicEngine, args: &GenerateCommonArgs) -> Result<Self> {
        let registry = engine.registry();
        let system_id = PitchSystemId::from(args.system.clone());
        let scale = args
            .scale
            .build_scale(args.root, &system_id, registry)
            .with_context(|| format!("failed to build {:?} scale", args.scale))?;
        let root_label = scale
            .root()
            .try_label(registry)
            .map(|label| label.to_string_lossy())
            .ok();
        Ok(Self {
            system: args.system.clone(),
            root_index: args.root,
            root_label,
            scale_kind: args.scale,
            scale,
            density: args.density,
        })
    }

    fn pattern_context(&self) -> PatternContext {
        PatternContext {
            system: self.system.clone(),
            scale: format!("{:?}", self.scale_kind),
            root_index: self.root_index,
            root_label: self.root_label.clone(),
            density: density_label(self.density).to_string(),
        }
    }
}

fn collect_notes(
    context: &GenerationContext,
    registry: &TuningRegistry,
    pattern: &[usize],
) -> Result<Vec<GeneratedNote>> {
    pattern
        .iter()
        .map(|degree| summarize_degree(context, registry, *degree))
        .collect::<Result<Vec<_>>>()
}

fn summarize_degree(
    context: &GenerationContext,
    registry: &TuningRegistry,
    degree: usize,
) -> Result<GeneratedNote> {
    let pitch = context.scale.degree_pitch(degree, registry)?;
    let label = pitch
        .try_label(registry)
        .map(|value| value.to_string_lossy())
        .ok();
    let frequency_hz = pitch.try_freq_hz(registry)?;
    let step_count = context.scale.step_count().max(1);
    let octave = (degree / step_count) as i32;
    let degree_in_scale = (degree % step_count) + 1;
    let index = pitch.index().ok_or_else(|| {
        anyhow!(
            "scale degree {} could not be resolved to an index in {}",
            degree_in_scale,
            context.system
        )
    })?;
    Ok(GeneratedNote {
        degree: degree_in_scale,
        octave,
        index,
        label,
        frequency_hz,
    })
}

fn contour(notes: &[GeneratedNote]) -> Vec<i32> {
    notes
        .windows(2)
        .map(|pair| pair[1].index - pair[0].index)
        .collect()
}

fn motif_pattern(density: PatternDensity) -> &'static [usize] {
    match density {
        PatternDensity::Sparse => &[0, 2, 4, 5],
        PatternDensity::Balanced => &[0, 2, 4, 7, 5, 2, 0],
        PatternDensity::Dense => &[0, 2, 4, 7, 9, 7, 5, 4, 2, 0],
    }
}

fn arpeggio_pattern(density: PatternDensity) -> &'static [usize] {
    match density {
        PatternDensity::Sparse => &[0, 2, 4, 7, 4, 2],
        PatternDensity::Balanced => &[0, 2, 4, 6, 7, 6, 4, 2, 0],
        PatternDensity::Dense => &[0, 2, 4, 6, 7, 9, 11, 9, 7, 6, 4, 2, 0],
    }
}

fn rhythm_durations(density: PatternDensity) -> &'static [f32] {
    match density {
        PatternDensity::Sparse => &[2.0, 2.0],
        PatternDensity::Balanced => &[1.0, 0.5, 0.5, 2.0],
        PatternDensity::Dense => &[0.5, 0.5, 1.0, 0.5, 0.5, 1.0],
    }
}

fn rhythm_degree_hints(density: PatternDensity) -> &'static [usize] {
    match density {
        PatternDensity::Sparse => &[1, 5],
        PatternDensity::Balanced => &[1, 5, 3, 2],
        PatternDensity::Dense => &[1, 5, 3, 4, 6, 2],
    }
}

fn motif_description(density: PatternDensity) -> &'static str {
    match density {
        PatternDensity::Sparse => "Triadic cell with a cadential descent",
        PatternDensity::Balanced => "Octave-reaching contour with a dominant setup",
        PatternDensity::Dense => "Extended diatonic sweep with upper neighbor turns",
    }
}

fn arpeggio_description(density: PatternDensity) -> &'static str {
    match density {
        PatternDensity::Sparse => "Simple tonic arpeggiation with resolution",
        PatternDensity::Balanced => "Seventh arpeggio cycling up then falling back",
        PatternDensity::Dense => "Ninth arpeggio with pendulum motion and passing tones",
    }
}

fn rhythm_description(density: PatternDensity) -> &'static str {
    match density {
        PatternDensity::Sparse => "Two-beat pillars for breathing room",
        PatternDensity::Balanced => "Backbeat syncopation feeding into a long tone",
        PatternDensity::Dense => "Offbeat sixteenths glued by mid-bar accents",
    }
}

fn density_label(density: PatternDensity) -> &'static str {
    match density {
        PatternDensity::Sparse => "Sparse",
        PatternDensity::Balanced => "Balanced",
        PatternDensity::Dense => "Dense",
    }
}

fn direction_symbol(delta: i32) -> &'static str {
    if delta > 0 {
        "up"
    } else if delta < 0 {
        "down"
    } else {
        "hold"
    }
}
