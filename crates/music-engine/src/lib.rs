#![forbid(unsafe_code)]

use music_core::{
    AbstractPitch, Pitch, PitchError, PitchSystem, PitchSystemId, TuningRegistry,
    systems::{TwelveTET, TwentyFourTET},
};

/// Primary access point for pitch/tuning utilities across crates.
///
/// The engine wraps [`TuningRegistry`] to provide sensible defaults (12- and
/// 24-tone equal temperaments) while allowing callers to register their own
/// systems.
#[derive(Debug, Clone)]
pub struct MusicEngine {
    registry: TuningRegistry,
}

impl MusicEngine {
    /// Create an empty engine.
    #[must_use]
    pub fn new() -> Self {
        Self {
            registry: TuningRegistry::new(),
        }
    }

    /// Create an engine pre-populated with standard equal temperaments.
    #[must_use]
    pub fn with_default_systems() -> Self {
        let registry = TuningRegistry::new()
            .with_system(PitchSystemId::from("12tet"), TwelveTET::a4_440())
            .with_system(PitchSystemId::from("24tet"), TwentyFourTET::a4_440());
        Self { registry }
    }

    /// Access the underlying registry.
    pub fn registry(&self) -> &TuningRegistry {
        &self.registry
    }

    /// Mutable access to the registry for advanced customization.
    pub fn registry_mut(&mut self) -> &mut TuningRegistry {
        &mut self.registry
    }

    /// Register an additional pitch system.
    pub fn register_system<T>(&mut self, id: impl Into<PitchSystemId>, system: T)
    where
        T: PitchSystem + 'static,
    {
        self.registry.register_system(id, system);
    }

    /// Resolve a pitch into frequency (Hz).
    pub fn resolve_pitch(&self, pitch: &Pitch) -> Result<f32, PitchError> {
        pitch.try_freq_hz(&self.registry)
    }

    /// Resolve a pitch into a human-friendly label, falling back to frequency when unnamed.
    pub fn describe_pitch(&self, pitch: &Pitch) -> Result<String, PitchError> {
        pitch
            .try_label(&self.registry)
            .map(|label| label.to_string_lossy())
    }

    /// Convenience helper for constructing abstract pitches tied to registered
    /// systems.
    pub fn pitch(&self, index: i32, system: impl Into<PitchSystemId>) -> AbstractPitch {
        AbstractPitch::new(index, system.into())
    }
}

impl Default for MusicEngine {
    fn default() -> Self {
        Self::with_default_systems()
    }
}

/// Re-export the `music-core` prelude alongside the [`MusicEngine`].
pub mod prelude {
    pub use super::MusicEngine;
    pub use music_core::prelude::*;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolves_default_pitches() {
        let engine = MusicEngine::with_default_systems();
        let pitch = Pitch::abstract_pitch(69, PitchSystemId::from("12tet"));
        assert!((engine.resolve_pitch(&pitch).unwrap() - 440.0).abs() < 1e-6);
    }

    #[test]
    fn allows_custom_registration() {
        struct Dummy;
        impl PitchSystem for Dummy {
            fn to_frequency(&self, index: i32) -> f32 {
                100.0 + index as f32
            }
        }

        let mut engine = MusicEngine::new();
        engine.register_system("dummy", Dummy);
        let pitch = engine.pitch(0, "dummy");
        assert_eq!(engine.resolve_pitch(&Pitch::from(pitch)).unwrap(), 100.0);
    }
}
