set shell := ["bash", "-uc"]

check:
	cargo check
	cargo check --features axum

fmt toolchain="+nightly":
	cargo {{toolchain}} fmt

fmt-check toolchain="+nightly":
	cargo {{toolchain}} fmt --check

lint:
	cargo clippy --no-deps --all-features -- -D warnings

test:
	cargo test --all-features

fix:
	cargo fix --allow-dirty --allow-staged --all-features

doc toolchain="+nightly":
	RUSTDOCFLAGS="-D warnings --cfg docsrs" cargo {{toolchain}} doc --no-deps --all-features

all: check fmt lint test doc
