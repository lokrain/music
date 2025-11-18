use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

use crate::{
    format::OutputFormat,
    theory::{ChordVoicing, ScaleKind},
};

#[derive(Parser)]
#[command(
    author,
    version,
    about = "music <command> [options]",
    long_about = "A theory-first toolkit for analysis, generation, and exploration."
)]
pub struct Cli {
    /// Output format for the selected command.
    #[arg(long, value_enum, default_value_t = OutputFormat::Text, global = true)]
    pub format: OutputFormat,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    #[command(about = "Enumerate static musical theory objects")]
    List {
        #[command(subcommand)]
        command: ListCommand,
    },
    #[command(about = "Detailed multi-section report for a musical entity")]
    Inspect {
        #[command(subcommand)]
        command: InspectCommand,
    },
    #[command(about = "Analyze input: key, scale, mode, tensions, function")]
    Analyze {
        #[command(subcommand)]
        command: AnalyzeCommand,
    },
    #[command(about = "Theory-aware suggestions (reharm, modulations, voicings)")]
    Suggest {
        #[command(subcommand)]
        command: SuggestCommand,
    },
    #[command(about = "Explain reasoning behind analysis or musical choices")]
    Explain {
        #[command(subcommand)]
        command: ExplainCommand,
    },
    #[command(about = "Transform musical representations (transpose, spell, roman→chord)")]
    Convert,
    #[command(about = "Check correctness of structures (chords, scales, intervals)")]
    Validate,
    #[command(about = "Generate diagrams/maps (key graph, chord graph)")]
    Render,
    #[command(about = "Show internal registries, tunings, pitch systems, modes")]
    Expose {
        #[command(subcommand)]
        command: ExposeCommand,
    },
    #[command(about = "Create musical material (melodies, progressions, basslines)")]
    Generate,
    #[command(about = "Rate or grade structures (tension, resolution, brightness)")]
    Score,
    #[command(about = "Extend or predict continuation of a musical pattern")]
    Extrapolate,
    #[command(
        name = "explain-diff",
        about = "Compare two objects and highlight differences"
    )]
    ExplainDiff,
    #[command(about = "Produce relational maps (keys, chords, mixture sets)")]
    Map,
    #[command(about = "Style/genre profiling for a key or progression")]
    Profile,
    #[command(about = "Blend between two musical entities")]
    Interpolate,
    #[command(about = "Find objects matching constraints (notes, chords, scales)")]
    Search,
    #[command(about = "Produce heuristic estimates (brightness, instability)")]
    Estimate,
    #[command(about = "Suggest resolution paths for tensions or non-chord tones")]
    Resolve,
}

#[derive(Subcommand)]
pub enum ListCommand {
    #[command(about = "List registered tuning systems and sample frequencies")]
    Systems(SystemsArgs),
    #[command(about = "List diatonic chords for a given root/scale/system")]
    Chords(ChordArgs),
    #[command(about = "Rotate the selected scale through its modes")]
    Modes(ModesArgs),
}

#[derive(Subcommand)]
pub enum InspectCommand {
    #[command(about = "Describe a pitch index within the selected tuning system")]
    Pitch(PitchArgs),
}

#[derive(Subcommand)]
pub enum ExplainCommand {
    #[command(about = "Explain a single pitch inside a tuning system")]
    Pitch(ExplainPitchArgs),
    #[command(about = "Explain a scale and its degrees")]
    Scale(ExplainScaleArgs),
    #[command(about = "Explain a diatonic chord within a scale context")]
    Chord(ExplainChordArgs),
}

#[derive(Subcommand)]
pub enum SuggestCommand {
    #[command(about = "Suggest borrowed chords from parallel modes for reharmonization")]
    Reharm(ReharmArgs),
}

#[derive(Subcommand)]
pub enum AnalyzeCommand {
    #[command(about = "Infer key/scale/mode and note statistics for a melody")]
    Melody(AnalyzeMelodyArgs),
    #[command(about = "Break down chord functions and cadences in a progression")]
    Chords(AnalyzeChordsArgs),
    #[command(about = "Summarize a MIDI file (size, track count placeholder)")]
    Midi(AnalyzeMidiArgs),
}

#[derive(Subcommand)]
pub enum ExposeCommand {
    #[command(about = "Show registered tuning systems and metadata")]
    Tunings(SystemsArgs),
    #[command(about = "Inspect modal rotations including frequency tables")]
    Modes(ModesArgs),
}

#[derive(Args, Clone, Copy)]
pub struct SystemsArgs {
    /// Reference pitch index used to sample each system.
    #[arg(long = "reference-index", default_value_t = 69)]
    pub reference_index: i32,
}

#[derive(Args)]
pub struct PitchArgs {
    /// MIDI-like index (defaults to concert A4 = 69).
    #[arg(short, long, default_value_t = 69)]
    pub index: i32,

    /// Pitch system identifier registered with the engine.
    #[arg(short, long, default_value = "12tet")]
    pub system: String,
}

#[derive(Args)]
pub struct ChordArgs {
    /// Root index used to anchor the scale/chords.
    #[arg(short, long, default_value_t = 60)]
    pub root: i32,

    /// Pitch system identifier registered with the engine.
    #[arg(short, long, default_value = "12tet")]
    pub system: String,

    /// Scale used to derive diatonic chords.
    #[arg(long, value_enum, default_value_t = ScaleKind::Major)]
    pub scale: ScaleKind,

    /// Whether to list triads or seventh chords.
    #[arg(long, value_enum, default_value_t = ChordVoicing::Triads)]
    pub voicing: ChordVoicing,
}

#[derive(Args)]
pub struct ModesArgs {
    /// Root index used to anchor the parent scale.
    #[arg(short, long, default_value_t = 60)]
    pub root: i32,

    /// Pitch system identifier registered with the engine.
    #[arg(short, long, default_value = "12tet")]
    pub system: String,

    /// Scale that will be rotated through its modes.
    #[arg(long, value_enum, default_value_t = ScaleKind::Major)]
    pub scale: ScaleKind,
}

#[derive(Args)]
pub struct ReharmArgs {
    /// Root index used to anchor the parent scale.
    #[arg(short, long, default_value_t = 60)]
    pub root: i32,

    /// Pitch system identifier registered with the engine.
    #[arg(short, long, default_value = "12tet")]
    pub system: String,

    /// Scale whose parallel modes will be evaluated.
    #[arg(long, value_enum, default_value_t = ScaleKind::Major)]
    pub scale: ScaleKind,

    /// Whether to list triads or seventh chords for each mode.
    #[arg(long, value_enum, default_value_t = ChordVoicing::Triads)]
    pub voicing: ChordVoicing,

    /// Optional base-scale degree (1-indexed) to match when filtering borrowed chords.
    #[arg(long)]
    pub degree: Option<usize>,
}

#[derive(Args)]
pub struct AnalyzeMelodyArgs {
    /// Comma-separated list of MIDI-like pitch indices (e.g., 60,62,64,65).
    #[arg(long = "notes", value_delimiter = ',', num_args = 1..)]
    pub notes: Vec<i32>,

    /// Optional context key to bias analysis (e.g., Cmaj, Amin).
    #[arg(long = "in", value_name = "KEY")]
    pub key_hint: Option<String>,

    /// Pitch system identifier registered with the engine.
    #[arg(long, default_value = "12tet")]
    pub system: String,
}

#[derive(Args)]
pub struct AnalyzeChordsArgs {
    /// Comma-separated Roman numerals (e.g., I,vi,ii,V) describing the progression.
    #[arg(long = "progression", value_delimiter = ',', num_args = 1..)]
    pub progression: Vec<String>,

    /// Optional context key to anchor the progression (e.g., Cmaj).
    #[arg(long = "in", value_name = "KEY")]
    pub key_hint: Option<String>,

    /// Pitch system identifier registered with the engine.
    #[arg(long, default_value = "12tet")]
    pub system: String,
}

#[derive(Args)]
pub struct AnalyzeMidiArgs {
    /// Path to a MIDI file to summarize.
    #[arg(long = "file", value_name = "PATH")]
    pub file: PathBuf,

    /// Optional context key for downstream reporting.
    #[arg(long = "in", value_name = "KEY")]
    pub key_hint: Option<String>,

    /// Pitch system identifier registered with the engine (used for note labels).
    #[arg(long, default_value = "12tet")]
    pub system: String,
}

#[derive(Args)]
pub struct ExplainPitchArgs {
    /// Abstract pitch index to explain (default A4).
    #[arg(short, long, default_value_t = 69)]
    pub index: i32,

    /// Pitch system identifier registered with the engine.
    #[arg(short, long, default_value = "12tet")]
    pub system: String,

    /// Optional key context (e.g., Cmaj, Amin) used to describe functional role.
    #[arg(long = "in", value_name = "KEY")]
    pub key_hint: Option<String>,
}

#[derive(Args)]
pub struct ExplainScaleArgs {
    /// Root index used to anchor the scale.
    #[arg(short, long, default_value_t = 60)]
    pub root: i32,

    /// Pitch system identifier registered with the engine.
    #[arg(short, long, default_value = "12tet")]
    pub system: String,

    /// Scale to explain.
    #[arg(long, value_enum, default_value_t = ScaleKind::Major)]
    pub scale: ScaleKind,

    /// Number of degrees to describe (defaults to one diatonic octave).
    #[arg(long, default_value_t = 7)]
    pub degrees: usize,
}

#[derive(Args)]
pub struct ExplainChordArgs {
    /// Root index used to anchor the parent scale.
    #[arg(short, long, default_value_t = 60)]
    pub root: i32,

    /// Pitch system identifier registered with the engine.
    #[arg(short, long, default_value = "12tet")]
    pub system: String,

    /// Scale providing diatonic context for the chord.
    #[arg(long, value_enum, default_value_t = ScaleKind::Major)]
    pub scale: ScaleKind,

    /// Diatonic degree (1-indexed) to explain.
    #[arg(long, default_value_t = 1)]
    pub degree: usize,

    /// Whether to analyze a triad or seventh chord at the selected degree.
    #[arg(long, value_enum, default_value_t = ChordVoicing::Triads)]
    pub voicing: ChordVoicing,
}
