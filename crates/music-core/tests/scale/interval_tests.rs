use music_core::{pitch::AbstractPitch, scale::Scale};

use super::support::{major_pattern, registry};

#[test]
fn degree_interval_matches_pitch_transposition() {
    let (registry, system) = registry();
    let pattern = major_pattern(&registry, &system);
    let root = AbstractPitch::new(60, system.clone());
    let scale = Scale::from_abstract_root(root, pattern);

    let interval = scale.degree_interval(4).unwrap();
    let transposed = interval.apply_to(scale.root(), &registry).unwrap();
    assert_eq!(transposed.as_abstract().unwrap().index, 67);
    let steps = interval.steps().expect("steps tracked");
    assert_eq!(steps.0, 7);
}

#[test]
fn degree_intervals_accumulate_from_root() {
    let (registry, system) = registry();
    let pattern = major_pattern(&registry, &system);
    let root = AbstractPitch::new(60, system.clone());
    let scale = Scale::from_abstract_root(root, pattern);

    let intervals = scale.degree_intervals(7).unwrap();
    assert_eq!(intervals.len(), 8);
    assert!((intervals[0].ratio() - 1.0).abs() < 1e-6);

    for (degree, cached) in intervals.iter().enumerate() {
        let single = scale.degree_interval(degree).unwrap();
        assert_eq!(cached, &single);
    }

    let pattern_intervals = scale.pattern().degree_intervals(7).unwrap();
    assert_eq!(intervals, pattern_intervals);

    let (steps, captured_system) = intervals[4].steps().expect("steps tracked");
    assert_eq!(steps, 7);
    assert_eq!(captured_system.as_str(), system.as_str());
}

#[test]
fn pattern_interval_iterator_matches_vector() {
    let (registry, system) = registry();
    let pattern = major_pattern(&registry, &system);
    let iterated: Vec<_> = pattern
        .degree_intervals_iter()
        .take(8)
        .map(|result| result.expect("intervals valid").1)
        .collect();
    let via_vec = pattern.degree_intervals(7).unwrap();
    assert_eq!(iterated, via_vec);
}

#[test]
fn scale_interval_iterator_matches_pattern() {
    let (registry, system) = registry();
    let pattern = major_pattern(&registry, &system);
    let root = AbstractPitch::new(60, system.clone());
    let scale = Scale::from_abstract_root(root, pattern);

    let collected: Vec<_> = scale
        .degree_interval_iter()
        .take(8)
        .map(|result| result.expect("interval valid").1)
        .collect();
    let via_vec = scale.degree_intervals(7).unwrap();
    assert_eq!(collected, via_vec);
}

#[test]
fn pattern_rotation_wraps_steps_cleanly() {
    let (registry, system) = registry();
    let pattern = major_pattern(&registry, &system);
    let rotated = pattern.rotate(3);
    assert_eq!(rotated.steps().len(), pattern.steps().len());
    let degrees = rotated.degree_intervals(6).unwrap();
    assert_eq!(degrees.len(), 7);
}
