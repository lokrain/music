use music_core::{Interval, pitch::AbstractPitch, scale::Scale};

use super::support::{major_pattern, registry};

#[test]
fn mode_back_restores_original_scale() {
    let (registry, system) = registry();
    let pattern = major_pattern(&registry, &system);
    let root = AbstractPitch::new(60, system.clone());
    let scale = Scale::from_abstract_root(root.clone(), pattern);

    let rotated = scale.mode(3, &registry).unwrap();
    let restored = rotated.mode_back(3, &registry).unwrap();

    assert_eq!(restored.root().as_abstract().unwrap().index, root.index);
    let original_ratios: Vec<f32> = scale
        .pattern()
        .steps()
        .iter()
        .map(Interval::ratio)
        .collect();
    let restored_ratios: Vec<f32> = restored
        .pattern()
        .steps()
        .iter()
        .map(Interval::ratio)
        .collect();
    assert_eq!(original_ratios, restored_ratios);
}

#[test]
fn mode_with_offset_handles_negative_values() {
    let (registry, system) = registry();
    let pattern = major_pattern(&registry, &system);
    let root = AbstractPitch::new(60, system.clone());
    let scale = Scale::from_abstract_root(root, pattern);

    let backwards = scale.mode_with_offset(-2, &registry).unwrap();
    let manual = scale.mode_back(2, &registry).unwrap();
    assert_eq!(
        backwards.root().as_abstract().unwrap().index,
        manual.root().as_abstract().unwrap().index
    );
    let ratios_back: Vec<f32> = backwards
        .pattern()
        .steps()
        .iter()
        .map(Interval::ratio)
        .collect();
    let ratios_manual: Vec<f32> = manual
        .pattern()
        .steps()
        .iter()
        .map(Interval::ratio)
        .collect();
    assert_eq!(ratios_back, ratios_manual);
}
