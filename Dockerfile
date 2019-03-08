FROM rust as build

RUN rustup target add x86_64-unknown-linux-musl

RUN mkdir /app
WORKDIR /src

COPY src src
COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock

RUN cargo install --path . --root /app --target x86_64-unknown-linux-musl

FROM alpine

WORKDIR /app

COPY --from=build /app/bin/csv2json /usr/bin/csv2json
