FROM rust:1.72.1-slim AS chef

WORKDIR /workspace

RUN echo "export RUSTFLAGS=-C target-feature=native" >> /etc/bash.bashrc

RUN set -eux; \
    apk add --no-cache musl-dev; \
    cargo install cargo-chef; \
    rm -rf $CARGO_HOME/registry

FROM chef AS planner

COPY . .
RUN cargo chef prepare --recipe_path recipe.json

FROM chef AS builder

COPY --from=planner /workspace/recipe.json .
RUN cargo chef cook --release --recipe-path recipe.json

COPY . .
RUN cargo build --release

FROM alpine:latest

WORKDIR /usr/local/bin

COPY --from=builder /workspace/target/release/triton_grpc_proxy .

CMD ["./triton_grpc_proxy"]
