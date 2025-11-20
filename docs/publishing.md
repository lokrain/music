## Publishing Playbook (v1)

1. **Tag & version bump**
   - Update each crate’s `Cargo.toml` with the release version (e.g., `1.0.0`).
   - For inter-crate dependencies, replace `path = "../..."` with version requirements (`music-theory = "1.0"`). Use `[patch]` overrides locally during staging if needed.
   - Commit with message `chore: prep v1.0.0 release`.

2. **Dry-run ordering**
   - `cargo publish -p music-acoustic --dry-run`
   - `cargo publish -p music-theory --dry-run`
   - `cargo publish -p music-time --dry-run`
   - `cargo publish -p music-notation --dry-run`
   - `cargo publish -p music-articulation --dry-run`
   - `cargo publish -p music-engraving --dry-run`
   - `cargo publish -p music-score --dry-run`
   - Each dry run must use versioned dependencies (no `path`).

3. **Actual publish**
   - Repeat the same order as above, waiting for crates.io indexing (~5 minutes) between publishes so downstream crates resolve.

4. **Verification**
   - Create a scratch project (`cargo new music-release-check`) and add dependencies on the published crates (versions `1.0`).
   - Run `cargo check` to ensure published artifacts resolve.

5. **Documentation**
   - Push the release tag and update GitHub releases with the contents of `docs/release_notes.md`.
