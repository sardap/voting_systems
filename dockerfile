# React frotnend
FROM node:18.7.0 as frontend_builder
WORKDIR /app
# Seprated for caching
COPY ./frontend/package.json .
COPY ./frontend/package-lock.json .
RUN npm install .
COPY ./frontend .
RUN npm run build

# Backend Build
FROM rust:1.65.0-buster as backend_builder

ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get update && apt-get install -y pkg-config libssl-dev libpq-dev

RUN rustup update
RUN rustup default nightly

RUN mkdir app
WORKDIR /app

COPY ./Cargo.lock Cargo.lock
COPY ./Cargo.toml Cargo.toml
COPY ./src src

RUN cargo build --release

RUN mkdir /app/config

# Runner
FROM debian:buster-20230411-slim

RUN apt-get update && apt-get install -y pkg-config libssl-dev libpq-dev

RUN useradd -ms /bin/bash mr_vote

RUN mkdir /app
WORKDIR /app

COPY --from=backend_builder /app/target/release/backend /app/release/backend
COPY --from=frontend_builder /app/dist /app/frontend

RUN chown -R mr_vote /app

USER mr_vote

EXPOSE 8080

ENV RUST_BACKTRACE=full
ENV BUILD_DIR=/app/frontend

WORKDIR /app

CMD ["/app/release/backend"]
