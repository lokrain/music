use music_core::prelude::*;

fn registry() -> TuningRegistry {
    TuningRegistry::new().with_system(
        PitchSystemId::try_new("12tet").unwrap(),
        TwelveTET::a4_440(),
    )
}

#[test]
fn interval_between_and_apply_in_12tet() {
    let registry = registry();
    let id = PitchSystemId::try_new("12tet").unwrap();

    let a4 = Pitch::abstract_pitch(69, id.clone());
    let e5 = Pitch::abstract_pitch(76, id.clone());

    let interval = Interval::try_between(&a4, &e5, &registry).unwrap();

    let (steps, system) = interval.steps().expect("steps captured");
    assert_eq!(steps, 7);
    assert_eq!(system.as_str(), id.as_str());

    let transposed = interval.apply_to(&a4, &registry).unwrap();
    let abstract_result = transposed.as_abstract().expect("remains abstract");
    assert_eq!(abstract_result.index, 76);
    assert_eq!(abstract_result.system, id);

    let cents = interval.cents();
    assert!((cents - 700.0).abs() < 2.0);
}

#[test]
fn inverse_and_powi_round_trip() {
    let registry = registry();
    let id = PitchSystemId::try_new("12tet").unwrap();
    let a4 = Pitch::abstract_pitch(69, id.clone());
    let e5 = Pitch::abstract_pitch(76, id.clone());
    let interval = Interval::try_between(&a4, &e5, &registry).unwrap();

    let inverse = interval.inverse().unwrap();
    let back = inverse.apply_to(&e5, &registry).unwrap();
    let abstract_back = back.as_abstract().expect("remains abstract");
    assert_eq!(abstract_back.index, 69);

    let doubled = interval.powi(2).unwrap();
    let up_twice = doubled.apply_to(&a4, &registry).unwrap();
    let cents = up_twice.cents_offset(&a4, &registry).unwrap();
    assert!((cents - 1400.0).abs() < 4.0);
}

#[test]
fn literal_interval_preserves_ratio() {
    let registry = registry();
    let base = Pitch::hz(220.0);
    let target = Pitch::hz(330.0);

    let interval = Interval::try_between(&base, &target, &registry).unwrap();
    assert!(interval.steps().is_none());
    assert!((interval.ratio() - 1.5).abs() < 1e-6);

    let applied = interval.apply_to(&Pitch::hz(440.0), &registry).unwrap();
    assert!((applied.as_frequency().unwrap() - 660.0).abs() < 1e-4);
}

#[test]
fn interval_falls_back_to_literal_in_other_system() {
    let registry = TuningRegistry::new()
        .with_system(PitchSystemId::try_new("12").unwrap(), TwelveTET::a4_440())
        .with_system(
            PitchSystemId::try_new("24").unwrap(),
            TwentyFourTET::a4_440(),
        );
    let twelve = PitchSystemId::try_new("12").unwrap();
    let twenty_four = PitchSystemId::try_new("24").unwrap();

    let base = Pitch::abstract_pitch(60, twelve.clone());
    let target = Pitch::abstract_pitch(67, twelve.clone());
    let interval = Interval::try_between(&base, &target, &registry).unwrap();

    let other = Pitch::abstract_pitch(0, twenty_four.clone());
    let other_freq = other.try_freq_hz(&registry).unwrap();
    let expected = other_freq * interval.ratio();

    let transposed = interval.apply_to(&other, &registry).unwrap();
    assert!(transposed.is_frequency());
    let transposed_freq = transposed.as_frequency().unwrap();
    assert!((transposed_freq - expected).abs() < 1e-4);
}
