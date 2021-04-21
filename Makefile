prepare:
	rustup target add wasm32-unknown-unknown

build-contract:
	cargo build --release -p contract --target wasm32-unknown-unknown

test-only:
	cargo test -p tests

copy-wasm-file-to-test:
	cp target/wasm32-unknown-unknown/release/contract.wasm tests/wasm

test: build-contract copy-wasm-file-to-test test-only

check-lint:
	cargo fmt --all -- --check

lint:
	cargo fmt --all
	cargo clippy --all-targets --all -- -D warnings -A renamed_and_removed_lints

clean:
	cargo clean
	rm -rf tests/wasm/contract.wasm
