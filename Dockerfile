FROM clux/muslrust:stable as chef

USER root
WORKDIR /app

RUN echo "export RUSTFLAGS=-C target-cpu=native" >> /etc/bash.bashrc
RUN cargo install cargo-chef

FROM chef AS planner

COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder

COPY --from=planner /app/recipe.json .
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release --target x86_64-unknown-linux-musl --bin server

FROM alpine:latest AS runtime

WORKDIR /usr/local/bin

EXPOSE 8001 8080

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/server .

CMD ["./server"]
