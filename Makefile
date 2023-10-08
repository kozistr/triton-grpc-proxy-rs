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
	docker build . -t triton-proxy
	docker run --rm --network=host --shm-size=2g -it -p8080:8080 triton-proxy

run-example:
	./run_client.sh
