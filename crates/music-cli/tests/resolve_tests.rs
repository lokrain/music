use assert_cmd::cargo::cargo_bin_cmd;

#[test]
fn resolve_v7_to_i_c_major_text() {
    let mut cmd = cargo_bin_cmd!("music-cli");
    cmd.args([
        "resolve",
        "chord",
        "--chord",
        "V7",
        "--in",
        "Cmaj",
    ])
    .assert()
    .success()
    .stdout(predicates::str::contains("Resolution in Cmaj (12tet)"))
    .stdout(predicates::str::contains("Suggested voice-leading"))
    // Leading tone B -> C (+1)
    .stdout(predicates::str::contains("pc 11"))
    .stdout(predicates::str::contains("pc 0"))
    .stdout(predicates::str::contains("+1 st"));
}

#[test]
fn resolve_notes_to_i_c_major_text() {
    let mut cmd = cargo_bin_cmd!("music-cli");
    cmd.args([
        "resolve",
        "notes",
        "--notes",
        "71,65,67,62",
        "--in",
        "Cmaj",
    ])
    .assert()
    .success()
    .stdout(predicates::str::contains("Resolution in Cmaj"))
    .stdout(predicates::str::contains("Notes 71,65,67,62"));
}

#[test]
fn resolve_json_output() {
    let mut cmd = cargo_bin_cmd!("music-cli");
    cmd.args([
        "--format",
        "json",
        "resolve",
        "chord",
        "--chord",
        "V7",
        "--in",
        "Cmaj",
    ])
    .assert()
    .success()
    .stdout(predicates::str::contains("\"resolutions\""))
    .stdout(predicates::str::contains("\"from_pc\""))
    .stdout(predicates::str::contains("\"to_pc\""));
}
