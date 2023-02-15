FROM rust:alpine AS builder

RUN apk update
RUN apk add --no-cache sqlite-dev openssl-dev musl-dev
RUN apk add --no-cache sqlcipher-dev --repository=http://dl-cdn.alpinelinux.org/alpine/edge/testing/ --repository=http://dl-cdn.alpinelinux.org/alpine/edge/main/

WORKDIR /build

# Cache dependencies
RUN cargo new --lib libmb
RUN cargo new --bin cli

COPY ./Cargo.toml .
COPY ./Cargo.lock .
COPY ./cli/Cargo.toml cli/
COPY ./libmb/Cargo.toml libmb/

RUN RUSTFLAGS=-Ctarget-feature=-crt-static cargo build --release

# Build
COPY . .
RUN touch cli/src/main.rs libmb/src/lib.rs
RUN RUSTFLAGS=-Ctarget-feature=-crt-static cargo build --release -p cli

# Install
RUN install ./target/release/mb /usr/bin

FROM alpine:latest

RUN apk update
RUN apk add --no-cache libcrypto3 libgcc ffmpeg imagemagick ghostscript poppler-utils

COPY --from=builder /usr/bin/mb /usr/bin/mb

ENTRYPOINT ["mb", "server"]
CMD ["-p", "80", "-a", "0.0.0.0", "-c", "/var/buckets/config.toml"]