RFLAGS="-C link-arg=-s"

define compile_release
	@rustup target add wasm32-unknown-unknown
	RUSTFLAGS=$(RFLAGS) cargo build -p $(1) --target wasm32-unknown-unknown --release
endef

build_demo: demo
	$(call compile_release,demo)
	cp target/wasm32-unknown-unknown/release/demo.wasm ./demo.wasm

test: demo
	cargo test
