FROM rust:1.82 as builder

WORKDIR /app

# Cache dependencies (Cargo.lock not present in repo yet)
COPY backend/Cargo.toml ./
RUN mkdir -p src && echo "fn main() {}" > src/main.rs
RUN cargo build --release || true

# Build application
COPY backend/ .
RUN cargo build --release

FROM debian:bookworm-slim

WORKDIR /app
COPY --from=builder /app/target/release/uniliste-backend /usr/local/bin/uniliste-backend

EXPOSE 8000
CMD ["/usr/local/bin/uniliste-backend"]
