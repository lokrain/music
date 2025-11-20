use std::path::Path;
use std::process::Command;

use assert_cmd::prelude::*;
use predicates::str::contains;
use tempfile::TempDir;

const TEMPLATE_ENV: &str = "MUSIC_CLI_TEMPLATES_DIR";

fn cli_command() -> Command {
    Command::new(assert_cmd::cargo::cargo_bin!("music-cli"))
}

#[test]
fn templates_list_includes_builtin() {
    let mut cmd = cli_command();
    cmd.arg("templates").arg("list").arg("--source").arg("builtin");
    cmd.assert().success().stdout(contains("jazz_aaba_v1"));
}

#[test]
fn templates_import_export_roundtrip() {
    let temp_dir = TempDir::new().expect("temp dir");
    let data_dir = temp_dir.path().join("templates_store");
    std::fs::create_dir_all(&data_dir).expect("templates dir");
    let fixture =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/simple_template.json5");

    let mut import = cli_command();
    import.env(TEMPLATE_ENV, &data_dir).arg("templates").arg("import").arg(&fixture);
    import.assert().success().stdout(contains("Imported template 'test_template_v1"));

    let mut list = cli_command();
    list.env(TEMPLATE_ENV, &data_dir).arg("templates").arg("list").arg("--source").arg("local");
    list.assert().success().stdout(contains("test_template_v1"));

    let export_path = data_dir.join("exported.json5");
    let mut export = cli_command();
    export
        .env(TEMPLATE_ENV, &data_dir)
        .arg("templates")
        .arg("export")
        .arg("test_template_v1")
        .arg("--source")
        .arg("local")
        .arg("--output")
        .arg(&export_path)
        .arg("--overwrite");
    export.assert().success().stdout(contains("Exported template 'test_template_v1"));

    let exported = std::fs::read_to_string(&export_path).expect("exported file");
    assert!(exported.contains("test_template_v1"));
}
