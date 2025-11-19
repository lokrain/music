fn main() {
    #[cfg(feature = "schema")]
    generate_schema_marker();
}

#[cfg(feature = "schema")]
fn generate_schema_marker() {
    use std::{fs, path::PathBuf};
    let out_dir = PathBuf::from("crates/music-cli/schemas");
    let _ = fs::create_dir_all(&out_dir);
    let marker = out_dir.join("schema_feature_active.txt");
    fs::write(marker, "schema feature build script executed").expect("write marker");
    // Placeholder: real schema generation will move to runtime module to avoid build.rs import issues.
}
