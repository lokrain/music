use music_core::{PitchSystem, systems::JustIntonation};

#[test]
fn major_third_uses_just_ratio() {
    let ji = JustIntonation::a4_440_major();
    let expected = 440.0 * (5.0 / 4.0);
    assert!((ji.to_frequency(69) - 440.0).abs() < 1e-6);
    assert!((ji.to_frequency(73) - expected).abs() < 1e-6);
}

#[test]
fn label_is_present() {
    let ji = JustIntonation::a4_440_major();
    assert_eq!(ji.name_of(69).as_deref(), Some("JI-major(69)"));
}
