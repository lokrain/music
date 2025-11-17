use music_engine::prelude::*;

fn main() {
    let engine = MusicEngine::with_default_systems();
    let standard = Pitch::abstract_pitch(69, PitchSystemId::from("12tet"));
    let quarter = Pitch::abstract_pitch(70, PitchSystemId::from("24tet"));

    print_pitch(&engine, "A4", &standard);
    print_pitch(&engine, "Quarter tone", &quarter);
}

fn print_pitch(engine: &MusicEngine, label: &str, pitch: &Pitch) {
    match engine.describe_pitch(pitch) {
        Ok(name) => {
            let freq = engine.resolve_pitch(pitch).unwrap_or_default();
            println!("{label}: {name} ({freq:.2} Hz)");
        }
        Err(err) => eprintln!("{label}: failed to resolve pitch ({err})"),
    }
}
