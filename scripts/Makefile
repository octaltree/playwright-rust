target/generated.rs: ../src/api/1.11.0-next-1617207797000.json
	rustfmt --emit stdout <(cargo run < $^)| tail +3 > $@
