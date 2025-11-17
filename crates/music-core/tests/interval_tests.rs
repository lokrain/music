use music_core::{
    interval::{Interval, IntervalBetweenError, IntervalError},
    pitch::Pitch,
    registry::TuningRegistry,
    system::{PitchSystem, PitchSystemId},
    systems::{TwelveTET, TwentyFourTET},
};

fn registry() -> TuningRegistry {
    TuningRegistry::new()
        .with_system(PitchSystemId::from("12"), TwelveTET::a4_440())
        .with_system(PitchSystemId::from("24"), TwentyFourTET::a4_440())
}

#[test]
fn intervals_preserve_steps_within_same_system() {
    let registry = registry();
    let system = PitchSystemId::from("12");
    let base = Pitch::abstract_pitch(60, system.clone());
    let target = Pitch::abstract_pitch(64, system.clone());

    let interval = Interval::between(&base, &target, &registry).unwrap();
    let (delta, captured_system) = interval.steps().expect("steps recorded");
    assert_eq!(delta, 4);
    assert_eq!(captured_system.as_str(), system.as_str());

    let transposed = interval.apply_to(&base, &registry).unwrap();
    assert_eq!(transposed, target);
}

#[test]
fn intervals_fall_back_to_frequencies_across_systems() {
    let registry = registry();
    let base = Pitch::abstract_pitch(60, PitchSystemId::from("12"));
    let target = Pitch::abstract_pitch(67, PitchSystemId::from("12"));
    let other = Pitch::abstract_pitch(0, PitchSystemId::from("24"));

    let interval = Interval::between(&base, &target, &registry).unwrap();

    let other_freq = other.try_freq_hz(&registry).unwrap();
    let expected = other_freq * interval.ratio();

    let transposed = interval.apply_to(&other, &registry).unwrap();
    let transposed_freq = transposed.try_freq_hz(&registry).unwrap();
    assert!((transposed_freq - expected).abs() < 1e-4);
}

#[test]
fn pitch_convenience_methods_round_trip() {
    let registry = registry();
    let system = PitchSystemId::from("12");
    let base = Pitch::abstract_pitch(60, system.clone());
    let target = Pitch::abstract_pitch(63, system.clone());

    let interval = base.interval_to(&target, &registry).unwrap();
    let via_try = base.try_interval_to(&target, &registry).unwrap();
    assert_eq!(via_try.ratio(), interval.ratio());
    let back = base.transpose_interval(&interval, &registry).unwrap();
    assert_eq!(back, target);
}

#[test]
fn from_ratio_validates_inputs() {
    assert!(matches!(
        Interval::from_ratio(f32::NAN),
        Err(IntervalError::NonFiniteRatio(_))
    ));
    assert!(matches!(
        Interval::from_ratio(0.0),
        Err(IntervalError::NonPositiveRatio(_))
    ));
    assert!(Interval::from_ratio(2.0).is_ok());
}

#[test]
fn cents_representation_matches_ratio() {
    let interval = Interval::from_ratio(2.0).unwrap();
    assert!((interval.cents() - 1200.0).abs() < 1e-6);
}

#[test]
fn powi_scales_ratio_and_steps() {
    let registry = registry();
    let system = PitchSystemId::from("12");
    let base = Pitch::abstract_pitch(60, system.clone());
    let target = Pitch::abstract_pitch(62, system.clone());
    let interval = Interval::between(&base, &target, &registry).unwrap();

    let doubled = interval.powi(2).expect("valid ratio remains finite");
    assert!((doubled.ratio() - interval.ratio().powi(2)).abs() < 1e-6);
    let (steps, _) = doubled.steps().expect("steps preserved");
    assert_eq!(steps, 4);

    let inverted = interval.powi(-1).expect("inverse ratio remains finite");
    assert!((inverted.ratio() - interval.ratio().powi(-1)).abs() < 1e-6);
    let (inverted_steps, _) = inverted.steps().expect("steps preserved");
    assert_eq!(inverted_steps, -2);
}

#[test]
fn powi_reports_invalid_ratios() {
    let interval = Interval::from_ratio(f32::MAX).unwrap();
    let err = interval.powi(2).expect_err("overflow should error");
    assert!(matches!(err, IntervalError::NonFiniteRatio(_)));
}

#[test]
fn compose_accumulates_ratio_and_steps() {
    let registry = registry();
    let system = PitchSystemId::from("12");
    let whole = Interval::between(
        &Pitch::abstract_pitch(60, system.clone()),
        &Pitch::abstract_pitch(62, system.clone()),
        &registry,
    )
    .unwrap();
    let half = Interval::between(
        &Pitch::abstract_pitch(62, system.clone()),
        &Pitch::abstract_pitch(63, system.clone()),
        &registry,
    )
    .unwrap();

    let composed = whole.compose(&half).expect("composition succeeds");
    assert!((composed.ratio() - whole.ratio() * half.ratio()).abs() < 1e-6);
    let (steps, captured_system) = composed.steps().expect("steps tracked");
    assert_eq!(steps, 3);
    assert_eq!(captured_system.as_str(), system.as_str());
}

#[test]
fn inverse_reports_invalid_ratios() {
    let interval = Interval::from_ratio(f32::from_bits(1)).unwrap();
    let err = interval.inverse().expect_err("underflow should error");
    assert!(matches!(err, IntervalError::NonFiniteRatio(_)));
}

#[test]
fn display_shows_ratio_and_steps() {
    let registry = registry();
    let system = PitchSystemId::from("12");
    let base = Pitch::abstract_pitch(60, system.clone());
    let target = Pitch::abstract_pitch(64, system.clone());
    let interval = Interval::between(&base, &target, &registry).unwrap();
    let rendered = format!("{interval}");
    assert!(rendered.contains("ratio="));
    assert!(rendered.contains("steps=4@12"));

    let literal = Interval::from_ratio(1.5).unwrap();
    let literal_rendered = format!("{literal}");
    assert!(literal_rendered.contains("ratio=1.500000"));
    assert!(!literal_rendered.contains("steps"));
}

#[test]
fn convenience_helpers_follow_result_behavior() {
    let registry = registry();
    let system = PitchSystemId::from("12");
    let base = Pitch::abstract_pitch(60, system.clone());
    let target = Pitch::abstract_pitch(67, system.clone());
    let interval = Interval::between(&base, &target, &registry).unwrap();

    let inverse_result = interval.inverse();
    assert_eq!(interval.inverse_if_valid(), inverse_result.ok());

    let pow_result = interval.powi(2);
    assert_eq!(interval.powi_if_valid(2), pow_result.ok());

    let overflow = Interval::from_ratio(f32::MAX).unwrap();
    assert!(overflow.powi_if_valid(2).is_none());
    let subnormal = Interval::from_ratio(f32::from_bits(1)).unwrap();
    assert!(subnormal.inverse_if_valid().is_none());
}

#[derive(Debug)]
struct DegenerateSystem;

impl PitchSystem for DegenerateSystem {
    fn to_frequency(&self, index: i32) -> f32 {
        if index == 0 { 1.0 } else { f32::INFINITY }
    }
}

#[test]
fn try_between_surface_interval_errors() {
    let system = PitchSystemId::from("degenerate");
    let registry = TuningRegistry::new().with_system(system.clone(), DegenerateSystem);
    let base = Pitch::abstract_pitch(0, system.clone());
    let target = Pitch::abstract_pitch(1, system.clone());

    let err = Interval::try_between(&base, &target, &registry).expect_err("ratio invalid");
    let IntervalBetweenError::Interval { source, .. } = err else {
        panic!("expected interval error");
    };
    assert!(matches!(source, IntervalError::NonFiniteRatio(_)));
}

#[test]
fn pitch_try_interval_to_surface_interval_errors() {
    let system = PitchSystemId::from("degenerate");
    let registry = TuningRegistry::new().with_system(system.clone(), DegenerateSystem);
    let base = Pitch::abstract_pitch(0, system.clone());
    let target = Pitch::abstract_pitch(1, system.clone());

    let err = base
        .try_interval_to(&target, &registry)
        .expect_err("ratio invalid");
    assert!(matches!(err, IntervalBetweenError::Interval { .. }));
}
