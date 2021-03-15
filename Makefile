all: format lint test doc

d:
	cargo watch -c -s 'make all'

format:
	cargo fmt

lint:
	cargo clippy --no-default-features --features chrono --features rt-tokio --all-targets
	cargo clippy --no-default-features --features chrono --features rt-actix --all-targets
	cargo clippy --no-default-features --features chrono --features rt-async-std --all-targets

test:
	cargo test hello
	cargo test --no-default-features --features chrono --features rt-tokio --all-targets
	cargo test --no-default-features --features chrono --features rt-actix --all-targets
	cargo test --no-default-features --features chrono --features rt-async-std --all-targets

doc:
	cargo doc
