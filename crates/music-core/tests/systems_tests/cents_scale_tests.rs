use music_core::{systems::CentsScale, PitchSystem};

#[test]
fn quarter_tone_matches_expected_ratio() {
    let cents = CentsScale::a4_440_quarter_tone();
    let ratio = 2.0_f32.powf(50.0 / 1200.0);
    assert!((cents.to_frequency(69) - 440.0).abs() < 1e-6);
    assert!((cents.to_frequency(70) - 440.0 * ratio).abs() < 1e-6);
}

#[test]
fn label_propagates() {
    let cents = CentsScale::a4_440_quarter_tone();
    assert_eq!(cents.name_of(69).as_deref(), Some("24-EDO(69)"));
}