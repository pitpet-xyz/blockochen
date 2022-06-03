all:
	cargo wasi build --release
	file target/wasm32-wasi/release/blockochen.wasm
