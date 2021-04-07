all: format lint test doc

dev:
	cargo fmt
	cargo clippy --all-targets
	cargo test
	cargo doc

d:
	cargo watch -c -s 'make dev'

format:
	cargo fmt

lint:
	cargo clippy --all-targets
	cargo clippy --no-default-features --features chrono --features rt-actix --all-targets
	cargo clippy --no-default-features --features chrono --features rt-async-std --all-targets

test:
	cargo test hello
	cargo tarpaulin
	cargo test --no-default-features --features chrono --features rt-actix --all-targets
	cargo test --no-default-features --features chrono --features rt-async-std --all-targets

doc:
	cargo doc
