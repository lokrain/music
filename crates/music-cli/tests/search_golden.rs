use assert_cmd::cargo::cargo_bin_cmd;

const SCALE_TEXT: &str = "Scale search in 12tet: 2 match(es) for pcs [0, 4, 7].\n  - Ionian rooted at 12-TET(60) (60).\n  - Lydian rooted at 12-TET(60) (60).\n\n";

const SCALE_JSON: &str = r#"{
  "system": "12tet",
  "criteria": [
    0,
    4,
    7
  ],
  "match_count": 2,
  "matches": [
    {
      "scale": "Ionian",
      "root_index": 60,
      "root_label": "12-TET(60)"
    },
    {
      "scale": "Lydian",
      "root_index": 60,
      "root_label": "12-TET(60)"
    }
  ]
}
"#;

const CHORD_TEXT: &str = "Chord search (Triads) in 12tet: 1 match(es) for pcs [0, 4, 7].\n  - Ionian degree 1 (I) root 12-TET(60), pcs [0, 4, 7].\n\n";

#[test]
fn scale_search_text_matches_golden() {
    let mut cmd = cargo_bin_cmd!("music-cli");
    cmd.args(["search", "scale", "--notes", "60,64,67", "--limit", "2"]);
    cmd.assert().success().stdout(SCALE_TEXT);
}

#[test]
fn scale_search_json_matches_golden() {
    let mut cmd = cargo_bin_cmd!("music-cli");
    cmd.args([
        "--format", "json", "search", "scale", "--notes", "60,64,67", "--limit", "2",
    ]);
    cmd.assert().success().stdout(SCALE_JSON);
}

#[test]
fn chord_search_text_matches_golden() {
    let mut cmd = cargo_bin_cmd!("music-cli");
    cmd.args(["search", "chord", "--notes", "60,64,67", "--limit", "1"]);
    cmd.assert().success().stdout(CHORD_TEXT);
}
