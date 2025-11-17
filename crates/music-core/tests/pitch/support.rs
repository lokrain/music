use music_core::{PitchSystemId, TuningRegistry, systems::TwelveTET};

pub fn empty_registry() -> TuningRegistry {
    TuningRegistry::new()
}

pub fn registered_registry() -> (TuningRegistry, PitchSystemId) {
    let system = PitchSystemId::from("12tet");
    let registry = TuningRegistry::new().with_system(system.clone(), TwelveTET::a4_440());
    (registry, system)
}
