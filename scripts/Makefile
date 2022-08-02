.PHONY: clean nop
nop:

clean_driver:
	rm -r /tmp/build-playwright-rust
	rm -r ~/.cache/ms-playwright/playwright-rust

../src/api/generated.rs: ../src/api/api.json src/gen.rs
	rustfmt --config-path ../.rustfmt.toml --emit stdout <(cargo run --bin gen < $^)| tail +3 > $@

../src/api/api.json: ../src/build.rs
	cd .. && cargo run print-api-json |jq > src/api/api.json

diff:
	cargo run --bin diff <(git show master:src/api/api.json) ../src/api/api.json

../src/protocol/protocol.yml: ../src/build.rs
	cd .. && cargo run print-api-json
	cp ~/.cache/ms-playwright/playwright-rust/driver/package/protocol.yml $@

../src/protocol/generated.rs:../src/protocol/protocol.yml src/generate_protocol.rs
	rustfmt --config-path ../.rustfmt.toml --emit stdout <(cargo run --bin generate_protocol < $^)| tail +3 > $@
