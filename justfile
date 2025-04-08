set shell := ["bash", "-uc"]

check:
	cargo check --tests
	cargo check --tests --features axum
	cargo check --tests --features axum,utoipa

fmt toolchain="+nightly":
	cargo {{toolchain}} fmt

fmt-check toolchain="+nightly":
	cargo {{toolchain}} fmt --check

lint:
	cargo clippy --tests --no-deps                        -- -D warnings
	cargo clippy --tests --no-deps --features axum        -- -D warnings
	cargo clippy --tests --no-deps --features axum,utoipa -- -D warnings

test:
	cargo test --all-features

fix:
	cargo fix --allow-dirty --allow-staged --all-features

doc toolchain="+nightly":
	RUSTDOCFLAGS="-D warnings --cfg docsrs" cargo {{toolchain}} doc --no-deps --all-features

all: check fmt lint test doc
