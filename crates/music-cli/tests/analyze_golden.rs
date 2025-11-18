use assert_cmd::cargo::cargo_bin_cmd;

const MELODY_TEXT: &str = "Melody analysis: 5 notes (5 unique pitch classes).\nAmbitus: 7 semitones (lowest 60, highest 67).\nBest key: 12-TET(60) Major (forced) — match 100.0%\nTension: 0 notes (0.0%) outside the implied scale.\nPitch-class histogram:\n  pc  0: 1\n  pc  2: 1\n  pc  4: 1\n  pc  5: 1\n  pc  7: 1\n\n";

const MELODY_JSON: &str = r#"{
  "note_count": 5,
  "distinct_pitch_classes": 5,
  "ambitus": {
    "lowest": 60,
    "highest": 67,
    "span": 7
  },
  "best_key": {
    "tonic_label": "12-TET(60)",
    "tonic_pitch_class": 0,
    "tonic_index": 60,
    "mode": "Major",
    "match_ratio": 1.0,
    "enforced": true
  },
  "histogram": [
    {
      "pitch_class": 0,
      "count": 1
    },
    {
      "pitch_class": 2,
      "count": 1
    },
    {
      "pitch_class": 4,
      "count": 1
    },
    {
      "pitch_class": 5,
      "count": 1
    },
    {
      "pitch_class": 7,
      "count": 1
    }
  ],
  "tension": {
    "out_of_scale": 0,
    "percent_out_of_scale": 0.0
  }
}
"#;

const CHORD_TEXT: &str = "Chord progression (3 chords, 3 unique):\n  I → ii → V\nFunctional counts: tonic 1, predominant 1, dominant 1, other 0.\nCadence: none detected.\nContext key hint: Cmaj.\n\n";

const CHORD_JSON: &str = r#"{
  "progression": [
    "I",
    "ii",
    "V"
  ],
  "chord_count": 3,
  "unique_chords": 3,
  "function_counts": {
    "tonic": 1,
    "predominant": 1,
    "dominant": 1,
    "other": 0
  },
  "cadence": null,
  "key_hint": "Cmaj"
}
"#;

const MIDI_TEXT: &str = "MIDI file: tests/fixtures/simple.mid (26 bytes).\nStandard MIDI header detected.\nDeclared format: 0, tracks: Some(1), detected chunks: 1.\nTicks per quarter note: 480.\n\n";

const MIDI_JSON: &str = r#"{
  "file": "tests/fixtures/simple.mid",
  "size_bytes": 26,
  "header_format": 0,
  "declared_tracks": 1,
  "detected_tracks": 1,
  "ticks_per_quarter": 480,
  "key_hint": null,
  "is_standard_midi": true
}
"#;

const MIDI_FIXTURE: &str = "tests/fixtures/simple.mid";

#[test]
fn melody_text_output_matches_golden() {
    let mut cmd = cargo_bin_cmd!("music-cli");
    cmd.args([
        "analyze",
        "melody",
        "--notes",
        "60,62,64,65,67",
        "--in",
        "Cmaj",
    ]);
    cmd.assert().success().stdout(MELODY_TEXT);
}

#[test]
fn melody_json_output_matches_golden() {
    let mut cmd = cargo_bin_cmd!("music-cli");
    cmd.args([
        "analyze",
        "melody",
        "--notes",
        "60,62,64,65,67",
        "--in",
        "Cmaj",
        "--format",
        "json",
    ]);
    cmd.assert().success().stdout(MELODY_JSON);
}

#[test]
fn chord_text_output_matches_golden() {
    let mut cmd = cargo_bin_cmd!("music-cli");
    cmd.args([
        "analyze",
        "chords",
        "--progression",
        "I,ii,V",
        "--in",
        "Cmaj",
    ]);
    cmd.assert().success().stdout(CHORD_TEXT);
}

#[test]
fn chord_json_output_matches_golden() {
    let mut cmd = cargo_bin_cmd!("music-cli");
    cmd.args([
        "analyze",
        "chords",
        "--progression",
        "I,ii,V",
        "--in",
        "Cmaj",
        "--format",
        "json",
    ]);
    cmd.assert().success().stdout(CHORD_JSON);
}

#[test]
fn midi_text_output_matches_golden() {
    let mut cmd = cargo_bin_cmd!("music-cli");
    cmd.args(["analyze", "midi", "--file", MIDI_FIXTURE]);
    cmd.current_dir(env!("CARGO_MANIFEST_DIR"));
    cmd.assert().success().stdout(MIDI_TEXT);
}

#[test]
fn midi_json_output_matches_golden() {
    let mut cmd = cargo_bin_cmd!("music-cli");
    cmd.args([
        "analyze",
        "midi",
        "--file",
        MIDI_FIXTURE,
        "--format",
        "json",
    ]);
    cmd.current_dir(env!("CARGO_MANIFEST_DIR"));
    cmd.assert().success().stdout(MIDI_JSON);
}
