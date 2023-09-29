FROM rust:1.72.1-slim

RUN echo "export RUSTFLAGS=-C target-feature=native" >> /etc/bash.bashrc

COPY . ./workspace

WORKDIR /workspace

RUN make server

EXPOSE 8080

CMD ["./target/release/triton_grpc_proxy"]
