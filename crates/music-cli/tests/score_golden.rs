use assert_cmd::cargo::cargo_bin_cmd;

const PROGRESSION_TEXT: &str = "Progression score: 65.0/100 (3 unique / 3 total).\nFunction balance — tonic 1, predominant 1, dominant 1, other 0.\nCoverage 100% · Cadence 0% · Variety 100%.\nCadence detected: none.\nContext key hint: Cmaj.\n\n- Balanced tonic→predominant→dominant flow detected\n- No terminal cadence detected\n\n";

const PROGRESSION_JSON: &str = r#"{
  "progression": [
    "I",
    "ii",
    "V"
  ],
  "total_chords": 3,
  "unique_chords": 3,
  "function_counts": {
    "tonic": 1,
    "predominant": 1,
    "dominant": 1,
    "other": 0
  },
  "cadence": null,
  "coverage_score": 1.0,
  "cadence_score": 0.0,
  "variety_score": 1.0,
  "total_score": 65.0,
  "commentary": [
    "Balanced tonic→predominant→dominant flow detected",
    "No terminal cadence detected"
  ],
  "key_hint": "Cmaj"
}
"#;

const MELODY_TEXT: &str = "Melody score: 78.0/100 across 5 notes (ambitus 7 st).\nStepwise 100% · Leaps 0% · Direction changes 0.\nRange 100% · Motion 100% · Contour 0% (closure Δ +7).\nContext key hint: Cmaj.\n\n- Mostly conjunct motion keeps tension low\n- Ending away from the opening pitch leaves tension unresolved\n\n";

const MELODY_JSON: &str = r#"{
  "note_count": 5,
  "ambitus": {
    "lowest": 60,
    "highest": 67,
    "span": 7
  },
  "leap_ratio": 0.0,
  "stepwise_ratio": 1.0,
  "direction_changes": 0,
  "closure_interval": 7,
  "range_score": 1.0,
  "motion_score": 1.0,
  "contour_score": 0.0,
  "total_score": 78.0,
  "commentary": [
    "Mostly conjunct motion keeps tension low",
    "Ending away from the opening pitch leaves tension unresolved"
  ],
  "key_hint": "Cmaj"
}
"#;

const CHORD_TEXT: &str = "Chord score: 74.2/100 — span 11 st, 4 pitch classes, 0 extension(s).\nColor 80% · Stability 68% · Tension index 30%.\nNotes (12tet): 12-TET(60), 12-TET(64), 12-TET(67), 12-TET(71).\n\n";

const CHORD_JSON: &str = r#"{
  "system": "12tet",
  "note_count": 4,
  "pitch_span": 11,
  "unique_pitch_classes": 4,
  "extensions": 0,
  "color_score": 0.8,
  "stability_score": 0.685,
  "tension_index": 0.3,
  "total_score": 74.25,
  "note_labels": [
    "12-TET(60)",
    "12-TET(64)",
    "12-TET(67)",
    "12-TET(71)"
  ],
  "commentary": []
}
"#;

#[test]
fn progression_score_matches_golden() {
    let mut cmd = cargo_bin_cmd!("music-cli");
    cmd.args([
        "score",
        "progression",
        "--progression",
        "I,ii,V",
        "--in",
        "Cmaj",
    ]);
    cmd.assert().success().stdout(PROGRESSION_TEXT);
}

#[test]
fn progression_score_json_matches_golden() {
    let mut cmd = cargo_bin_cmd!("music-cli");
    cmd.args([
        "score",
        "progression",
        "--progression",
        "I,ii,V",
        "--in",
        "Cmaj",
        "--format",
        "json",
    ]);
    cmd.assert().success().stdout(PROGRESSION_JSON);
}

#[test]
fn melody_score_matches_golden() {
    let mut cmd = cargo_bin_cmd!("music-cli");
    cmd.args([
        "score",
        "melody",
        "--notes",
        "60,62,64,65,67",
        "--in",
        "Cmaj",
    ]);
    cmd.assert().success().stdout(MELODY_TEXT);
}

#[test]
fn melody_score_json_matches_golden() {
    let mut cmd = cargo_bin_cmd!("music-cli");
    cmd.args([
        "score",
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
fn chord_score_matches_golden() {
    let mut cmd = cargo_bin_cmd!("music-cli");
    cmd.args(["score", "chord", "--notes", "60,64,67,71"]);
    cmd.assert().success().stdout(CHORD_TEXT);
}

#[test]
fn chord_score_json_matches_golden() {
    let mut cmd = cargo_bin_cmd!("music-cli");
    cmd.args([
        "score",
        "chord",
        "--notes",
        "60,64,67,71",
        "--format",
        "json",
    ]);
    cmd.assert().success().stdout(CHORD_JSON);
}
