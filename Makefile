.PHONY: all build

all:
	@echo "No default make command. Try one of the following:";
	@echo "-> build	Compiles rust and preps wasm bindings";
	@echo "-> test	Runs rust tests";

build:
	# Build the project & creating wasm bindings
	cargo +nightly build --target wasm32-unknown-unknown
	wasm-bindgen target/wasm32-unknown-unknown/debug/z80_emulator.wasm \
		--out-dir ./pkg
	# Link built pkg to www source folder.
	cd www && npm link z80-emulator


test:
	cargo test