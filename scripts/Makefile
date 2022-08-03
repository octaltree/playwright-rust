.PHONY: clean nop
nop:

clean_driver:
	rm -r /tmp/build-playwright-rust
	rm -r ~/.cache/ms-playwright/playwright-rust

../src/api/generated.rs: ../src/api/api.json src/gen.rs
	cargo run --bin gen < $< > $@
	rustfmt +nightly --config-path ../.rustfmt.toml $@

../src/api/api.json: ../src/build.rs
	cd .. && cargo run print-api-json |jq > src/api/api.json

diff:
	cargo run --bin diff <(git show master:src/api/api.json) ../src/api/api.json

../src/protocol/protocol.yml: ../src/build.rs
	cd .. && cargo run print-api-json
	cp ~/.cache/ms-playwright/playwright-rust/driver/package/protocol.yml $@

../src/protocol/generated.rs: ../src/protocol/protocol.yml src/generate_protocol.rs
	cat $<| sed 's#null#"null"#'| cargo run --bin generate_protocol > $@
	rustfmt +nightly --config-path ../.rustfmt.toml $@
