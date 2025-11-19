#[cfg(feature = "schema")]
use assert_cmd::cargo::cargo_bin_cmd;

#[cfg(feature = "schema")]
#[test]
fn export_schemas_creates_json_files() {
    let temp_dir = std::env::temp_dir().join("music-schema-test");
    if temp_dir.exists() {
        std::fs::remove_dir_all(&temp_dir).ok();
    }

    cargo_bin_cmd!("music-cli")
        .args(["export-schemas", "--output-dir"])
        .arg(&temp_dir)
        .assert()
        .success()
        .stdout(predicates::str::contains("ChordResolutionReport.json"))
        .stdout(predicates::str::contains("EstimateReport.json"))
        .stdout(predicates::str::contains("Successfully exported 41 schemas"));

    // Verify some key schema files exist
    assert!(temp_dir.join("ChordResolutionReport.json").exists());
    assert!(temp_dir.join("EstimateReport.json").exists());
    assert!(temp_dir.join("PitchExplanation.json").exists());

    // Cleanup
    std::fs::remove_dir_all(&temp_dir).ok();
}

#[cfg(feature = "schema")]
#[test]
fn exported_schemas_are_valid_json() {
    let temp_dir = std::env::temp_dir().join("music-schema-test-json");
    if temp_dir.exists() {
        std::fs::remove_dir_all(&temp_dir).ok();
    }

    cargo_bin_cmd!("music-cli")
        .args(["export-schemas", "--output-dir"])
        .arg(&temp_dir)
        .assert()
        .success();

    // Read and parse a schema file to ensure it's valid JSON
    let schema_file = temp_dir.join("ChordResolutionReport.json");
    let content = std::fs::read_to_string(&schema_file).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&content).unwrap();

    // Verify it has expected JSON Schema structure
    assert!(parsed.get("$schema").is_some());
    assert!(parsed.get("title").is_some());
    assert_eq!(parsed.get("type").and_then(|v| v.as_str()), Some("object"));

    // Cleanup
    std::fs::remove_dir_all(&temp_dir).ok();
}
