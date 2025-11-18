# music-cli

Small executable for inspecting pitches and building diatonic chords using the `music-engine` + `music-core` crates.

## Usage

```
cargo run -p music-cli -- pitch --index 69 --system 12tet
```

```
cargo run -p music-cli -- chords --root 60 --system 12tet --scale major --voicing sevenths
```

The chord subcommand lists each diatonic triad/seventh derived from the requested scale, including pitch labels and frequencies pulled from the engine's tuning registry. Use `--help` on the binary or an individual subcommand for the full argument list.
