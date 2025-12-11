set shell := ["bash", "-uc"]

rust_version := `grep channel rust-toolchain.toml | sed -r 's/channel = "(.*)"/\1/'`
nightly := "nightly-2025-12-08"

check:
	cargo check --tests
	cargo check --tests --features axum
	cargo check --tests --features axum,axum-utoipa

fmt:
    cargo +{{nightly}} fmt

fmt-check:
    cargo +{{nightly}} fmt --check

fix:
	cargo fix --tests --all-features --allow-dirty --allow-staged

lint:
	cargo clippy --tests --no-deps                             -- -D warnings
	cargo clippy --tests --no-deps --features axum             -- -D warnings
	cargo clippy --tests --no-deps --features axum,axum-utoipa -- -D warnings

lint-fix:
	cargo clippy --tests --no-deps --all-features --allow-dirty --allow-staged --fix

test:
	cargo test --tests
	cargo test --tests --features axum
	cargo test --tests --features axum,axum-utoipa

doc:
	RUSTDOCFLAGS="-D warnings --cfg docsrs" cargo +{{nightly}} doc --no-deps --all-features

all: check fmt lint test doc
