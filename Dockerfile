FROM rust:1.78 as builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release

COPY . .
RUN cargo build --release

FROM debian:bullseye-slim

RUN apt-get update && \
    apt-get install -y libssl-dev libsqlite3-dev && \
    rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/rust-backend-starter /usr/local/bin/rust-backend-starter

WORKDIR /data
VOLUME ["/data"]

ENTRYPOINT ["/usr/local/bin/rust-backend-starter"]

EXPOSE 8080

CMD ["rust-backend-starter"]