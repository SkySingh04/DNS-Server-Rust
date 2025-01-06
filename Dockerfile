FROM rust:1.73 as builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./

COPY . .


RUN cargo build --release

FROM debian:bullseye-slim

RUN apt-get update && apt-get install -y \
    ca-certificates && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/DNS-Server-Rust /usr/local/bin/

CMD ["DNS-Server-Rust"]
