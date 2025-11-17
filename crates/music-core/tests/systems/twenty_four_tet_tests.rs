use music_core::{PitchSystem, systems::TwentyFourTET};

#[test]
fn quarter_tone_steps() {
    let tet = TwentyFourTET::a4_440();
    let quarter_up = tet.to_frequency(70);
    let expected = 440.0 * 2.0_f32.powf(1.0 / 24.0);
    assert!((quarter_up - expected).abs() < 1e-6);
}
