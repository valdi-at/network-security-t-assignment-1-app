FROM rust:alpine AS builder
RUN apk add --no-cache musl-dev build-base pkgconfig ca-certificates perl

RUN rustup target add wasm32-unknown-unknown
RUN cargo install cargo-leptos --locked
RUN apk add --no-cache npm
RUN npm install -g sass

WORKDIR /app

ENV CARGO_BUILD_TARGET=x86_64-unknown-linux-musl
ENV LEPTOS_BIN_TARGET_TRIPLE=x86_64-unknown-linux-musl

COPY Cargo.toml Cargo.lock ./
RUN mkdir src \
    && echo "" > src/lib.rs \
    && echo "fn main() {}" > src/main.rs
RUN cargo build \
    --release \
    --target x86_64-unknown-linux-musl \
    --no-default-features \
    --features ssr
RUN rm -rf src

COPY package.json package-lock.json ./
RUN npm install

COPY . .
RUN cargo leptos build --release
RUN find | grep site && sleep 10

FROM alpine:latest
RUN apk add --no-cache ca-certificates
WORKDIR /app
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/network-security-t-assignment-1-app /app/network-security-t-assignment-1-app
COPY --from=builder /app/target/site /app/site
ENTRYPOINT ["/app/network-security-t-assignment-1-app"]