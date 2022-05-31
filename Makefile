all:
	cargo wasi build --release
	file target/wasm32-wasi/debug/blockochen.wasm
