FROM rust:slim-bookworm AS builder
WORKDIR /app

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

COPY Cargo.toml Cargo.lock ./

RUN mkdir src && \
    echo "fn main() {println!(\"if you see this, the build broke\");}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

COPY . .

ENV SQLX_OFFLINE=true

RUN touch src/main.rs && cargo build --release

FROM gcr.io/distroless/cc-debian12

COPY --from=builder /app/target/release/HomelabBack /app/HomelabBack

ENTRYPOINT ["/app/HomelabBack"]