use music_score::planner::{ExplainMode, StyleProfile};

use crate::args::StylePresetArg;

pub fn profile_for_preset(preset: StylePresetArg, explain: ExplainMode) -> StyleProfile {
    let mut profile = match preset {
        StylePresetArg::Balanced => StyleProfile::default(),
        StylePresetArg::SmoothBallad => StyleProfile {
            beam_width: 5,
            max_depth: 14,
            risk_level: 0.3,
            reharm_depth: 0.35,
            voice_leading_strictness: 0.9,
            modulation_aggressiveness: 0.25,
            max_chord_complexity: 0.5,
            explain_mode: ExplainMode::None,
        },
        StylePresetArg::GospelDrive => StyleProfile {
            beam_width: 8,
            max_depth: 20,
            risk_level: 0.75,
            reharm_depth: 0.85,
            voice_leading_strictness: 0.55,
            modulation_aggressiveness: 0.7,
            max_chord_complexity: 0.8,
            explain_mode: ExplainMode::None,
        },
        StylePresetArg::PopRadio => StyleProfile {
            beam_width: 7,
            max_depth: 18,
            risk_level: 0.5,
            reharm_depth: 0.6,
            voice_leading_strictness: 0.65,
            modulation_aggressiveness: 0.5,
            max_chord_complexity: 0.65,
            explain_mode: ExplainMode::None,
        },
    };
    profile.explain_mode = explain;
    profile
}
