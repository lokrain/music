use music_core::{PitchSystem, system::EqualTemperament};

#[test]
fn equal_temperament_scales_pitches() {
    let et = EqualTemperament::new(12, 440.0, 69).with_label("12-TET");
    assert!((et.to_frequency(69) - 440.0).abs() < 1e-6);
    assert!((et.to_frequency(81) - 880.0).abs() < 1e-6);
    assert_eq!(et.name_of(69), Some("12-TET(69)".to_string()));
}
