all: format lint test doc

d:
	cargo watch -c -s 'make all'

format:
	cargo fmt

lint:
	cargo clippy --no-default-features --features rt-tokio --all-targets
	cargo clippy --no-default-features --features rt-actix --all-targets
	cargo clippy --no-default-features --features rt-async-std --all-targets

test:
	cargo test --no-default-features --features rt-tokio --all-targets
	cargo test --no-default-features --features rt-actix --all-targets
	cargo test --no-default-features --features rt-async-std --all-targets

doc:
	cargo doc
