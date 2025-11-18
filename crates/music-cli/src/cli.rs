use std::path::PathBuf;

use clap::{Args, Parser, Subcommand, ValueEnum};

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
    Convert {
        #[command(subcommand)]
        command: ConvertCommand,
    },
    #[command(about = "Check correctness of structures (chords, scales, intervals)")]
    Validate {
        #[command(subcommand)]
        command: ValidateCommand,
    },
    #[command(about = "Render visual representations (staff notation, piano roll)")]
    Render {
        #[command(subcommand)]
        command: RenderCommand,
    },
    #[command(about = "Show internal registries, tunings, pitch systems, modes")]
    Expose {
        #[command(subcommand)]
        command: ExposeCommand,
    },
    #[command(about = "Create musical material (motifs, arpeggios, rhythm cells)")]
    Generate {
        #[command(subcommand)]
        command: GenerateCommand,
    },
    #[command(about = "Rate or grade structures (tension, resolution, brightness)")]
    Score {
        #[command(subcommand)]
        command: ScoreCommand,
    },
    #[command(about = "Extend or predict continuation of a musical pattern")]
    Extrapolate {
        #[command(subcommand)]
        command: ExtrapolateCommand,
    },
    #[command(
        name = "explain-diff",
        about = "Compare two objects and highlight differences"
    )]
    ExplainDiff {
        #[command(subcommand)]
        command: ExplainDiffCommand,
    },
    #[command(about = "Produce relational maps (keys, chords, mixture sets)")]
    Map {
        #[command(subcommand)]
        command: MapCommand,
    },
    #[command(about = "Profile timing, density, and register usage of musical inputs")]
    Profile {
        #[command(subcommand)]
        command: ProfileCommand,
    },
    #[command(about = "Blend between two musical entities")]
    Interpolate {
        #[command(subcommand)]
        command: InterpolateCommand,
    },
    #[command(about = "Find objects matching constraints (notes, chords, scales)")]
    Search {
        #[command(subcommand)]
        command: SearchCommand,
    },
    #[command(about = "Estimate tempo, key, and meter from musical input")]
    Estimate {
        #[command(subcommand)]
        command: EstimateCommand,
    },
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
pub enum GenerateCommand {
    #[command(about = "Generate a short motif within the selected scale")]
    Motif(GenerateMotifArgs),
    #[command(about = "Generate an arpeggio pattern anchored to the scale")]
    Arpeggio(GenerateArpeggioArgs),
    #[command(about = "Generate a rhythm cell sized for the density")]
    Rhythm(GenerateRhythmArgs),
}

#[derive(Subcommand)]
pub enum MapCommand {
    #[command(about = "Render a pitch-class map for a scale in a system")]
    Scale(MapScaleArgs),
}

#[derive(Subcommand)]
pub enum ProfileCommand {
    #[command(about = "Profile a melody sequence (pitch indices only)")]
    Melody(ProfileMelodyArgs),
    #[command(about = "Profile a MIDI file with timing information")]
    Midi(ProfileMidiArgs),
}

#[derive(Subcommand)]
pub enum InterpolateCommand {
    #[command(about = "Interpolate tempo envelope between anchor points")]
    Tempo(InterpolateTempoArgs),
    #[command(about = "Interpolate velocity/expression envelopes")]
    Velocity(InterpolateVelocityArgs),
}

#[derive(Subcommand)]
pub enum SearchCommand {
    #[command(about = "Find scales containing the provided notes")]
    Scale(SearchScaleArgs),
    #[command(about = "Find chords containing the provided notes")]
    Chord(SearchChordArgs),
}

#[derive(Subcommand)]
pub enum EstimateCommand {
    #[command(about = "Estimate key from a melody sequence")]
    Melody(EstimateMelodyArgs),
    #[command(about = "Estimate tempo, key, and meter from a MIDI file")]
    Midi(EstimateMidiArgs),
}

#[derive(Subcommand)]
pub enum ExplainDiffCommand {
    #[command(about = "Compare two melodies via pitch-class histograms")]
    Melody(ExplainDiffMelodyArgs),
    #[command(about = "Compare two Roman-numeral progressions")]
    Progression(ExplainDiffProgressionArgs),
    #[command(about = "Compare two MIDI files via header metadata")]
    Midi(ExplainDiffMidiArgs),
}

#[derive(Subcommand)]
pub enum ScoreCommand {
    #[command(about = "Score functional strength and cadence weight")]
    Progression(ScoreProgressionArgs),
    #[command(about = "Score melodic tension density and resolution")]
    Melody(ScoreMelodyArgs),
    #[command(about = "Score chord color/tension profile")]
    Chord(ScoreChordArgs),
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
pub enum ConvertCommand {
    #[command(about = "Convert pitch indices and literal frequencies")]
    Pitch {
        #[command(subcommand)]
        command: PitchConvertCommand,
    },
    #[command(about = "Convert MIDI files and CSV note tables")]
    Midi {
        #[command(subcommand)]
        command: MidiConvertCommand,
    },
    #[command(about = "Remap note indices between temperaments")]
    Temperament(ConvertTemperamentArgs),
}

#[derive(Subcommand)]
pub enum ValidateCommand {
    #[command(about = "Validate a melody against a scale and interval constraints")]
    Melody(ValidateMelodyArgs),
    #[command(about = "Validate a Roman numeral progression for syntax and flow")]
    Progression(ValidateProgressionArgs),
    #[command(about = "Validate tuning system registration and sample indices")]
    Tuning(ValidateTuningArgs),
}

#[derive(Subcommand)]
pub enum RenderCommand {
    #[command(about = "Render a staff notation for a melody or note sequence")]
    Staff(RenderStaffArgs),
    #[command(about = "Render a piano roll visualization for a note sequence")]
    PianoRoll(RenderPianoRollArgs),
}

#[derive(Subcommand)]
pub enum PitchConvertCommand {
    #[command(about = "Resolve an index to frequency and label")]
    ToFrequency(ConvertPitchArgs),
    #[command(about = "Find the closest pitch index for a frequency")]
    ToIndex(ConvertFrequencyArgs),
}

#[derive(Subcommand)]
pub enum MidiConvertCommand {
    #[command(about = "Flatten a MIDI file into a CSV note table")]
    ToCsv(ConvertMidiToCsvArgs),
    #[command(about = "Rebuild a MIDI file from a CSV note table")]
    FromCsv(ConvertCsvToMidiArgs),
}

#[derive(Subcommand)]
pub enum SuggestCommand {
    #[command(about = "Suggest borrowed chords from parallel modes for reharmonization")]
    Reharm(ReharmArgs),
}

#[derive(Subcommand)]
pub enum ExtrapolateCommand {
    #[command(about = "Predict likely melody continuations using n-gram analysis")]
    Melody(ExtrapolateMelodyArgs),
    #[command(about = "Predict likely chord progressions using transition models")]
    Chords(ExtrapolateChordsArgs),
}

#[derive(Clone, Copy, Debug, ValueEnum, PartialEq, Eq)]
pub enum PatternDensity {
    Sparse,
    Balanced,
    Dense,
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
pub struct ScoreProgressionArgs {
    /// Comma-separated Roman numerals (e.g., I,vi,ii,V) describing the progression.
    #[arg(long = "progression", value_delimiter = ',', num_args = 1..)]
    pub progression: Vec<String>,

    /// Optional key hint (e.g., Cmaj) surfaced with commentary.
    #[arg(long = "in", value_name = "KEY")]
    pub key_hint: Option<String>,
}

#[derive(Args)]
pub struct ScoreMelodyArgs {
    /// Comma-separated list of MIDI-like pitch indices (e.g., 60,62,64,65).
    #[arg(long = "notes", value_delimiter = ',', num_args = 1..)]
    pub notes: Vec<i32>,

    /// Optional context key to bias commentary (e.g., Cmaj, Amin).
    #[arg(long = "in", value_name = "KEY")]
    pub key_hint: Option<String>,

    /// Pitch system identifier registered with the engine for labeling.
    #[arg(long, default_value = "12tet")]
    pub system: String,
}

#[derive(Args)]
pub struct ScoreChordArgs {
    /// Comma-separated list of MIDI-like pitch indices (root to top).
    #[arg(long = "notes", value_delimiter = ',', num_args = 1..)]
    pub notes: Vec<i32>,

    /// Pitch system identifier registered with the engine for labeling.
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
pub struct ExplainDiffMelodyArgs {
    /// Left-hand melody as comma-separated MIDI-like indices.
    #[arg(long = "left-notes", value_delimiter = ',', num_args = 1..)]
    pub left_notes: Vec<i32>,

    /// Right-hand melody as comma-separated MIDI-like indices.
    #[arg(long = "right-notes", value_delimiter = ',', num_args = 1..)]
    pub right_notes: Vec<i32>,

    /// Optional context key surfaced in commentary (e.g., Cmaj).
    #[arg(long = "in", value_name = "KEY")]
    pub key_hint: Option<String>,

    /// Pitch system identifier registered with the engine.
    #[arg(long, default_value = "12tet")]
    pub system: String,
}

#[derive(Args)]
pub struct ExplainDiffProgressionArgs {
    /// Left-hand Roman numeral progression.
    #[arg(long = "left", value_delimiter = ',', num_args = 1..)]
    pub left: Vec<String>,

    /// Right-hand Roman numeral progression.
    #[arg(long = "right", value_delimiter = ',', num_args = 1..)]
    pub right: Vec<String>,

    /// Optional context key associated with both progressions.
    #[arg(long = "in", value_name = "KEY")]
    pub key_hint: Option<String>,
}

#[derive(Args)]
pub struct ExplainDiffMidiArgs {
    /// Left-hand MIDI file path.
    #[arg(long = "left-file", value_name = "PATH")]
    pub left_file: PathBuf,

    /// Right-hand MIDI file path.
    #[arg(long = "right-file", value_name = "PATH")]
    pub right_file: PathBuf,
}

#[derive(Args)]
pub struct MapScaleArgs {
    /// Root index anchoring the map (e.g., 60 for middle C).
    #[arg(short, long, default_value_t = 60)]
    pub root: i32,

    /// Pitch system identifier registered with the engine.
    #[arg(short, long, default_value = "12tet")]
    pub system: String,

    /// Scale used to populate the pitch-class map.
    #[arg(long, value_enum, default_value_t = ScaleKind::Major)]
    pub scale: ScaleKind,

    /// Number of modal rotations to highlight as modulation candidates.
    #[arg(long = "modulations", default_value_t = 2)]
    pub modulations: usize,
}

#[derive(Args, Clone)]
pub struct InterpolateCommonArgs {
    /// Comma-separated time:value anchor points (e.g., 0:120,4:140).
    #[arg(long = "points", value_delimiter = ',')]
    pub points: Vec<String>,

    /// Number of interpolated samples between the anchors.
    #[arg(long = "samples", default_value_t = 8)]
    pub samples: usize,

    /// Interpolation curve.
    #[arg(long, value_enum, default_value_t = InterpolationCurve::Linear)]
    pub curve: InterpolationCurve,
}

#[derive(Args)]
pub struct SearchScaleArgs {
    /// Notes or pitch indices whose pitch classes must be present.
    #[arg(long = "notes", value_delimiter = ',', num_args = 1..)]
    pub notes: Vec<i32>,

    /// Pitch system identifier used for labeling.
    #[arg(long, default_value = "12tet")]
    pub system: String,

    /// Maximum number of matches to display.
    #[arg(long = "limit", default_value_t = 12)]
    pub limit: usize,
}

#[derive(Args)]
pub struct SearchChordArgs {
    /// Notes or pitch indices whose pitch classes must be present.
    #[arg(long = "notes", value_delimiter = ',', num_args = 1..)]
    pub notes: Vec<i32>,

    /// Pitch system identifier used for labeling.
    #[arg(long, default_value = "12tet")]
    pub system: String,

    /// Whether to search triads or seventh chords.
    #[arg(long, value_enum, default_value_t = ChordVoicing::Triads)]
    pub voicing: ChordVoicing,

    /// Maximum number of matches to display.
    #[arg(long = "limit", default_value_t = 12)]
    pub limit: usize,
}

#[derive(Clone, Copy, Debug, ValueEnum)]
pub enum InterpolationCurve {
    Linear,
    EaseIn,
    EaseOut,
    EaseInOut,
}

#[derive(Args, Clone)]
pub struct InterpolateTempoArgs {
    #[command(flatten)]
    pub common: InterpolateCommonArgs,

    /// Units for value column (BPM or relative multiplier).
    #[arg(long, value_enum, default_value_t = TempoUnit::Bpm)]
    pub unit: TempoUnit,
}

#[derive(Clone, Copy, Debug, ValueEnum)]
pub enum TempoUnit {
    Bpm,
    Multiplier,
}

#[derive(Args, Clone)]
pub struct InterpolateVelocityArgs {
    #[command(flatten)]
    pub common: InterpolateCommonArgs,

    /// Clamp velocity range (default MIDI 0-127).
    #[arg(long = "min", default_value_t = 0)]
    pub min_value: i32,

    #[arg(long = "max", default_value_t = 127)]
    pub max_value: i32,
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

#[derive(Args)]
pub struct ConvertPitchArgs {
    /// Pitch index to resolve (defaults to concert A4 = 69).
    #[arg(short, long, default_value_t = 69)]
    pub index: i32,

    /// Pitch system identifier registered with the engine.
    #[arg(short, long, default_value = "12tet")]
    pub system: String,
}

#[derive(Args)]
pub struct ConvertFrequencyArgs {
    /// Literal frequency in Hz to match.
    #[arg(long = "frequency", value_name = "HZ")]
    pub frequency_hz: f32,

    /// Pitch system identifier used when searching for the closest index.
    #[arg(short, long, default_value = "12tet")]
    pub system: String,

    /// Center index used when scanning for the best match.
    #[arg(long = "center", default_value_t = 69)]
    pub center: i32,

    /// Number of semitone steps (in each direction) inspected during the search.
    #[arg(long = "search-span", default_value_t = 96)]
    pub search_span: i32,
}

#[derive(Args)]
pub struct ConvertMidiToCsvArgs {
    /// Path to the MIDI file to flatten.
    #[arg(long = "file", value_name = "PATH")]
    pub file: PathBuf,

    /// Optional limit on the number of note rows to emit.
    #[arg(long = "max-rows")]
    pub max_rows: Option<usize>,
}

#[derive(Args)]
pub struct ConvertCsvToMidiArgs {
    /// Path to the CSV note table.
    #[arg(long = "csv", value_name = "PATH")]
    pub csv: PathBuf,

    /// Output MIDI file that will be written.
    #[arg(long = "out", value_name = "PATH")]
    pub out: PathBuf,

    /// Pulses-per-quarter-note resolution of the generated file.
    #[arg(long = "ticks-per-quarter", default_value_t = 480)]
    pub ticks_per_quarter: u16,
}

#[derive(Args)]
pub struct ConvertTemperamentArgs {
    /// Comma-separated list of pitch indices to remap.
    #[arg(long = "indices", value_delimiter = ',', num_args = 1..)]
    pub indices: Vec<i32>,

    /// Source pitch system identifier.
    #[arg(long = "from", default_value = "12tet")]
    pub from_system: String,

    /// Target pitch system identifier.
    #[arg(long = "to", default_value = "24tet")]
    pub to_system: String,

    /// Number of semitone steps (in each direction) to inspect when resolving targets.
    #[arg(long = "search-span", default_value_t = 96)]
    pub search_span: i32,
}

#[derive(Args)]
pub struct ValidateMelodyArgs {
    /// Comma-separated list of MIDI-like pitch indices (e.g., 60,62,64).
    #[arg(long = "notes", value_delimiter = ',', num_args = 1..)]
    pub notes: Vec<i32>,

    /// Pitch system identifier registered with the engine.
    #[arg(short, long, default_value = "12tet")]
    pub system: String,

    /// Root index anchoring the validation scale.
    #[arg(short, long, default_value_t = 60)]
    pub root: i32,

    /// Scale used for validation.
    #[arg(long, value_enum, default_value_t = ScaleKind::Major)]
    pub scale: ScaleKind,

    /// Maximum allowed melodic interval (in semitones) between adjacent notes.
    #[arg(long = "max-interval", default_value_t = 12)]
    pub max_interval: i32,
}

#[derive(Args)]
pub struct ValidateProgressionArgs {
    /// Comma-separated Roman numerals (e.g., I,vi,ii,V) describing the progression.
    #[arg(long = "progression", value_delimiter = ',', num_args = 1..)]
    pub progression: Vec<String>,

    /// Optional key hint (e.g., Cmaj) surfaced with diagnostics.
    #[arg(long = "in", value_name = "KEY")]
    pub key_hint: Option<String>,
}

#[derive(Args)]
pub struct ValidateTuningArgs {
    /// Pitch system identifier registered with the engine.
    #[arg(short, long, default_value = "12tet")]
    pub system: String,

    /// Comma-separated pitch indices probed for consistency.
    #[arg(
        long = "indices",
        value_delimiter = ',',
        num_args = 1..,
        default_values_t = [60, 61, 69, 72]
    )]
    pub indices: Vec<i32>,
}

#[derive(Args)]
pub struct RenderStaffArgs {
    /// Comma-separated list of MIDI-like pitch indices (e.g., 60,62,64,65,67).
    #[arg(long = "notes", value_delimiter = ',', num_args = 1..)]
    pub notes: Vec<i32>,

    /// Pitch system identifier registered with the engine.
    #[arg(short, long, default_value = "12tet")]
    pub system: String,

    /// Whether to use Unicode box-drawing characters for the staff.
    #[arg(long = "unicode", default_value_t = false)]
    pub unicode: bool,

    /// Optional key context (e.g., Cmaj) for accidental rendering.
    #[arg(long = "in", value_name = "KEY")]
    pub key_hint: Option<String>,
}

#[derive(Args)]
pub struct RenderPianoRollArgs {
    /// Comma-separated list of MIDI-like pitch indices (e.g., 60,62,64).
    #[arg(long = "notes", value_delimiter = ',', num_args = 1..)]
    pub notes: Vec<i32>,

    /// Pitch system identifier registered with the engine.
    #[arg(short, long, default_value = "12tet")]
    pub system: String,

    /// Width of the piano roll in character columns.
    #[arg(long = "width", default_value_t = 60)]
    pub width: usize,

    /// Number of vertical pitch steps to display (centered on note range).
    #[arg(long = "height", default_value_t = 24)]
    pub height: usize,

    /// Whether to use Unicode box-drawing characters for the grid.
    #[arg(long = "unicode", default_value_t = false)]
    pub unicode: bool,
}

#[derive(Args, Clone)]
pub struct GenerateCommonArgs {
    /// Root index anchoring the generated material.
    #[arg(short, long, default_value_t = 60)]
    pub root: i32,

    /// Pitch system identifier registered with the engine.
    #[arg(short, long, default_value = "12tet")]
    pub system: String,

    /// Scale used to derive pitch material.
    #[arg(long, value_enum, default_value_t = ScaleKind::Major)]
    pub scale: ScaleKind,

    /// Density of generated events (sparse → dense).
    #[arg(long, value_enum, default_value_t = PatternDensity::Balanced)]
    pub density: PatternDensity,
}

#[derive(Args, Clone)]
pub struct GenerateMotifArgs {
    #[command(flatten)]
    pub common: GenerateCommonArgs,
}

#[derive(Args, Clone)]
pub struct GenerateArpeggioArgs {
    #[command(flatten)]
    pub common: GenerateCommonArgs,
}

#[derive(Args, Clone)]
pub struct GenerateRhythmArgs {
    #[command(flatten)]
    pub common: GenerateCommonArgs,
}

#[derive(Args)]
pub struct ExtrapolateMelodyArgs {
    /// Comma-separated list of MIDI-like pitch indices forming the input sequence.
    #[arg(long = "notes", value_delimiter = ',', num_args = 1..)]
    pub notes: Vec<i32>,

    /// Pitch system identifier registered with the engine.
    #[arg(short, long, default_value = "12tet")]
    pub system: String,

    /// N-gram order (1=bigram, 2=trigram, etc.) for transition model.
    #[arg(long = "order", default_value_t = 2)]
    pub order: usize,

    /// Number of predictions to generate.
    #[arg(long = "count", default_value_t = 5)]
    pub count: usize,

    /// Optional key context (e.g., Cmaj, Amin) to guide predictions.
    #[arg(long = "in", value_name = "KEY")]
    pub key_hint: Option<String>,
}

#[derive(Args)]
pub struct ExtrapolateChordsArgs {
    /// Comma-separated Roman numerals forming the input progression.
    #[arg(long = "progression", value_delimiter = ',', num_args = 1..)]
    pub progression: Vec<String>,

    /// N-gram order (1=bigram, 2=trigram, etc.) for transition model.
    #[arg(long = "order", default_value_t = 1)]
    pub order: usize,

    /// Number of predictions to generate.
    #[arg(long = "count", default_value_t = 5)]
    pub count: usize,

    /// Optional key context for reporting.
    #[arg(long = "in", value_name = "KEY")]
    pub key_hint: Option<String>,
}

#[derive(Args)]
pub struct ProfileMelodyArgs {
    /// Comma-separated list of MIDI-like pitch indices to profile.
    #[arg(long = "notes", value_delimiter = ',', num_args = 1..)]
    pub notes: Vec<i32>,

    /// Pitch system identifier (for context, not used in profiling).
    #[arg(short, long, default_value = "12tet")]
    pub system: String,
}

#[derive(Args)]
pub struct ProfileMidiArgs {
    /// Path to MIDI file to profile.
    #[arg(long = "file")]
    pub file: std::path::PathBuf,

    /// Pitch system identifier (for context, not used in profiling).
    #[arg(short, long, default_value = "12tet")]
    pub system: String,
}

#[derive(Args)]
pub struct EstimateMelodyArgs {
    /// Comma-separated list of MIDI-like pitch indices to analyze.
    #[arg(long = "notes", value_delimiter = ',', num_args = 1..)]
    pub notes: Vec<i32>,

    /// Pitch system identifier (for context).
    #[arg(short, long, default_value = "12tet")]
    pub system: String,
}

#[derive(Args)]
pub struct EstimateMidiArgs {
    /// Path to MIDI file to analyze.
    #[arg(long = "file")]
    pub file: std::path::PathBuf,

    /// Pitch system identifier (for context).
    #[arg(short, long, default_value = "12tet")]
    pub system: String,
}
