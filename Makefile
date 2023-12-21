run:
	cargo run

watch:
	cargo watch -w src -x run

fmt:
	cargo fmt

lint:
	cargo clippy

test:
	cargo test
