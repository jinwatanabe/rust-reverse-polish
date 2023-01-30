FROM rust:alpine3.17

RUN apk update && apk add git alpine-sdk make libffi-dev openssl-dev pkgconfig
RUN rustup component add rls rust-analysis rustfmt clippy
RUN cargo install sqlx-cli cargo-watch

WORKDIR /app/src