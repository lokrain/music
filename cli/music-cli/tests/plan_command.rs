use assert_cmd::prelude::*;
use predicates::prelude::*;
use predicates::str::contains;
use std::process::Command;

#[test]
fn plan_builtin_template_outputs_text_and_json() {
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("music-cli"));
    cmd.arg("plan")
        .arg("--template")
        .arg("jazz_aaba_v1")
        .arg("--tonic")
        .arg("C")
        .arg("--mode")
        .arg("major")
        .arg("--style")
        .arg("balanced")
        .arg("--explain")
        .arg("brief");

    cmd.assert().success().stdout(
        contains("Template: jazz_aaba_v1")
            .and(contains("--- JSON report ---"))
            .and(contains("\"template\""))
            .and(contains("\"style\": \"balanced\"")),
    );
}

#[test]
fn plan_requires_explicit_template_source() {
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("music-cli"));
    cmd.arg("plan");

    cmd.assert().failure().stderr(contains("template not specified"));
}
