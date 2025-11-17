use music_core::{
    Interval, TuningRegistry, pitch::Pitch, scale::ScalePattern, system::PitchSystemId,
    systems::TwelveTET,
};

pub fn registry() -> (TuningRegistry, PitchSystemId) {
    let system = PitchSystemId::from("12");
    let registry = TuningRegistry::new().with_system(system.clone(), TwelveTET::a4_440());
    (registry, system)
}

pub fn major_pattern(registry: &TuningRegistry, system: &PitchSystemId) -> ScalePattern {
    ScalePattern::from_steps(vec![
        step_interval(2, system, registry),
        step_interval(2, system, registry),
        step_interval(1, system, registry),
        step_interval(2, system, registry),
        step_interval(2, system, registry),
        step_interval(2, system, registry),
        step_interval(1, system, registry),
    ])
    .unwrap()
}

pub fn step_interval(steps: i32, system: &PitchSystemId, registry: &TuningRegistry) -> Interval {
    let base = Pitch::abstract_pitch(0, system.clone());
    let target = Pitch::abstract_pitch(steps, system.clone());
    Interval::between(&base, &target, registry).unwrap()
}
