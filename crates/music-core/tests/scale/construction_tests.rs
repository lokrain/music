use music_core::{
    Interval,
    pitch::{AbstractPitch, Pitch},
    scale::{Scale, ScalePattern},
};

use super::support::{major_pattern, registry};

#[test]
fn major_scale_degrees_resolve_correctly() {
    let (registry, system) = registry();
    let pattern = major_pattern(&registry, &system);
    let root = AbstractPitch::new(60, system.clone());
    let scale = Scale::from_abstract_root(root.clone(), pattern);

    let second = scale.degree_pitch(1, &registry).unwrap();
    assert_eq!(second.as_abstract().unwrap().index, 62);

    let fifth = scale.degree_pitch(4, &registry).unwrap();
    assert_eq!(fifth.as_abstract().unwrap().index, 67);

    let octave = scale.degree_pitch(7, &registry).unwrap();
    assert_eq!(octave.as_abstract().unwrap().index, 72);
}

#[test]
fn twelve_tet_major_helper_matches_manual_pattern() {
    let (registry, system) = registry();
    let manual_pattern = major_pattern(&registry, &system);
    let manual = Scale::from_abstract_root(AbstractPitch::new(60, system.clone()), manual_pattern);

    let helper = Scale::twelve_tet_major(60, system.clone(), &registry).unwrap();
    assert_eq!(helper.root(), manual.root());
    for (lhs, rhs) in manual
        .pattern()
        .steps()
        .iter()
        .map(Interval::ratio)
        .zip(helper.pattern().steps().iter().map(Interval::ratio))
    {
        assert!((lhs - rhs).abs() < 1e-6);
    }
}

#[test]
fn literal_root_scale_operates_on_frequencies() {
    let (registry, system) = registry();
    let octave = Interval::between(
        &Pitch::abstract_pitch(0, system.clone()),
        &Pitch::abstract_pitch(12, system),
        &registry,
    )
    .unwrap();
    let pattern = ScalePattern::from_steps(vec![octave]).unwrap();
    let scale = Scale::from_pitch(Pitch::hz(440.0), pattern);

    let octave = scale.degree_pitch(1, &registry).unwrap();
    assert!((octave.as_frequency().unwrap() - 880.0).abs() < 1e-4);
}
