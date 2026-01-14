FROM lukemathwalker/cargo-chef:latest-rust-1.75 AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN echo "=== CONTENT OF ROOT CARGO.TOML ===" && cat Cargo.toml
RUN echo "=== FILE STRUCTURE ===" && ls -R apps
RUN cargo chef prepare --recipe-path recipe.json --bin user-management

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json --bin user-management
COPY . .
RUN cargo build --release --bin user-management

FROM debian:bookworm-slim AS runtime
WORKDIR /app
RUN apt-get update && apt-get install -y ca-certificates libssl-dev && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/user-management /usr/local/bin/user-management
ENTRYPOINT ["/usr/local/bin/user-management"]