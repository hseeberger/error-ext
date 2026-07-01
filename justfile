set shell := ["bash", "-uc"]

nightly := `rustc --version | grep -oE '[0-9]{4}-[0-9]{2}-[0-9]{2}' | sed 's/^/nightly-/'`

check:
    cargo check --tests
    cargo check --tests --features axum
    cargo check --tests --features axum,axum-utoipa

fix:
    cargo fix --tests --all-features --allow-dirty --allow-staged

fmt:
    cargo +{{ nightly }} fmt
    RUST_LOG=error taplo fmt

fmt-check:
    cargo +{{ nightly }} fmt --check

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
    RUSTDOCFLAGS="-D warnings --cfg docsrs" cargo +{{ nightly }} doc --no-deps --all-features

all: check fmt lint test doc
