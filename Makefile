run:
	cargo run

watch:
	watchexec -w src -r "cargo run"

fmt:
	cargo fmt

lint:
	cargo clippy

test:
	cargo test
