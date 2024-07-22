FROM rust:alpine AS builder
WORKDIR /usr/src/app

COPY . .
RUN apk add --no-cache \
    build-base \
    libressl-dev \
    musl-dev \
    curl \
    gcc

RUN rustup target add x86_64-unknown-linux-musl
RUN cargo build --target x86_64-unknown-linux-musl --release

# We do not need the Rust toolchain to run the binary!
FROM alpine:latest AS bot
COPY --from=builder /usr/src/app/target/x86_64-unknown-linux-musl/release/bot ./bot

ENTRYPOINT ["./bot"]