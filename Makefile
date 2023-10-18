.PHONY: update format lint build

update:
	cargo upgrade

format:
	cargo +nightly fmt

lint:
	cargo +nightly clippy --all-features --all-targets -- -D warnings

build:
	cargo run --release --bin server

build-pgo:
	cargo pgo build

build-docker:
	docker build . -t triton-proxy

run-docker:
	docker run --rm --shm-size=2g -it -p8080:8080 triton-proxy

run-docker-compose:
	docker-compose up -d

run-example:
	./run_client.sh
