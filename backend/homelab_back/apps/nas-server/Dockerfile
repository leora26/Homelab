FROM lukemathwalker/cargo-chef:latest-rust-1.75 AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN rm Cargo.lock || true
RUN cargo chef prepare --recipe-path recipe.json --bin nas-server

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json

RUN cargo chef cook --release --recipe-path recipe.json --bin nas-server
COPY . .
RUN cargo build --release --bin nas-server

FROM debian:bookworm-slim AS runtime
WORKDIR /app
RUN apt-get update && apt-get install -y ca-certificates libssl-dev && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/nas-server /usr/local/bin/nas-server
ENTRYPOINT ["/usr/local/bin/nas-server"]