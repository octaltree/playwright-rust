all: format lint test doc

d:
	cargo watch -c -s 'make all'

format:
	cargo fmt

lint:
	cargo clippy --features runtime-actix --all-targets
	cargo clippy --features runtime-async-std --all-targets
	cargo clippy --features runtime-tokio --all-targets

test:
	cargo test --features runtime-actix --all-targets
	cargo test --features runtime-async-std --all-targets
	cargo test --features runtime-tokio --all-targets

doc:
	cargo doc --features runtime-actix
	cargo doc --features runtime-async-std
	cargo doc --features runtime-tokio
