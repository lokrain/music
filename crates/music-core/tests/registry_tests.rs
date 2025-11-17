use music_core::{
    PitchSystemId, PitchSystemIdError, RegistryInsertError, TuningError, TuningRegistry,
    systems::TwelveTET,
};
use std::{cell::Cell, str::FromStr, sync::Arc};

#[test]
fn resolves_frequency_and_name() {
    let mut registry = TuningRegistry::new();
    let id = PitchSystemId::from("12tet");
    registry.register_system(id.clone(), TwelveTET::a4_440());

    assert!((registry.resolve_frequency(&id, 69).unwrap() - 440.0).abs() < 1e-6);
    assert_eq!(
        registry.resolve_name(&id, 69).unwrap(),
        Some("12-TET(69)".to_string())
    );
}

#[test]
fn reports_unknown_system() {
    let registry = TuningRegistry::new();
    let id = PitchSystemId::from("missing");
    let err = registry.resolve_frequency(&id, 0).unwrap_err();
    assert!(matches!(err, TuningError::UnknownSystem(_)));
}

#[test]
fn builder_and_remove_helpers() {
    let mut registry = TuningRegistry::new()
        .with_system("12", TwelveTET::a4_440())
        .with_system("other", TwelveTET::new(432.0, 69));

    assert_eq!(registry.len(), 2);
    assert!(!registry.is_empty());
    assert!(registry.contains(&PitchSystemId::from("12")));
    assert!(registry.contains_str("12"));
    assert!(registry.get_str("other").is_some());

    assert!(registry.register_if_absent("24", TwelveTET::a4_440()));
    assert!(!registry.register_if_absent("24", TwelveTET::a4_440()));

    let removed = registry.remove_str("other");
    assert!(removed.is_some());
    assert_eq!(registry.len(), 2);

    registry.clear();
    assert!(registry.is_empty());
}

#[test]
fn pitch_system_id_validation_and_parsing() {
    let valid = PitchSystemId::try_new("scala").unwrap();
    assert_eq!(valid.as_str(), "scala");

    match PitchSystemId::try_new("   ") {
        Err(PitchSystemIdError::Empty) => {}
        other => panic!("unexpected result: {:?}", other),
    }

    match PitchSystemId::try_new("bad\nid") {
        Err(PitchSystemIdError::ContainsControl('\n')) => {}
        other => panic!("unexpected result: {:?}", other),
    }

    let parsed = PitchSystemId::from_str("ji").unwrap();
    assert_eq!(parsed.as_str(), "ji");
}

#[test]
fn try_register_system_prevents_duplicates() {
    let mut registry = TuningRegistry::new();
    registry
        .try_register_system("12", TwelveTET::a4_440())
        .unwrap();
    let err = registry
        .try_register_system("12", TwelveTET::a4_440())
        .unwrap_err();
    assert!(matches!(
        err,
        RegistryInsertError::DuplicateSystem(id) if id.as_str() == "12"
    ));
}

#[test]
fn borrowed_resolution_helpers_behave_like_owned_variants() {
    let mut registry = TuningRegistry::new();
    registry.register_system(PitchSystemId::from("12"), TwelveTET::a4_440());

    assert!(registry.resolve_system_str("12").is_ok());
    assert!(registry.resolve_frequency_str("12", 69).unwrap() > 0.0);
    assert_eq!(
        registry.resolve_name_str("12", 69).unwrap(),
        Some("12-TET(69)".to_string())
    );
}

#[test]
fn iterator_helpers_cover_mutation_and_consumption() {
    let mut registry = TuningRegistry::new()
        .with_system("a", TwelveTET::a4_440())
        .with_system("b", TwelveTET::a4_440());

    let ids: Vec<_> = registry.ids().map(|id| id.as_str().to_owned()).collect();
    assert_eq!(ids.len(), 2);

    assert_eq!(registry.systems().count(), 2);

    let mut seen_ref = 0;
    for (id, _) in &registry {
        assert!(id.as_str() == "a" || id.as_str() == "b");
        seen_ref += 1;
    }
    assert_eq!(seen_ref, 2);

    for system in registry.systems_mut() {
        *system = Arc::new(TwelveTET::new(432.0, 69));
    }

    let mut seen_mut = 0;
    for (id, _) in registry.iter_mut() {
        assert!(id.as_str() == "a" || id.as_str() == "b");
        seen_mut += 1;
    }
    assert_eq!(seen_mut, 2);

    let len_before = registry.len();
    let consumed_entries: Vec<_> = registry.clone().into_entries().collect();
    assert_eq!(consumed_entries.len(), len_before);

    let consumed_via_into: Vec<_> = registry.into_iter().collect();
    assert_eq!(consumed_via_into.len(), len_before);
}

#[test]
fn lazy_insertion_helper_avoids_duplicate_work() {
    let mut registry = TuningRegistry::new();

    let invocations = Cell::new(0);

    let first_ptr = Arc::as_ptr(registry.get_or_insert_with("lazy", || {
        invocations.set(invocations.get() + 1);
        TwelveTET::a4_440()
    }));
    assert_eq!(registry.len(), 1);

    let second_ptr = Arc::as_ptr(registry.get_or_insert_with("lazy", || {
        invocations.set(invocations.get() + 1);
        TwelveTET::new(430.0, 69)
    }));
    assert!(core::ptr::eq(first_ptr, second_ptr));
    assert_eq!(invocations.get(), 1);
}
