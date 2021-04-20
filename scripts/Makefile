../src/api/generated.rs: ../src/api/api.json
	rustfmt --emit stdout <(cargo run < $^)| tail +3 > $@

../src/api/api.json: ../src/build.rs
	cd .. && cargo run print-api-json |jq > src/api/api.json

diff:
	cargo run --bin diff <(git show master:src/api/api.json) <(git show HEAD:src/api/api.json)
