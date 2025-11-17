use music_core::{
    AbstractPitch, Pitch, PitchError, PitchLabel, PitchSystemId, TuningRegistry, systems::TwelveTET,
};

#[test]
fn resolves_literal_frequency() {
    let pitch = Pitch::hz(123.4);
    let registry = TuningRegistry::new();
    assert_eq!(pitch.try_freq_hz(&registry).unwrap(), 123.4);
    assert!(matches!(
        pitch.try_label(&registry).unwrap(),
        PitchLabel::Frequency(freq) if (freq - 123.4).abs() < f32::EPSILON
    ));
    assert!(matches!(
        pitch.try_name(&registry),
        Err(PitchError::LiteralHasNoName)
    ));
}

#[test]
fn resolves_abstract_pitch() {
    let mut registry = TuningRegistry::new();
    let id = PitchSystemId::from("12tet");
    registry.register_system(id.clone(), TwelveTET::a4_440());

    let pitch = Pitch::abstract_pitch(69, id.clone());
    assert!((pitch.try_freq_hz(&registry).unwrap() - 440.0).abs() < 1e-6);
    match pitch.name(&registry).unwrap() {
        PitchLabel::Named(label) => assert_eq!(label, "12-TET(69)"),
        other => panic!("unexpected label: {other:?}"),
    }
    assert_eq!(pitch.try_name(&registry).unwrap(), "12-TET(69)".to_string());
}

#[test]
fn reports_missing_system() {
    let registry = TuningRegistry::new();
    let pitch = Pitch::abstract_pitch(0, PitchSystemId::from("missing"));
    match pitch.try_freq_hz(&registry) {
        Err(PitchError::UnknownSystem(id)) => assert_eq!(id, PitchSystemId::from("missing")),
        other => panic!("unexpected result: {other:?}"),
    }
}

#[test]
fn abstract_pitch_arithmetic_helpers() {
    let id = PitchSystemId::from("12tet");
    let pitch = AbstractPitch::new(60, id.clone());
    assert_eq!(pitch.transpose(12).index, 72);
    assert_eq!((pitch.clone() + 1).index, 61);
    assert_eq!((pitch.clone() - 1).index, 59);

    let mut mutable = pitch;
    mutable += 2;
    assert_eq!(mutable.index, 62);
    mutable -= 4;
    assert_eq!(mutable.index, 58);
    let (index, system) = mutable.components();
    assert_eq!(index, 58);
    assert_eq!(system, &id);
}

#[test]
fn pitch_introspection_works() {
    let abstract_pitch = Pitch::abstract_pitch(69, PitchSystemId::from("12"));
    assert!(abstract_pitch.is_abstract());
    assert!(!abstract_pitch.is_frequency());
    assert_eq!(abstract_pitch.index(), Some(69));
    assert_eq!(abstract_pitch.system_id().unwrap().as_str(), "12");
    let transposed = abstract_pitch.transpose(3);
    assert_eq!(transposed.index(), Some(72));

    let literal = Pitch::hz(42.0);
    assert!(literal.is_frequency());
    assert_eq!(literal.as_frequency(), Some(42.0));
    assert!(literal.as_abstract().is_none());
    assert_eq!(literal.transpose(7).as_frequency(), Some(42.0));
    assert_eq!(literal.to_string(), "42.000 Hz");
}

#[test]
fn owned_resolution_and_conversions_work() {
    let mut registry = TuningRegistry::new();
    let id = PitchSystemId::from("12tet");
    registry.register_system(id.clone(), TwelveTET::a4_440());

    let pitch = Pitch::abstract_pitch(69, id.clone());
    let resolved = pitch.clone().into_resolved(&registry).unwrap();
    assert!(resolved.is_frequency());

    let freq = resolved.clone().into_freq_hz(&registry).unwrap();
    assert!((freq - 440.0).abs() < f32::EPSILON);

    let abstract_again = AbstractPitch::try_from(pitch).unwrap();
    assert_eq!(abstract_again.index, 69);
}

#[test]
fn tolerance_helpers_cover_common_use_cases() {
    let mut registry = TuningRegistry::new();
    let id = PitchSystemId::from("12tet");
    registry.register_system(id.clone(), TwelveTET::a4_440());

    let a4 = Pitch::abstract_pitch(69, id.clone());
    let a4_plus_one_cent = Pitch::hz(440.0 * 2.0f32.powf(1.0 / 1200.0));

    assert!(a4.approx_eq(&a4_plus_one_cent, &registry, 0.5).unwrap());

    let cents = a4_plus_one_cent
        .cents_offset(&a4, &registry)
        .expect("offset available");
    assert!(cents > 0.9 && cents < 1.1);
}

#[cfg(feature = "serde")]
#[test]
fn serde_round_trip_retains_data() {
    let pitch = Pitch::abstract_pitch(42, PitchSystemId::from("edo"));
    let json = serde_json::to_string(&pitch).expect("serialize");
    let restored: Pitch = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(restored, pitch);
}
