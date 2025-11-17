use music_engine::prelude::*;

fn main() {
    let mut args = std::env::args().skip(1);
    let index = args
        .next()
        .and_then(|value| value.parse::<i32>().ok())
        .unwrap_or(69);
    let system_id = args.next().unwrap_or_else(|| "12tet".to_string());

    let engine = MusicEngine::with_default_systems();
    let pitch = Pitch::abstract_pitch(index, PitchSystemId::from(system_id.clone()));

    match (engine.describe_pitch(&pitch), engine.resolve_pitch(&pitch)) {
        (Ok(name), Ok(freq)) => {
            println!(
                "Pitch {index} in {system}: {name} ({freq:.3} Hz)",
                system = system_id
            );
        }
        (Err(err), _) | (_, Err(err)) => {
            eprintln!("Failed to resolve pitch: {err}");
        }
    }
}
