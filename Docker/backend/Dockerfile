FROM ubuntu:24.04 as builder

RUN apt-get update && apt-get install -y \
    curl \
    build-essential \
    pkg-config \
    libgtk-3-dev \
    libwebkit2gtk-4.1-dev \
    libappindicator3-dev \
    librsvg2-dev \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Installer Rust
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

WORKDIR /app
COPY uniliste/src-tauri/Cargo.toml uniliste/src-tauri/Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release || true

COPY uniliste/src-tauri/ .
RUN cargo build --release

FROM ubuntu:24.04

RUN apt-get update && apt-get install -y \
    libgtk-3-0 \
    libwebkit2gtk-4.1-0 \
    libappindicator3-1 \
    librsvg2-2 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /app/target/release/uniliste /usr/local/bin/uniliste

EXPOSE 8000
CMD ["uniliste"]
