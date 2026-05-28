# Build:
#   docker build -t ardent .
#
# Usage:
#   docker run --rm -v "$PWD:/work" ardent <command> [options] [file...]

FROM rust:1-alpine AS builder

RUN apk add --no-cache musl-dev

WORKDIR /build
COPY Cargo.toml Cargo.lock ./
COPY src/ src/

RUN cargo build --release

FROM alpine:3

WORKDIR /work

COPY --from=builder /build/target/release/ardent /usr/local/bin/ardent

ENTRYPOINT ["ardent"]
