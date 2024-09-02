FROM rust:1-slim-buster AS build

RUN cargo new --bin app
WORKDIR /app

COPY Cargo.toml /app/
COPY Cargo.lock /app/

RUN cargo build --release

ENV POSTGRES_DB=rinha_backend
ENV POSTGRES_PASSWORD=root
ENV POSTGRES_USER=postgres
ENV POSTGRES_HOST=db

COPY  src /app/src
COPY db /app/db
RUN touch /app/src/main.rs
RUN cargo build --release

FROM debian:buster-slim

COPY --from=build /app/target/release/rust-rinha-backend-2024 /app/rinha

CMD ["/app/rinha"]
