use music_core::{
    pitch::AbstractPitch,
    scale::{Scale, ScaleDegreeError},
    system::PitchSystemId,
};

use super::support::{major_pattern, registry};

#[test]
fn degree_pitches_batch_matches_individual_queries() {
    let (registry, system) = registry();
    let pattern = major_pattern(&registry, &system);
    let root = AbstractPitch::new(60, system.clone());
    let scale = Scale::from_abstract_root(root, pattern);

    let degrees = scale.degree_pitches(7, &registry).unwrap();
    assert_eq!(degrees.len(), 8);

    for (degree, pitch) in degrees.iter().enumerate() {
        let individual = scale.degree_pitch(degree, &registry).unwrap();
        assert_eq!(pitch, &individual);
    }

    let indexes: Vec<i32> = degrees
        .iter()
        .map(|pitch| pitch.as_abstract().unwrap().index)
        .collect();
    assert_eq!(indexes, vec![60, 62, 64, 65, 67, 69, 71, 72]);
}

#[test]
fn degree_iterator_matches_helpers() {
    let (registry, system) = registry();
    let pattern = major_pattern(&registry, &system);
    let root = AbstractPitch::new(60, system.clone());
    let scale = Scale::from_abstract_root(root, pattern);

    let collected: Vec<_> = scale
        .degrees_up_to(7, &registry)
        .map(|result| result.expect("iteration succeeds"))
        .collect();
    assert_eq!(collected.len(), 8);

    for (degree, interval, pitch) in collected {
        assert_eq!(interval, scale.degree_interval(degree).unwrap());
        assert_eq!(pitch, scale.degree_pitch(degree, &registry).unwrap());
    }
}

#[test]
fn degree_iterator_surfaces_pitch_errors() {
    let (registry_with, system) = registry();
    let pattern = major_pattern(&registry_with, &system);
    let other_system = PitchSystemId::from("other");
    let root = AbstractPitch::new(60, other_system);
    let scale = Scale::from_abstract_root(root, pattern);

    let mut iter = scale.degrees(&registry_with);

    assert!(iter.next().unwrap().is_ok());
    let err = iter.next().unwrap();
    assert!(matches!(err, Err(ScaleDegreeError::Pitch(_))));
}

#[test]
fn bounded_iterator_reports_remaining_items() {
    let (registry, system) = registry();
    let pattern = major_pattern(&registry, &system);
    let root = AbstractPitch::new(60, system.clone());
    let scale = Scale::from_abstract_root(root, pattern);

    let mut iter = scale.degrees_up_to(3, &registry);
    assert_eq!(iter.size_hint(), (4, Some(4)));
    assert_eq!(iter.len(), 4);

    iter.next();
    assert_eq!(iter.size_hint(), (3, Some(3)));
    assert_eq!(iter.len(), 3);
}
