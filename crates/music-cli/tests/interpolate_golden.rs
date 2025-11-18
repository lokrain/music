use assert_cmd::cargo::cargo_bin_cmd;

const TEMPO_TEXT: &str = "Interpolation (bpm) — curve Linear, 2 anchors, 6 samples.\nAnchors:\n  t= 0.00 → 120.000\n  t= 4.00 → 140.000\nSamples:\n  t= 0.00 → 120.000\n  t= 0.80 → 124.000\n  t= 1.60 → 128.000\n  t= 2.40 → 132.000\n  t= 3.20 → 136.000\n  t= 4.00 → 140.000\n\n";

const TEMPO_JSON: &str = r#"{
  "context": {
    "curve": "Linear",
    "samples": 6
  },
  "unit": "bpm",
  "anchors": [
    {
      "time": 0.0,
      "value": 120.0
    },
    {
      "time": 4.0,
      "value": 140.0
    }
  ],
  "samples": [
    {
      "time": 0.0,
      "value": 120.0
    },
    {
      "time": 0.8,
      "value": 124.0
    },
    {
      "time": 1.6,
      "value": 128.0
    },
    {
      "time": 2.4,
      "value": 132.0
    },
    {
      "time": 3.2,
      "value": 136.0
    },
    {
      "time": 4.0,
      "value": 140.0
    }
  ]
}
"#;

const VELOCITY_TEXT: &str = "Velocity interpolation [0..127] — curve EaseInOut, 2 anchors.\nAnchors:\n  t= 0.00 →    0.0\n  t= 2.00 →  100.0\nSamples:\n  t= 0.00 →    0.0\n  t= 0.44 →   22.2\n  t= 1.56 →   77.8\n  t= 2.00 →  100.0\n\n";

#[test]
fn tempo_interpolation_text_matches_golden() {
    let mut cmd = cargo_bin_cmd!("music-cli");
    cmd.args([
        "interpolate",
        "tempo",
        "--points",
        "0:120,4:140",
        "--samples",
        "5",
    ]);
    cmd.assert().success().stdout(TEMPO_TEXT);
}

#[test]
fn tempo_interpolation_json_matches_golden() {
    let mut cmd = cargo_bin_cmd!("music-cli");
    cmd.args([
        "--format",
        "json",
        "interpolate",
        "tempo",
        "--points",
        "0:120,4:140",
        "--samples",
        "5",
    ]);
    cmd.assert().success().stdout(TEMPO_JSON);
}

#[test]
fn velocity_interpolation_text_matches_golden() {
    let mut cmd = cargo_bin_cmd!("music-cli");
    cmd.args([
        "interpolate",
        "velocity",
        "--points",
        "0:0,2:100",
        "--samples",
        "3",
        "--curve",
        "ease-in-out",
    ]);
    cmd.assert().success().stdout(VELOCITY_TEXT);
}
