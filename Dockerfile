ARG RUST_VERSION=1.44
FROM rust:${RUST_VERSION} AS build

ENV RUST_LOG=trace
EXPOSE 6000
WORKDIR /api
RUN rustup target add x86_64-unknown-linux-musl

RUN USER=root cargo new rs-gp-api
WORKDIR /api/rs-gp-api

COPY Cargo.toml Cargo.lock ./
COPY ron ./ron
COPY src ./src

RUN cargo build --release
ENTRYPOINT [ "./target/release/rs-gp-api" ]
