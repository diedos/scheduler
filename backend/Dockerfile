FROM rust:bookworm AS base

WORKDIR /opt
RUN cargo init
COPY Cargo.toml Cargo.lock ./
RUN cargo fetch
COPY src/ ./src/
COPY migrations/ ./migrations/

FROM base AS development

RUN cargo install cargo-watch
EXPOSE 3000
CMD [ "cargo", "watch", "-x", "run" ]

FROM base AS builder

RUN cargo build --release --offline

FROM debian:bookworm-slim

EXPOSE 3000
COPY --from=builder /opt/target/release/todo-rs-svelte-backend /opt/todo-rs-svelte-backend
CMD [ "/opt/todo-rs-svelte-backend" ]