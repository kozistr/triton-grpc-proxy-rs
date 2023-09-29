.PHONY: update format server lint

update:
	cargo upgrade

format:
	cargo +nightly fmt

server:
	cargo run --release

lint:
	cargo clippy
