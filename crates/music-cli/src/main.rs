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
        handle_analyze, handle_explain, handle_expose, handle_inspect, handle_list,
        handle_placeholder, handle_suggest,
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
        Command::Convert => handle_placeholder("convert"),
        Command::Validate => handle_placeholder("validate"),
        Command::Render => handle_placeholder("render"),
        Command::Expose { command } => handle_expose(engine, format, command),
        Command::Generate => handle_placeholder("generate"),
        Command::Score => handle_placeholder("score"),
        Command::Extrapolate => handle_placeholder("extrapolate"),
        Command::ExplainDiff => handle_placeholder("explain-diff"),
        Command::Map => handle_placeholder("map"),
        Command::Profile => handle_placeholder("profile"),
        Command::Interpolate => handle_placeholder("interpolate"),
        Command::Search => handle_placeholder("search"),
        Command::Estimate => handle_placeholder("estimate"),
        Command::Resolve => handle_placeholder("resolve"),
    }
}
