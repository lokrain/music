use assert_cmd::cargo::cargo_bin_cmd;
use serde_json::Value;

const MELODY_TEXT: &str = "Melody diff (12tet) — shared pcs [0], distance 66.7%.\nContext key: Cmaj.\nLeft: 3 notes, 3 unique pcs, ambitus 4 st.\nRight: 3 notes, 3 unique pcs, ambitus 7 st.\nLeft-only pcs: [2, 4].\nRight-only pcs: [5, 7].\n\n- Melodies have markedly different pitch distributions\n- Ambitus differs: left 4 st, right 7 st\n\n";

const MELODY_JSON: &str = r#"{
  "system": "12tet",
  "key_hint": "Cmaj",
  "left": {
    "note_count": 3,
    "distinct_pitch_classes": 3,
    "ambitus": {
      "lowest": 60,
      "highest": 64,
      "span": 4
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
      }
    ]
  },
  "right": {
    "note_count": 3,
    "distinct_pitch_classes": 3,
    "ambitus": {
      "lowest": 60,
      "highest": 67,
      "span": 7
    },
    "histogram": [
      {
        "pitch_class": 0,
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
    ]
  },
  "shared_pitch_classes": [
    0
  ],
  "left_only_pitch_classes": [
    2,
    4
  ],
  "right_only_pitch_classes": [
    5,
    7
  ],
  "histogram_distance": 0.6666667,
  "commentary": [
    "Melodies have markedly different pitch distributions",
    "Ambitus differs: left 4 st, right 7 st"
  ]
}
"#;

const PROG_TEXT: &str = "Progression diff — shared 2, left unique 1, right unique 1.\nContext key: Cmaj.\nLeft: 3/4 unique chords, tonic 2, predominant 1, dominant 1, other 0.\n  Cadence: V–I (Authentic cadence, conf 92%).\nRight: 3/3 unique chords, tonic 1, predominant 1, dominant 1, other 0.\n  Cadence: none detected.\nShared chords: I, V.\nLeft-only chords: II.\nRight-only chords: IV.\n\n- Left progression cadences while right does not\n\n";

const PROG_JSON: &str = r#"{
  "key_hint": "Cmaj",
  "left": {
    "progression": [
      "I",
      "ii",
      "V",
      "I"
    ],
    "chord_count": 4,
    "unique_chords": 3,
    "function_counts": {
      "tonic": 2,
      "predominant": 1,
      "dominant": 1,
      "other": 0
    },
    "cadence": {
      "pattern": "V–I",
      "confidence": 0.92,
      "description": "Authentic cadence"
    }
  },
  "right": {
    "progression": [
      "I",
      "IV",
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
    "cadence": null
  },
  "shared_chords": [
    "I",
    "V"
  ],
  "left_unique": [
    "II"
  ],
  "right_unique": [
    "IV"
  ],
  "commentary": [
    "Left progression cadences while right does not"
  ]
}
"#;

#[test]
fn melody_diff_text_matches_golden() {
    let mut cmd = cargo_bin_cmd!("music-cli");
    cmd.args([
        "explain-diff",
        "melody",
        "--left-notes",
        "60,62,64",
        "--right-notes",
        "60,65,67",
        "--in",
        "Cmaj",
    ]);
    cmd.assert().success().stdout(MELODY_TEXT);
}

#[test]
fn melody_diff_json_matches_golden() {
    let mut cmd = cargo_bin_cmd!("music-cli");
    cmd.args([
        "explain-diff",
        "melody",
        "--left-notes",
        "60,62,64",
        "--right-notes",
        "60,65,67",
        "--in",
        "Cmaj",
        "--format",
        "json",
    ]);
    let output = cmd.output().expect("failed to run music-cli");
    assert!(output.status.success());
    let actual: Value = serde_json::from_slice(&output.stdout).expect("invalid CLI JSON");
    let expected: Value = serde_json::from_str(MELODY_JSON).expect("invalid golden JSON");
    assert_eq!(actual, expected);
}

#[test]
fn progression_diff_text_matches_golden() {
    let mut cmd = cargo_bin_cmd!("music-cli");
    cmd.args([
        "explain-diff",
        "progression",
        "--left",
        "I,ii,V,I",
        "--right",
        "I,IV,V",
        "--in",
        "Cmaj",
    ]);
    cmd.assert().success().stdout(PROG_TEXT);
}

#[test]
fn progression_diff_json_matches_golden() {
    let mut cmd = cargo_bin_cmd!("music-cli");
    cmd.args([
        "explain-diff",
        "progression",
        "--left",
        "I,ii,V,I",
        "--right",
        "I,IV,V",
        "--in",
        "Cmaj",
        "--format",
        "json",
    ]);
    let output = cmd.output().expect("failed to run music-cli");
    assert!(output.status.success());
    let actual: Value = serde_json::from_slice(&output.stdout).expect("invalid CLI JSON");
    let expected: Value = serde_json::from_str(PROG_JSON).expect("invalid golden JSON");
    assert_eq!(actual, expected);
}
