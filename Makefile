all:
	cargo build --release --target wasm32-unknown-unknown
	file target/wasm32-unknown-unknown/release/blockochen.wasm
