FROM rust:1.57-bullseye AS builder

WORKDIR /build
COPY . .
ARG APP_DIR
RUN cargo build --release --manifest-path "$APP_DIR/Cargo.toml"

FROM amazonlinux:2

WORKDIR /app
ARG APP_DIR
ARG APP_NAME
COPY --from=builder "/build/$APP_DIR/target/release/$APP_NAME" "bootstrap"
ENTRYPOINT ["/app/bootstrap"]
