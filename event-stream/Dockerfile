FROM rust:1.57.0-bullseye AS builder

WORKDIR /build
COPY Cargo.toml ./
COPY src/ ./src/
RUN cargo build --release

FROM debian:bullseye

RUN apt update && apt install -y ca-certificates
COPY --from=builder /build/target/release/event-stream /app
CMD /app/event-stream

