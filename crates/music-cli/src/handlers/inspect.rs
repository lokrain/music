use anyhow::Result;
use music_engine::prelude::*;

use crate::{
    cli::{InspectCommand, PitchArgs},
    format::OutputFormat,
    responses::PitchSummary,
};

pub fn handle_inspect(
    engine: &MusicEngine,
    format: OutputFormat,
    command: InspectCommand,
) -> Result<()> {
    match command {
        InspectCommand::Pitch(args) => inspect_pitch(engine, format, args),
    }
}

fn inspect_pitch(engine: &MusicEngine, format: OutputFormat, args: PitchArgs) -> Result<()> {
    let system_id = PitchSystemId::from(args.system.clone());
    let pitch = Pitch::abstract_pitch(args.index, system_id);
    let label = engine.describe_pitch(&pitch)?;
    let freq = engine.resolve_pitch(&pitch)?;
    let summary = PitchSummary {
        system: args.system,
        index: args.index,
        label,
        frequency_hz: freq,
    };
    format.emit(&summary, PitchSummary::render_text)
}
