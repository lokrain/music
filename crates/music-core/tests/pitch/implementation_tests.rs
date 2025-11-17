use music_core::{
    AbstractPitch, DEFAULT_FREQUENCY_EPSILON, Pitch, PitchError, PitchLabel, PitchSystemId,
};

use super::support::{empty_registry, registered_registry};

mod resolution {
    use super::*;

    #[test]
    fn literal_frequency_passthrough() {
        let registry = empty_registry();
        let pitch = Pitch::hz(123.4);
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
    fn abstract_pitch_resolution_uses_registry() {
        let (registry, system) = registered_registry();
        let pitch = Pitch::abstract_pitch(69, system.clone());

        assert!((pitch.try_freq_hz(&registry).unwrap() - 440.0).abs() < 1e-6);
        match pitch.name(&registry).unwrap() {
            PitchLabel::Named(label) => assert_eq!(label, "12-TET(69)"),
            other => panic!("unexpected label: {other:?}"),
        }
        assert_eq!(pitch.try_name(&registry).unwrap(), "12-TET(69)");
    }

    #[test]
    fn unknown_system_errors_cleanly() {
        let registry = empty_registry();
        let system = PitchSystemId::from("missing");
        let pitch = Pitch::abstract_pitch(0, system.clone());
        match pitch.try_freq_hz(&registry) {
            Err(PitchError::UnknownSystem(id)) => assert_eq!(id, system),
            other => panic!("unexpected result: {other:?}"),
        }
    }

    #[test]
    fn owned_resolution_and_conversions_work() {
        let (registry, system) = registered_registry();
        let pitch = Pitch::abstract_pitch(69, system);

        let resolved = pitch.clone().into_resolved(&registry).unwrap();
        assert!(resolved.is_frequency());

        let freq = resolved.clone().into_freq_hz(&registry).unwrap();
        assert!((freq - 440.0).abs() < f32::EPSILON);

        let abstract_again = AbstractPitch::try_from(pitch).unwrap();
        assert_eq!(abstract_again.index, 69);
    }
}

mod introspection {
    use super::*;

    #[test]
    fn abstract_pitch_metadata_is_exposed() {
        let (registry, system) = registered_registry();
        let pitch = Pitch::abstract_pitch(69, system.clone());

        assert!(pitch.is_abstract());
        assert!(!pitch.is_frequency());
        assert_eq!(pitch.index(), Some(69));
        assert_eq!(pitch.system_id().unwrap(), &system);

        let transposed = pitch.transpose(3);
        assert_eq!(transposed.index(), Some(72));
        assert_eq!(transposed.system_id().unwrap(), &system);

        let label = pitch.try_label(&registry).unwrap();
        assert!(matches!(label, PitchLabel::Named(_)));
    }

    #[test]
    fn literal_pitch_metadata_is_exposed() {
        let pitch = Pitch::hz(42.0);
        assert!(pitch.is_frequency());
        assert!(!pitch.is_abstract());
        assert_eq!(pitch.as_frequency(), Some(42.0));
        assert!(pitch.as_abstract().is_none());
        assert_eq!(pitch.transpose(7).as_frequency(), Some(42.0));
        assert_eq!(pitch.to_string(), "42.000 Hz");
    }

    #[test]
    fn map_abstract_only_touches_abstract_variants() {
        let (registry, system) = registered_registry();
        let literal = Pitch::hz(64.0);
        assert_eq!(
            literal
                .map_abstract(|pitch| pitch.transpose(12))
                .freq_hz(&registry)
                .unwrap(),
            64.0
        );

        let abstract_pitch = Pitch::abstract_pitch(60, system.clone());
        let mapped = abstract_pitch.map_abstract(|p| p.transpose(7));
        assert_eq!(mapped.index(), Some(67));
        assert_eq!(mapped.system_id(), Some(&system));
    }
}

mod intervals_and_tolerance {
    use super::*;

    #[test]
    fn tolerance_helpers_cover_common_use_cases() {
        let (registry, system) = registered_registry();
        let a4 = Pitch::abstract_pitch(69, system);
        let a4_plus_one_cent = Pitch::hz(440.0 * 2.0f32.powf(1.0 / 1200.0));

        assert!(a4.approx_eq(&a4_plus_one_cent, &registry, 0.5).unwrap());

        let cents = a4_plus_one_cent
            .cents_offset(&a4, &registry)
            .expect("offset available");
        assert!(cents > 0.9 && cents < 1.1);
    }

    #[test]
    fn interval_helpers_transpose_as_expected() {
        let (registry, system) = registered_registry();
        let root = Pitch::abstract_pitch(60, system.clone());
        let target = Pitch::abstract_pitch(64, system);

        let interval = root.interval_to(&target, &registry).unwrap();
        let via_try = root
            .try_interval_to(&target, &registry)
            .expect("interval available");
        assert_eq!(interval, via_try);

        let transposed = root.transpose_interval(&interval, &registry).unwrap();
        assert!(
            transposed
                .approx_eq(&target, &registry, DEFAULT_FREQUENCY_EPSILON)
                .unwrap()
        );
    }
}
