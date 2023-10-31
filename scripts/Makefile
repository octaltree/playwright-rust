.PHONY: clean nop
nop:

SRC=$(shell find src)

clean_driver:
	rm -r /tmp/build-playwright-rust
	rm -r ~/.cache/ms-playwright/playwright-rust

../src/api/api.json: ../src/build.rs
	cd .. && cargo run print-api-json |jq > src/api/api.json

../src/protocol/protocol.yml: ../src/api/api.json
	cp ~/.cache/ms-playwright/playwright-rust/driver/package/protocol.yml $@

../src/api/generated.rs: ../src/api/api.json ${SRC}
	cargo run --bin generate_api < $< > $@
	rustfmt +nightly --config-path ../.rustfmt.toml $@

../src/protocol/generated.rs: ../src/protocol/protocol.yml ${SRC}
	cat $<| sed 's#null#"null"#'| cargo run --bin generate_protocol > $@
	rustfmt +nightly --config-path ../.rustfmt.toml $@

diff:
	cargo run --bin diff <(git show master:src/api/api.json) ../src/api/api.json
