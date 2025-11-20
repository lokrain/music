use std::path::PathBuf;

use clap::{Args, Parser, Subcommand, ValueEnum};
use music_score::planner::ExplainMode;
use music_theory::key::Mode;

#[derive(Parser, Debug)]
#[command(name = "music-cli", version, about = "Command-line utilities for the music workspace")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Plan a harmonic section using the built-in planner.
    Plan(PlanArgs),
    /// Manage section templates (list/show/import/export).
    Templates(TemplatesArgs),
}

#[derive(Args, Debug, Clone)]
pub struct TemplatesArgs {
    #[command(subcommand)]
    pub command: TemplateCommands,
}

#[derive(Subcommand, Debug, Clone)]
pub enum TemplateCommands {
    /// List available templates (built-in and/or local).
    List(TemplateListArgs),
    /// Show metadata for a template.
    Show(TemplateShowArgs),
    /// Import a template DSL file into the local registry.
    Import(TemplateImportArgs),
    /// Export a template to disk.
    Export(TemplateExportArgs),
}

#[derive(Args, Debug, Clone)]
pub struct PlanArgs {
    /// Built-in template identifier (see `music_score::planner::builtin_template_ids`).
    #[arg(long, value_name = "ID")]
    pub template: Option<String>,

    /// Path to a template file (JSON5/RON) parsed via `load_template_from_path`.
    #[arg(long, value_name = "PATH")]
    pub template_path: Option<PathBuf>,

    /// Key tonic (e.g., C, F#, Bb).
    #[arg(long, default_value = "C", value_name = "TONIC")]
    pub tonic: String,

    /// Key mode.
    #[arg(long, value_enum, default_value_t = ModeArg::Major)]
    pub mode: ModeArg,

    /// Planner style preset controlling risk and modulation behavior.
    #[arg(long, value_enum, default_value_t = StylePresetArg::Balanced)]
    pub style: StylePresetArg,

    /// Explainability capture mode.
    #[arg(long, value_enum, default_value_t = ExplainArg::Brief)]
    pub explain: ExplainArg,
}

#[derive(Copy, Clone, Debug, ValueEnum, PartialEq, Eq)]
pub enum ModeArg {
    Major,
    Minor,
}

impl From<ModeArg> for Mode {
    fn from(value: ModeArg) -> Self {
        match value {
            ModeArg::Major => Mode::Major,
            ModeArg::Minor => Mode::Minor,
        }
    }
}

#[derive(Copy, Clone, Debug, ValueEnum, PartialEq, Eq)]
pub enum ExplainArg {
    None,
    Brief,
    Detailed,
    Debug,
}

impl From<ExplainArg> for ExplainMode {
    fn from(value: ExplainArg) -> Self {
        match value {
            ExplainArg::None => ExplainMode::None,
            ExplainArg::Brief => ExplainMode::Brief,
            ExplainArg::Detailed => ExplainMode::Detailed,
            ExplainArg::Debug => ExplainMode::Debug,
        }
    }
}

#[derive(Copy, Clone, Debug, ValueEnum, PartialEq, Eq)]
pub enum StylePresetArg {
    Balanced,
    SmoothBallad,
    GospelDrive,
    PopRadio,
}

#[derive(Args, Debug, Clone)]
pub struct TemplateListArgs {
    /// Choose which sources to include.
    #[arg(long, value_enum, default_value_t = TemplateSourceFilter::All)]
    pub source: TemplateSourceFilter,

    /// Include verbose metadata for each template.
    #[arg(long)]
    pub verbose: bool,
}

#[derive(Args, Debug, Clone)]
pub struct TemplateShowArgs {
    /// Template identifier to show.
    pub id: String,

    /// Choose which registry to query.
    #[arg(long, value_enum, default_value_t = TemplateSourcePriority::Auto)]
    pub source: TemplateSourcePriority,

    /// Emit the raw DSL when available (local templates only).
    #[arg(long)]
    pub raw: bool,
}

#[derive(Args, Debug, Clone)]
pub struct TemplateImportArgs {
    /// Path to a template DSL file (JSON5/RON).
    pub path: PathBuf,

    /// Overwrite any existing template with the same id.
    #[arg(long)]
    pub force: bool,
}

#[derive(Args, Debug, Clone)]
pub struct TemplateExportArgs {
    /// Template identifier to export.
    pub id: String,

    /// Choose which registry to query.
    #[arg(long, value_enum, default_value_t = TemplateSourcePriority::Auto)]
    pub source: TemplateSourcePriority,

    /// Destination path or directory (defaults to current directory).
    #[arg(long, value_name = "PATH")]
    pub output: Option<PathBuf>,

    /// Overwrite the output file if it already exists.
    #[arg(long)]
    pub overwrite: bool,
}

#[derive(Copy, Clone, Debug, ValueEnum, PartialEq, Eq)]
pub enum TemplateSourceFilter {
    /// Include both built-in and local templates.
    All,
    /// Built-in templates only.
    Builtin,
    /// Templates imported into the local registry.
    Local,
}

#[derive(Copy, Clone, Debug, ValueEnum, PartialEq, Eq)]
pub enum TemplateSourcePriority {
    /// Prefer local templates but fall back to built-ins.
    Auto,
    /// Built-in registry only.
    Builtin,
    /// Local registry only.
    Local,
}

impl StylePresetArg {
    #[must_use]
    pub fn label(self) -> &'static str {
        match self {
            StylePresetArg::Balanced => "balanced",
            StylePresetArg::SmoothBallad => "smooth_ballad",
            StylePresetArg::GospelDrive => "gospel_drive",
            StylePresetArg::PopRadio => "pop_radio",
        }
    }
}
