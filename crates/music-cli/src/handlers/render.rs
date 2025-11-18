use anyhow::{Context, Result, bail};
use music_engine::{
    MusicEngine,
    prelude::{Pitch, PitchSystemId},
};

use crate::{
    cli::{RenderCommand, RenderPianoRollArgs, RenderStaffArgs},
    format::OutputFormat,
    responses::{PianoRollRenderReport, StaffRenderReport},
};

pub fn handle_render(
    engine: &MusicEngine,
    format: OutputFormat,
    command: RenderCommand,
) -> Result<()> {
    match command {
        RenderCommand::Staff(args) => render_staff(engine, format, args),
        RenderCommand::PianoRoll(args) => render_piano_roll(engine, format, args),
    }
}

fn render_staff(engine: &MusicEngine, format: OutputFormat, args: RenderStaffArgs) -> Result<()> {
    if args.notes.is_empty() {
        bail!("provide at least one note via --notes");
    }

    let system_id = PitchSystemId::try_new(&args.system)
        .with_context(|| format!("invalid pitch system id '{}'", args.system))?;
    let registry = engine.registry();
    if !registry.contains(&system_id) {
        bail!("pitch system '{}' is not registered", args.system);
    }

    let min_note = *args.notes.iter().min().expect("checked earlier");
    let max_note = *args.notes.iter().max().expect("checked earlier");

    let padding = 2;
    let row_min = min_note - padding;
    let row_max = max_note + padding;
    let row_count = (row_max - row_min + 1) as usize;
    let columns = args.notes.len().max(1) * 2 + 1;
    let charset = StaffCharset::new(args.unicode);

    let mut grid = vec![vec![charset.space; columns]; row_count];
    for (row_idx, row) in grid.iter_mut().enumerate() {
        let pitch = row_max - row_idx as i32;
        if (pitch - row_min) % 2 == 0 {
            row.fill(charset.line);
        }
    }

    for (idx, note) in args.notes.iter().enumerate() {
        let clamped = (*note).clamp(row_min, row_max);
        let row_idx = (row_max - clamped) as usize;
        let col = idx * 2 + 1;
        if let Some(cell) = grid.get_mut(row_idx).and_then(|row| row.get_mut(col)) {
            *cell = charset.note;
        }
    }

    let rows = build_labeled_rows(engine, &system_id, row_max, &grid, charset.frame);
    let note_labels = describe_notes(engine, &system_id, &args.notes);
    let report = StaffRenderReport {
        system: args.system,
        unicode: args.unicode,
        key_hint: args.key_hint,
        note_count: args.notes.len(),
        min_index: row_min,
        max_index: row_max,
        rows,
        note_labels,
    };

    format.emit(&report, StaffRenderReport::render_text)
}

fn render_piano_roll(
    engine: &MusicEngine,
    format: OutputFormat,
    args: RenderPianoRollArgs,
) -> Result<()> {
    if args.notes.is_empty() {
        bail!("provide at least one note via --notes");
    }
    let width = args.width.max(8);
    let height = args.height.max(4);
    let system_id = PitchSystemId::try_new(&args.system)
        .with_context(|| format!("invalid pitch system id '{}'", args.system))?;
    let registry = engine.registry();
    if !registry.contains(&system_id) {
        bail!("pitch system '{}' is not registered", args.system);
    }

    let min_note = *args.notes.iter().min().expect("checked earlier");
    let max_note = *args.notes.iter().max().expect("checked earlier");
    let mut row_min = min_note;
    let mut row_max = max_note;
    let desired_span = height as i32;
    let natural_span = row_max - row_min + 1;
    if natural_span < desired_span {
        let delta = desired_span - natural_span;
        let lower_pad = delta / 2;
        row_min -= lower_pad;
        row_max = row_min + desired_span - 1;
    } else if natural_span > desired_span {
        row_max = row_min + desired_span - 1;
    }

    let charset = PianoRollCharset::new(args.unicode);
    let mut grid = vec![vec![charset.background; width]; height];
    for (idx, note) in args.notes.iter().enumerate() {
        let clamped = (*note).clamp(row_min, row_max);
        let row_idx = (row_max - clamped) as usize;
        let column = if args.notes.len() == 1 {
            width / 2
        } else {
            let denom = args.notes.len() - 1;
            (idx * (width - 1)) / denom
        };
        if let Some(cell) = grid.get_mut(row_idx).and_then(|row| row.get_mut(column)) {
            *cell = charset.note;
        }
    }

    let rows = build_labeled_rows(engine, &system_id, row_max, &grid, charset.frame);
    let note_labels = describe_notes(engine, &system_id, &args.notes);
    let report = PianoRollRenderReport {
        system: args.system,
        width,
        height,
        note_count: args.notes.len(),
        min_index: row_min,
        max_index: row_max,
        rows,
        note_labels,
    };

    format.emit(&report, PianoRollRenderReport::render_text)
}

fn build_labeled_rows(
    engine: &MusicEngine,
    system: &PitchSystemId,
    row_max: i32,
    grid: &[Vec<char>],
    frame_char: char,
) -> Vec<String> {
    grid.iter()
        .enumerate()
        .map(|(row_idx, cells)| {
            let pitch_index = row_max - row_idx as i32;
            let label = describe_pitch_for_row(engine, system, pitch_index);
            let mut line = format!("{label:>12} {frame}", frame = frame_char);
            for cell in cells {
                line.push(*cell);
            }
            line.push(frame_char);
            line
        })
        .collect()
}

fn describe_notes(engine: &MusicEngine, system: &PitchSystemId, notes: &[i32]) -> Vec<String> {
    notes
        .iter()
        .map(|index| describe_pitch_for_row(engine, system, *index))
        .collect()
}

fn describe_pitch_for_row(engine: &MusicEngine, system: &PitchSystemId, index: i32) -> String {
    let pitch = Pitch::abstract_pitch(index, system.clone());
    engine
        .describe_pitch(&pitch)
        .unwrap_or_else(|_| format!("{}({index})", system))
}

struct StaffCharset {
    line: char,
    space: char,
    note: char,
    frame: char,
}

impl StaffCharset {
    fn new(unicode: bool) -> Self {
        if unicode {
            Self {
                line: '─',
                space: ' ',
                note: '●',
                frame: '│',
            }
        } else {
            Self {
                line: '-',
                space: ' ',
                note: 'o',
                frame: '|',
            }
        }
    }
}

struct PianoRollCharset {
    background: char,
    note: char,
    frame: char,
}

impl PianoRollCharset {
    fn new(unicode: bool) -> Self {
        if unicode {
            Self {
                background: '·',
                note: '█',
                frame: '│',
            }
        } else {
            Self {
                background: ' ',
                note: '#',
                frame: '|',
            }
        }
    }
}
