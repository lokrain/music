use assert_cmd::cargo::cargo_bin_cmd;

#[test]
fn profile_melody_basic() {
    let mut cmd = cargo_bin_cmd!("music-cli");
    cmd.args(["profile", "melody", "--notes", "60,62,64,65,67,69,71,72"])
        .assert()
        .success()
        .stdout(predicates::str::contains("Profile for melody"))
        .stdout(predicates::str::contains("8 events"))
        .stdout(predicates::str::contains("Pitch Range"));
}

#[test]
fn profile_melody_json_output() {
    let mut cmd = cargo_bin_cmd!("music-cli");
    cmd.args([
        "--format",
        "json",
        "profile",
        "melody",
        "--notes",
        "60,62,64,65,67",
    ])
    .assert()
    .success()
    .stdout(predicates::str::contains("\"input_type\": \"melody\""))
    .stdout(predicates::str::contains("\"event_count\": 5"))
    .stdout(predicates::str::contains("pitch_range"))
    .stdout(predicates::str::contains("\"min_pitch\": 60"))
    .stdout(predicates::str::contains("\"max_pitch\": 67"));
}

#[test]
fn profile_melody_empty_notes() {
    let mut cmd = cargo_bin_cmd!("music-cli");
    cmd.args(["profile", "melody", "--notes", ""])
        .assert()
        .failure();
}

#[test]
fn profile_melody_with_percentiles() {
    let mut cmd = cargo_bin_cmd!("music-cli");
    cmd.args([
        "profile",
        "melody",
        "--notes",
        "60,61,62,63,64,65,66,67,68,69,70,71,72",
    ])
    .assert()
    .success()
    .stdout(predicates::str::contains("Median pitch"))
    .stdout(predicates::str::contains("25th percentile"))
    .stdout(predicates::str::contains("75th percentile"));
}

#[test]
fn profile_melody_single_note() {
    let mut cmd = cargo_bin_cmd!("music-cli");
    cmd.args(["profile", "melody", "--notes", "60"])
        .assert()
        .success()
        .stdout(predicates::str::contains("1 events"));
}

#[test]
fn profile_melody_wide_range() {
    let mut cmd = cargo_bin_cmd!("music-cli");
    cmd.args(["profile", "melody", "--notes", "36,48,60,72,84,96"])
        .assert()
        .success()
        .stdout(predicates::str::contains("Min pitch: 36"))
        .stdout(predicates::str::contains("Max pitch: 96"));
}
