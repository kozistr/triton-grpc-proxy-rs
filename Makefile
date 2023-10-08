.PHONY: update format lint build

update:
	cargo upgrade

format:
	cargo +nightly fmt

lint:
	cargo +nightly clippy

build:
	cargo run --release --bin server

build-docker:
	docker build .
	docker run -it -p8080:8080 -p8001:8001

run-example:
	./run_client.sh
