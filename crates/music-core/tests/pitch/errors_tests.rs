use music_core::{PitchError, PitchSystemId, TuningError};

#[test]
fn tuning_error_converts_to_pitch_error() {
    let system = PitchSystemId::from("missing");
    let tuning_error = TuningError::UnknownSystem(system.clone());
    let pitch_error = PitchError::from(tuning_error);
    assert!(matches!(pitch_error, PitchError::UnknownSystem(id) if id == system));
}

#[test]
fn display_mentions_literal_frequency() {
    let err = PitchError::InvalidLiteralFrequency(-10.0);
    assert!(format!("{err}").contains("-10"));
}

#[test]
fn name_unavailable_mentions_system_and_index() {
    let err = PitchError::NameUnavailable {
        system: PitchSystemId::from("edo"),
        index: 99,
    };
    let display = err.to_string();
    assert!(display.contains("edo"));
    assert!(display.contains("99"));
}
