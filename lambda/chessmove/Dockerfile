FROM rust:1.57-bullseye AS builder

WORKDIR /build
COPY src/ src/
COPY Cargo.toml .
COPY Cargo.lock .

RUN cargo build --release

CMD ["/bin/bash"]
