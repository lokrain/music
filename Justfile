set shell := ["bash", "-cu"]

CRATES := "core/music-acoustic core/music-theory core/music-time notation/music-notation notation/music-articulation notation/music-engraving score/music-score"

check:
	cargo check --workspace --all-targets --all-features

lint:
	cargo fmt --check
	RUSTC_WRAPPER= cargo clippy --all-targets --all-features -- -W clippy::pedantic -W clippy::unwrap_used -W clippy::expect_used

fmt:
	cargo fmt

## run workspace tests
test:
	RUSTC_WRAPPER= cargo test --workspace

## build docs without dependencies
 doc:
	cargo doc --workspace --no-deps --all-features

## run benchmarks (if any)
 bench:
	cargo bench --workspace

## ensure each publishable crate can be packaged
 release-dry-run:
	for crate in {{CRATES}}; do \
		echo "Packaging $crate"; \
		( cd "$crate" && cargo package --locked --allow-dirty --no-verify ); \
	done
