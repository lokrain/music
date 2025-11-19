#[cfg(feature = "schema")]
use anyhow::Result;
#[cfg(feature = "schema")]
use schemars::schema_for;
#[cfg(feature = "schema")]
use std::fs;
#[cfg(feature = "schema")]
use std::path::Path;

#[cfg(feature = "schema")]
use crate::responses::*;

#[cfg(feature = "schema")]
pub fn export_schemas(output_dir: &Path) -> Result<()> {
    use crate::reports::validation::{
        MelodyValidationReport, ProgressionValidationReport, TuningValidationReport,
    };

    fs::create_dir_all(output_dir)?;

    // Macro to export a schema for a given type
    macro_rules! export_schema {
        ($type:ty, $filename:expr) => {
            let schema = schema_for!($type);
            let json = serde_json::to_string_pretty(&schema)?;
            let path = output_dir.join($filename);
            fs::write(&path, json)?;
            println!("Exported: {}", path.display());
        };
    }

    // Export schemas for all report types
    export_schema!(PitchSummary, "PitchSummary.json");
    export_schema!(PitchExplanation, "PitchExplanation.json");
    export_schema!(PitchContext, "PitchContext.json");

    export_schema!(ChordListing, "ChordListing.json");
    export_schema!(ChordDetails, "ChordDetails.json");
    export_schema!(ChordExplanation, "ChordExplanation.json");

    export_schema!(ScaleExplanation, "ScaleExplanation.json");
    export_schema!(ScaleDegreeSummary, "ScaleDegreeSummary.json");

    export_schema!(SystemsListing, "SystemsListing.json");
    export_schema!(SystemSummary, "SystemSummary.json");

    export_schema!(ModeListing, "ModeListing.json");
    export_schema!(ModeSummary, "ModeSummary.json");

    export_schema!(ReharmListing, "ReharmListing.json");

    export_schema!(MelodyAnalysisReport, "MelodyAnalysisReport.json");
    export_schema!(ChordAnalysisReport, "ChordAnalysisReport.json");

    export_schema!(MelodyDiffReport, "MelodyDiffReport.json");
    export_schema!(ProgressionDiffReport, "ProgressionDiffReport.json");
    export_schema!(MidiDiffReport, "MidiDiffReport.json");

    export_schema!(ScaleMapReport, "ScaleMapReport.json");
    export_schema!(
        InterpolatedEnvelopeReport,
        "InterpolatedEnvelopeReport.json"
    );
    export_schema!(VelocityEnvelopeReport, "VelocityEnvelopeReport.json");

    export_schema!(ScaleSearchReport, "ScaleSearchReport.json");
    export_schema!(ChordSearchReport, "ChordSearchReport.json");

    export_schema!(EstimateReport, "EstimateReport.json");

    export_schema!(ChordResolutionReport, "ChordResolutionReport.json");
    export_schema!(VoiceResolution, "VoiceResolution.json");

    export_schema!(MotifGeneration, "MotifGeneration.json");
    export_schema!(ArpeggioGeneration, "ArpeggioGeneration.json");
    export_schema!(RhythmCellGeneration, "RhythmCellGeneration.json");

    export_schema!(MelodyScoreReport, "MelodyScoreReport.json");
    export_schema!(ProgressionScoreReport, "ProgressionScoreReport.json");
    export_schema!(ChordScoreReport, "ChordScoreReport.json");

    export_schema!(StaffRenderReport, "StaffRenderReport.json");
    export_schema!(PianoRollRenderReport, "PianoRollRenderReport.json");

    export_schema!(MelodyValidationReport, "MelodyValidationReport.json");
    export_schema!(
        ProgressionValidationReport,
        "ProgressionValidationReport.json"
    );
    export_schema!(TuningValidationReport, "TuningValidationReport.json");

    export_schema!(MidiAnalysisReport, "MidiAnalysisReport.json");
    export_schema!(PitchIndexToFrequency, "PitchIndexToFrequency.json");
    export_schema!(FrequencyToIndexReport, "FrequencyToIndexReport.json");
    export_schema!(TemperamentRemapReport, "TemperamentRemapReport.json");
    export_schema!(MidiCsvConversionReport, "MidiCsvConversionReport.json");

    export_schema!(MelodyExtrapolationReport, "MelodyExtrapolationReport.json");
    export_schema!(ChordExtrapolationReport, "ChordExtrapolationReport.json");

    export_schema!(ProfileReport, "ProfileReport.json");

    println!(
        "\n✓ Successfully exported {} schemas to {}",
        41,
        output_dir.display()
    );

    Ok(())
}
