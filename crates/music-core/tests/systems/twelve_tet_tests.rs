use music_core::{PitchSystem, systems::TwelveTET};

#[test]
fn doubles_every_twelve_steps() {
    let twelve = TwelveTET::a4_440();
    assert!((twelve.to_frequency(69) - 440.0).abs() < 1e-6);
    assert!((twelve.to_frequency(81) - 880.0).abs() < 1e-6);
}
