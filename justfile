set shell := ["bash", "-uc"]

nightly := `rustc --version | grep -oE '[0-9]{4}-[0-9]{2}-[0-9]{2}' | sed 's/^/nightly-/'`

check:
    cargo check --all-targets
    cargo check --all-targets --features axum
    cargo check --all-targets --features axum,axum-utoipa

fix:
    cargo fix --all-targets --all-features --allow-dirty --allow-staged

fmt:
    cargo +{{ nightly }} fmt
    RUST_LOG=error taplo fmt

fmt-check:
    cargo +{{ nightly }} fmt --check

lint:
    cargo clippy --all-targets --no-deps                             -- -D warnings
    cargo clippy --all-targets --no-deps --features axum             -- -D warnings
    cargo clippy --all-targets --no-deps --features axum,axum-utoipa -- -D warnings

lint-fix:
    cargo clippy --all-targets --no-deps --all-features --allow-dirty --allow-staged --fix

test:
    cargo test
    cargo test --features axum
    cargo test --features axum,axum-utoipa

doc:
    RUSTDOCFLAGS="-D warnings --cfg docsrs" cargo +{{ nightly }} doc --no-deps --all-features

all: check fmt lint test doc
