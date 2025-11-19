use assert_cmd::cargo::cargo_bin_cmd;

const SCALE_TEXT: &str = "Scale map for Major in 12tet, root 60 (12-TET(60)).\nPitch-class layout:\n  pc  0 ● degree 1 (12-TET(60))\n  pc  1 ·\n  pc  2 ● degree 2 (12-TET(62))\n  pc  3 ·\n  pc  4 ● degree 3 (12-TET(64))\n  pc  5 ● degree 4 (12-TET(65))\n  pc  6 ·\n  pc  7 ● degree 5 (12-TET(67))\n  pc  8 ·\n  pc  9 ● degree 6 (12-TET(69))\n  pc 10 ·\n  pc 11 ● degree 7 (12-TET(71))\n\nModulatory paths:\n  Rotation 1: Dorian, root 62 (12-TET(62)), pivots [0, 2, 4, 5, 7, 9, 11].\n  Rotation 2: Phrygian, root 64 (12-TET(64)), pivots [0, 2, 4, 5, 7, 9, 11].\n\n";

const SCALE_JSON: &str = r#"{
  "system": "12tet",
  "root_index": 60,
  "root_label": "12-TET(60)",
  "scale": "Major",
  "members": [
    {
      "degree": 1,
      "pitch_class": 0,
      "index": 60,
      "label": "12-TET(60)",
      "frequency_hz": 261.62558
    },
    {
      "degree": 2,
      "pitch_class": 2,
      "index": 62,
      "label": "12-TET(62)",
      "frequency_hz": 293.66476
    },
    {
      "degree": 3,
      "pitch_class": 4,
      "index": 64,
      "label": "12-TET(64)",
      "frequency_hz": 329.62756
    },
    {
      "degree": 4,
      "pitch_class": 5,
      "index": 65,
      "label": "12-TET(65)",
      "frequency_hz": 349.22824
    },
    {
      "degree": 5,
      "pitch_class": 7,
      "index": 67,
      "label": "12-TET(67)",
      "frequency_hz": 391.99542
    },
    {
      "degree": 6,
      "pitch_class": 9,
      "index": 69,
      "label": "12-TET(69)",
      "frequency_hz": 440.0
    },
    {
      "degree": 7,
      "pitch_class": 11,
      "index": 71,
      "label": "12-TET(71)",
      "frequency_hz": 493.8833
    }
  ],
  "pitch_class_map": [
    {
      "pitch_class": 0,
      "occupied": true,
      "degree": 1,
      "label": "12-TET(60)"
    },
    {
      "pitch_class": 1,
      "occupied": false,
      "degree": null,
      "label": null
    },
    {
      "pitch_class": 2,
      "occupied": true,
      "degree": 2,
      "label": "12-TET(62)"
    },
    {
      "pitch_class": 3,
      "occupied": false,
      "degree": null,
      "label": null
    },
    {
      "pitch_class": 4,
      "occupied": true,
      "degree": 3,
      "label": "12-TET(64)"
    },
    {
      "pitch_class": 5,
      "occupied": true,
      "degree": 4,
      "label": "12-TET(65)"
    },
    {
      "pitch_class": 6,
      "occupied": false,
      "degree": null,
      "label": null
    },
    {
      "pitch_class": 7,
      "occupied": true,
      "degree": 5,
      "label": "12-TET(67)"
    },
    {
      "pitch_class": 8,
      "occupied": false,
      "degree": null,
      "label": null
    },
    {
      "pitch_class": 9,
      "occupied": true,
      "degree": 6,
      "label": "12-TET(69)"
    },
    {
      "pitch_class": 10,
      "occupied": false,
      "degree": null,
      "label": null
    },
    {
      "pitch_class": 11,
      "occupied": true,
      "degree": 7,
      "label": "12-TET(71)"
    }
  ],
  "modulatory_paths": [
    {
      "rotation": 1,
      "mode_name": "Dorian",
      "root_index": 62,
      "root_label": "12-TET(62)",
      "pivot_pitch_classes": [
        0,
        2,
        4,
        5,
        7,
        9,
        11
      ]
    },
    {
      "rotation": 2,
      "mode_name": "Phrygian",
      "root_index": 64,
      "root_label": "12-TET(64)",
      "pivot_pitch_classes": [
        0,
        2,
        4,
        5,
        7,
        9,
        11
      ]
    }
  ]
}
"#;

#[test]
fn scale_map_text_matches_golden() {
    let mut cmd = cargo_bin_cmd!("music-cli");
    cmd.args([
        "map",
        "scale",
        "--root",
        "60",
        "--scale",
        "major",
        "--system",
        "12tet",
        "--modulations",
        "2",
    ]);
    cmd.assert().success().stdout(SCALE_TEXT);
}

#[test]
fn scale_map_json_matches_golden() {
    let mut cmd = cargo_bin_cmd!("music-cli");
    cmd.args([
        "--format",
        "json",
        "map",
        "scale",
        "--root",
        "60",
        "--scale",
        "major",
        "--system",
        "12tet",
        "--modulations",
        "2",
    ]);
    cmd.assert().success().stdout(SCALE_JSON);
}
