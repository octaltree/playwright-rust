all: format lint test doc build

d:
	cargo watch -c -s 'make all'

format:
	cargo fmt

lint:
	cargo clippy --all-features --all-targets

test:
	cargo test --all-features --all-targets

doc:
	cargo doc --all-features

build:
	cargo build --all-features --all-targets
