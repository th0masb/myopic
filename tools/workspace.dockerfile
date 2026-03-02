FROM --platform=$BUILDPLATFORM lukemathwalker/cargo-chef:latest-rust-1.86-bookworm AS chef
WORKDIR /build

ARG TARGETARCH
ENV CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=aarch64-linux-gnu-gcc

RUN set -eux; \
    case "${TARGETARCH}" in \
      arm64) \
        apt-get update; \
        apt-get install -y --no-install-recommends \
            gcc-aarch64-linux-gnu \
            g++-aarch64-linux-gnu \
            libc6-dev-arm64-cross \
            pkg-config; \
        rustup target add aarch64-unknown-linux-gnu; \
        echo "aarch64-unknown-linux-gnu" > /tmp/rust-target; \
        rm -rf /var/lib/apt/lists/*; \
        ;; \
      amd64) \
        rustup target add x86_64-unknown-linux-gnu; \
        echo "x86_64-unknown-linux-gnu" > /tmp/rust-target; \
        ;; \
      "") \
        echo "TARGETARCH must be provided (e.g. arm64 or amd64)" >&2; \
        exit 1; \
        ;; \
      *) \
        echo "Unsupported TARGETARCH: ${TARGETARCH}" >&2; \
        exit 1; \
        ;; \
    esac

FROM --platform=$BUILDPLATFORM chef AS planner
COPY . .
RUN cargo chef prepare  --recipe-path recipe.json

FROM --platform=$BUILDPLATFORM chef AS builder
COPY --from=planner /build/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN RUST_TARGET="$(cat /tmp/rust-target)"; \
    cargo chef cook --release --recipe-path recipe.json --target "${RUST_TARGET}"
# Build application
COPY . .
ARG APP_NAME
RUN RUST_TARGET="$(cat /tmp/rust-target)"; \
    cargo build --release --target "${RUST_TARGET}" --bin "$APP_NAME"; \
    cp "/build/target/${RUST_TARGET}/release/${APP_NAME}" /build/bootstrap

FROM gcr.io/distroless/cc-debian12@sha256:c53c9416a1acdbfd6e09abba720442444a3d1a6338b8db850e5e198b59af5570
WORKDIR /app
ARG APP_NAME
ARG APP_CONFIG
ENV APP_CONFIG="$APP_CONFIG"
COPY --from=builder "/build/bootstrap" "bootstrap"
ENTRYPOINT ["/app/bootstrap"]
