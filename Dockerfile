FROM rust:alpine AS mb-builder

RUN apk update
RUN apk add --no-cache sqlite-dev openssl-dev musl-dev
RUN apk add --no-cache sqlcipher-dev --repository=http://dl-cdn.alpinelinux.org/alpine/edge/testing/ --repository=http://dl-cdn.alpinelinux.org/alpine/edge/main/

WORKDIR /build

# Cache dependencies
RUN cargo new --lib libmb
RUN cargo new --bin cli

COPY ./media-bucket/Cargo.toml .
COPY ./media-bucket/Cargo.lock .
COPY ./media-bucket/cli/Cargo.toml cli/
COPY ./media-bucket/libmb/Cargo.toml libmb/

RUN RUSTFLAGS=-Ctarget-feature=-crt-static cargo build --release

# Build
COPY ./media-bucket .
RUN touch cli/src/main.rs libmb/src/lib.rs
RUN RUSTFLAGS=-Ctarget-feature=-crt-static cargo build --release -p cli

# Install
RUN install ./target/release/mb /usr/bin

FROM node:lts-alpine as webclient-builder

RUN mkdir /var/www/html -p

WORKDIR /build
COPY ./webclient .

RUN npm install
RUN npm run build-prod

RUN cp -r -v ./dist/webclient/. /var/www/html/

FROM alpine:latest

RUN apk update
RUN apk add --no-cache libcrypto3 libgcc ffmpeg imagemagick ghostscript poppler-utils

COPY --from=mb-builder /usr/bin/mb /usr/bin/mb
COPY --from=webclient-builder /var/www/html /var/www/html

ENTRYPOINT ["mb", "server"]
CMD ["-p", "80", "-a", "0.0.0.0", "-c", "/var/buckets/config.toml"]
