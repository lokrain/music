use assert_cmd::cargo::cargo_bin_cmd;

#[test]
fn extrapolate_melody_requires_notes() {
    let mut cmd = cargo_bin_cmd!("music-cli");
    cmd.args(["extrapolate", "melody"])
        .assert()
        .failure()
        .stderr(predicates::str::contains("required"));
}

#[test]
fn extrapolate_melody_basic() {
    let mut cmd = cargo_bin_cmd!("music-cli");
    cmd.args([
        "extrapolate",
        "melody",
        "--notes",
        "60,62,64,65,67,69,71,72",
        "--order",
        "2",
        "--count",
        "3",
    ])
    .assert()
    .success()
    .stdout(predicates::str::contains("Melody extrapolation"))
    .stdout(predicates::str::contains("n-gram order 2"));
}

#[test]
fn extrapolate_melody_json_output() {
    let mut cmd = cargo_bin_cmd!("music-cli");
    cmd.args([
        "--format",
        "json",
        "extrapolate",
        "melody",
        "--notes",
        "60,62,64,65,67",
        "--order",
        "1",
    ])
    .assert()
    .success()
    .stdout(predicates::str::contains("\"model_order\":1"))
    .stdout(predicates::str::contains("\"system\":\"12tet\""));
}

#[test]
fn extrapolate_chords_requires_progression() {
    let mut cmd = cargo_bin_cmd!("music-cli");
    cmd.args(["extrapolate", "chords"])
        .assert()
        .failure()
        .stderr(predicates::str::contains("required"));
}

#[test]
fn extrapolate_chords_basic() {
    let mut cmd = cargo_bin_cmd!("music-cli");
    cmd.args([
        "extrapolate",
        "chords",
        "--progression",
        "I,IV,V,I,I,vi,ii,V",
        "--order",
        "1",
        "--count",
        "3",
    ])
    .assert()
    .success()
    .stdout(predicates::str::contains("Chord extrapolation"))
    .stdout(predicates::str::contains("n-gram order 1"));
}

#[test]
fn extrapolate_chords_json_output() {
    let mut cmd = cargo_bin_cmd!("music-cli");
    cmd.args([
        "--format",
        "json",
        "extrapolate",
        "chords",
        "--progression",
        "I,V,I,IV,V",
        "--order",
        "1",
    ])
    .assert()
    .success()
    .stdout(predicates::str::contains("\"model_order\":1"))
    .stdout(predicates::str::contains("predictions"));
}

#[test]
fn extrapolate_melody_with_key_hint() {
    let mut cmd = cargo_bin_cmd!("music-cli");
    cmd.args([
        "extrapolate",
        "melody",
        "--notes",
        "60,62,64,65,67",
        "--in",
        "Cmaj",
        "--order",
        "1",
    ])
    .assert()
    .success()
    .stdout(predicates::str::contains("Context key: Cmaj"));
}

#[test]
fn extrapolate_chords_with_key_hint() {
    let mut cmd = cargo_bin_cmd!("music-cli");
    cmd.args([
        "extrapolate",
        "chords",
        "--progression",
        "I,IV,V,I",
        "--in",
        "Gmaj",
        "--order",
        "1",
    ])
    .assert()
    .success()
    .stdout(predicates::str::contains("Context key: Gmaj"));
}
