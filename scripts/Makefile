../src/api/generated.rs: ../src/api/api.json
	rustfmt --emit stdout <(cargo run < $^)| tail +3 > $@
