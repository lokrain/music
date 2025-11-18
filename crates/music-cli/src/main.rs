use std::process;

use anyhow::Result;
use clap::Parser;
use music_engine::MusicEngine;

mod cli;
mod format;
mod handlers;
mod responses;
mod theory;

use crate::{
    cli::{Cli, Command},
    handlers::{
        handle_analyze, handle_convert, handle_explain, handle_explain_diff, handle_expose,
        handle_extrapolate, handle_generate, handle_inspect, handle_list, handle_map,
        handle_placeholder, handle_render, handle_score, handle_suggest, handle_validate,
    },
};

fn main() {
    if let Err(error) = run() {
        eprintln!("{error:?}");
        process::exit(1);
    }
}

fn run() -> Result<()> {
    let cli = Cli::parse();
    let engine = MusicEngine::with_default_systems();
    dispatch(&engine, cli)
}

fn dispatch(engine: &MusicEngine, cli: Cli) -> Result<()> {
    let Cli { format, command } = cli;
    match command {
        Command::List { command } => handle_list(engine, format, command),
        Command::Inspect { command } => handle_inspect(engine, format, command),
        Command::Analyze { command } => handle_analyze(engine, format, command),
        Command::Suggest { command } => handle_suggest(engine, format, command),
        Command::Explain { command } => handle_explain(engine, format, command),
        Command::Convert { command } => handle_convert(engine, format, command),
        Command::Validate { command } => handle_validate(engine, format, command),
        Command::Render { command } => handle_render(engine, format, command),
        Command::Expose { command } => handle_expose(engine, format, command),
        Command::Generate { command } => handle_generate(engine, format, command),
        Command::Score { command } => handle_score(engine, format, command),
        Command::Extrapolate { command } => handle_extrapolate(engine, format, command),
        Command::ExplainDiff { command } => handle_explain_diff(engine, format, command),
        Command::Map { command } => handle_map(engine, format, command),
        Command::Profile => handle_placeholder("profile"),
        Command::Interpolate => handle_placeholder("interpolate"),
        Command::Search => handle_placeholder("search"),
        Command::Estimate => handle_placeholder("estimate"),
        Command::Resolve => handle_placeholder("resolve"),
    }
}
