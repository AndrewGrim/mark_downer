debug:
	cargo build && cargo test -- --show-output
release:
	cargo build --release