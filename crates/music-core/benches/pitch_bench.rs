use criterion::{Criterion, criterion_group, criterion_main};
use music_core::{PitchSystemId, TuningRegistry, pitch::Pitch, systems::TwelveTET};
use std::hint::black_box;

fn build_registry() -> TuningRegistry {
    TuningRegistry::new().with_system(PitchSystemId::from("12tet"), TwelveTET::a4_440())
}

fn bench_abstract_pitch_resolution(c: &mut Criterion) {
    let registry = build_registry();
    let pitch = Pitch::abstract_pitch(69, PitchSystemId::from("12tet"));

    c.bench_function("pitch::freq_hz abstract", |b| {
        b.iter(|| black_box(pitch.freq_hz(black_box(&registry))));
    });
}

fn bench_literal_pitch_resolution(c: &mut Criterion) {
    let pitch = Pitch::hz(black_box(440.0));

    c.bench_function("pitch::freq_hz literal", |b| {
        b.iter(|| black_box(pitch.freq_hz(black_box(&build_registry()))));
    });
}

criterion_group!(
    benches,
    bench_abstract_pitch_resolution,
    bench_literal_pitch_resolution
);
criterion_main!(benches);
