use assert_cmd::cargo::cargo_bin_cmd;

#[test]
fn map_scale_basic() {
    let mut cmd = cargo_bin_cmd!("music-cli");
    cmd.args(["map", "scale", "--root", "60"])
        .assert()
        .success()
        .stdout(predicates::str::contains("Scale map for Major"))
        .stdout(predicates::str::contains("Pitch-class layout:"));
}

#[test]
fn map_scale_with_modulations() {
    let mut cmd = cargo_bin_cmd!("music-cli");
    cmd.args(["map", "scale", "--root", "60", "--modulations", "3"])
        .assert()
        .success()
        .stdout(predicates::str::contains("Modulatory paths:"))
        .stdout(predicates::str::contains("Rotation 1:"))
        .stdout(predicates::str::contains("Rotation 2:"))
        .stdout(predicates::str::contains("Rotation 3:"));
}

#[test]
fn map_scale_minor() {
    let mut cmd = cargo_bin_cmd!("music-cli");
    cmd.args(["map", "scale", "--root", "60", "--scale", "minor"])
        .assert()
        .success()
        .stdout(predicates::str::contains("Scale map for Minor"));
}

#[test]
fn map_scale_json_output() {
    let mut cmd = cargo_bin_cmd!("music-cli");
    cmd.args([
        "--format", "json", "map", "scale", "--root", "60", "--scale", "major",
    ])
    .assert()
    .success()
    .stdout(predicates::str::contains("\"system\": \"12tet\""))
    .stdout(predicates::str::contains("\"root_index\": 60"))
    .stdout(predicates::str::contains("pitch_class_map"))
    .stdout(predicates::str::contains("modulatory_paths"));
}

#[test]
fn map_scale_dorian() {
    let mut cmd = cargo_bin_cmd!("music-cli");
    cmd.args(["map", "scale", "--root", "62", "--scale", "dorian"])
        .assert()
        .success()
        .stdout(predicates::str::contains("Scale map for Dorian"));
}

#[test]
fn map_scale_different_system() {
    let mut cmd = cargo_bin_cmd!("music-cli");
    cmd.args(["map", "scale", "--root", "60", "--system", "24tet"])
        .assert()
        .success()
        .stdout(predicates::str::contains("24tet"));
}

#[test]
fn map_scale_no_modulations() {
    let mut cmd = cargo_bin_cmd!("music-cli");
    cmd.args(["map", "scale", "--root", "60", "--modulations", "0"])
        .assert()
        .success()
        .stdout(predicates::str::contains("Pitch-class layout:"));
}
