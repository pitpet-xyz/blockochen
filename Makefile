all:
	cargo wasi build
	file target/wasm32-wasi/debug/blockochen.wasm
