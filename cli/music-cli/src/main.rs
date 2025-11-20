mod args;
mod planner_util;
mod report;
mod style;
mod templates;

use anyhow::{Context, Result};
use clap::Parser;
use music_score::planner::{ExplainMode, plan_section};
use music_theory::key::Mode;

use crate::args::{Cli, Commands, PlanArgs};
use crate::planner_util::{parse_key, resolve_template};
use crate::report::{build_json_report, print_text_report};
use crate::style::profile_for_preset;
use crate::templates::run_template_command;

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Plan(args) => run_plan(args),
        Commands::Templates(args) => run_template_command(args.command),
    }
}

fn run_plan(args: PlanArgs) -> Result<()> {
    let (template, locator) =
        resolve_template(args.template.as_deref(), args.template_path.as_deref())?;
    let mode: Mode = args.mode.into();
    let key = parse_key(&args.tonic, mode)?;
    let explain_mode: ExplainMode = args.explain.into();
    let style_label = args.style.label();
    let profile = profile_for_preset(args.style, explain_mode);

    let planned = plan_section(&template, key, &profile)
        .with_context(|| format!("planner failed for template {}", template.metadata.id))?;
    let summaries = planned.explain_summaries();

    print_text_report(&template, key, &locator, style_label, explain_mode, &planned, &summaries);
    println!("\n--- JSON report ---");
    let json = build_json_report(
        &template,
        key,
        &locator,
        style_label,
        explain_mode,
        &planned,
        &summaries,
    );
    let serialized =
        serde_json::to_string_pretty(&json).context("failed to serialize JSON report")?;
    println!("{}", serialized);

    Ok(())
}
