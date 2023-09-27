.PHONY: format client server builder lint

update:
	cargo upgrade

format:
	cargo +nightly fmt

server:
	cargo run --release --bin server

lint:
	cargo clippy
