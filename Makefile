RUST_TOOLCHAIN := $(shell cat tests/rust-toolchain)

prepare:
	rustup update
	rustup toolchain install $(RUST_TOOLCHAIN)
	rustup target add --toolchain $(RUST_TOOLCHAIN) wasm32-unknown-unknown

build-contract:
	cd contract && cargo build --release

test:
	cd tests && cargo test

lint:
	cd contract && cargo clippy --all-targets -- -D warnings -A renamed_and_removed_lints
	cd contract && cargo fmt
	cd tests && cargo clippy --all-targets -- -D warnings -A renamed_and_removed_lints
	cd tests && cargo fmt

clean:
	cd contract && cargo clean
	cd tests && cargo clean && rm -rf wasm/*
