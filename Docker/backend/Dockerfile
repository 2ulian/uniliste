FROM rust:1.83 as builder

WORKDIR /app

COPY backend/Cargo.toml backend/Cargo.lock ./
RUN mkdir -p src && echo "fn main() {}" > src/main.rs
RUN cargo build --release

# Build application
COPY backend/ .
RUN cargo clean
RUN cargo build --release

FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /app/target/release/uniliste-backend /usr/local/bin/uniliste-backend

# Ensure the binary is executable
RUN chmod +x /usr/local/bin/uniliste-backend

EXPOSE 8000
CMD ["/usr/local/bin/uniliste-backend"]
