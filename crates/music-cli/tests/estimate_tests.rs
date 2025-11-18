use assert_cmd::cargo::cargo_bin_cmd;

#[test]
fn estimate_melody_c_major() {
    let mut cmd = cargo_bin_cmd!("music-cli");
    cmd.args(["estimate", "melody", "--notes", "60,62,64,65,67,69,71,72"])
        .assert()
        .success()
        .stdout(predicates::str::contains("Musical feature estimation"))
        .stdout(predicates::str::contains("Estimated key: C"));
}

#[test]
fn estimate_melody_d_minor() {
    let mut cmd = cargo_bin_cmd!("music-cli");
    cmd.args(["estimate", "melody", "--notes", "62,64,65,67,69,70,72,74"])
        .assert()
        .success()
        .stdout(predicates::str::contains("Estimated key:"));
}

#[test]
fn estimate_melody_json_output() {
    let mut cmd = cargo_bin_cmd!("music-cli");
    cmd.args([
        "--format",
        "json",
        "estimate",
        "melody",
        "--notes",
        "60,62,64,65,67",
    ])
    .assert()
    .success()
    .stdout(predicates::str::contains("\"input_type\": \"melody\""))
    .stdout(predicates::str::contains("key_estimate"))
    .stdout(predicates::str::contains("key_confidence"));
}

#[test]
fn estimate_melody_empty_notes() {
    let mut cmd = cargo_bin_cmd!("music-cli");
    cmd.args(["estimate", "melody", "--notes", ""])
        .assert()
        .failure();
}

#[test]
fn estimate_melody_chromatic() {
    let mut cmd = cargo_bin_cmd!("music-cli");
    cmd.args([
        "estimate",
        "melody",
        "--notes",
        "60,61,62,63,64,65,66,67,68,69,70,71,72",
    ])
    .assert()
    .success()
    .stdout(predicates::str::contains("Estimated key:"));
}

#[test]
fn estimate_melody_single_note() {
    let mut cmd = cargo_bin_cmd!("music-cli");
    cmd.args(["estimate", "melody", "--notes", "60"])
        .assert()
        .success()
        .stdout(predicates::str::contains("Estimated key:"));
}

#[test]
fn estimate_melody_g_major() {
    let mut cmd = cargo_bin_cmd!("music-cli");
    cmd.args(["estimate", "melody", "--notes", "67,69,71,72,74,76,78,79"])
        .assert()
        .success()
        .stdout(predicates::str::contains("Estimated key:"))
        .stdout(predicates::str::contains("confidence"));
}

#[test]
fn estimate_melody_confidence_in_json() {
    let mut cmd = cargo_bin_cmd!("music-cli");
    cmd.args([
        "--format",
        "json",
        "estimate",
        "melody",
        "--notes",
        "60,64,67,60,64,67,60",
    ])
    .assert()
    .success()
    .stdout(predicates::str::contains("key_confidence"));
}
