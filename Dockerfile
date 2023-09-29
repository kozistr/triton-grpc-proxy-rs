FROM rust:1.72.1-slim AS builder

WORKDIR /workspace

COPY . /workspace

RUN echo "export RUSTFLAGS=-C target-feature=native" >> /etc/bash.bashrc

RUN make server

FROM alpine:latest

WORKDIR /usr/local/bin

COPY --from=builder /workspace/target/release/triton_grpc_proxy .

CMD ["./triton_grpc_proxy"]
