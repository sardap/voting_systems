# React frotnend
FROM node:18.7.0 AS frontend_builder
WORKDIR /app
# Seprated for caching
COPY ./frontend/package.json .
COPY ./frontend/package-lock.json .
RUN npm install .
COPY ./frontend .
RUN npm run build

# Backend Build
FROM rust:1.67.0-buster AS backend_builder

ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get update && apt-get install -y pkg-config libssl-dev libpq-dev

RUN rustup update

RUN mkdir app
WORKDIR /app

COPY ./backend ./backend
COPY ./systems ./systems
COPY ./Cargo.toml .
COPY ./Cargo.lock .
COPY rust-toolchain .

RUN cargo build --release

RUN mkdir /app/config

# Runner
FROM debian:buster-20230411-slim

RUN apt-get update && apt-get install -y pkg-config libssl-dev libpq-dev

RUN useradd -ms /bin/bash mr_vote

RUN mkdir /app
WORKDIR /app

COPY --from=backend_builder /app/target/release/voting-systems-site-backend /app/release/backend
COPY --from=frontend_builder /app/dist /app/frontend

RUN chown -R mr_vote /app

USER mr_vote

EXPOSE 8080

ENV RUST_BACKTRACE=full
ENV BUILD_DIR=/app/frontend

WORKDIR /app

CMD ["/app/release/backend"]
