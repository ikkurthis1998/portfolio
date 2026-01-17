FROM rust:bookworm as builder

RUN rustup toolchain install nightly-2025-04-01 && rustup default nightly-2025-04-01

# Install system dependencies
RUN apt-get update && apt-get install -y pkg-config libssl-dev

# Add WASM target
RUN rustup target add wasm32-unknown-unknown

# Install cargo-binstall for faster tool installation
RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash

# Install cargo-leptos
RUN cargo binstall cargo-leptos -y

# Copy the actual code
COPY . .

# Debug build
RUN cargo build --release --features ssr --no-default-features --bin portfolio

# Build the application
RUN cargo leptos build --release -vv

FROM debian:bookworm-slim as runtime
WORKDIR /app
RUN apt-get update -y \
  && apt-get install -y --no-install-recommends openssl ca-certificates \
  && apt-get autoremove -y \
  && apt-get clean -y \
  && rm -rf /var/lib/apt/lists/*

# Copy the built binary and the site files
COPY --from=builder /target/release/portfolio /app/
COPY --from=builder /target/site /app/site
COPY --from=builder /Cargo.toml /app/
COPY --from=builder /assets /app/assets

# Set the environment to Production
ENV LEPTOS_OUTPUT_NAME="portfolio"
ENV LEPTOS_SITE_ROOT="site"
ENV LEPTOS_SITE_PKG_DIR="pkg"
ENV LEPTOS_SITE_ADDR="0.0.0.0:3000"
ENV LEPTOS_RELOAD_PORT="3001"
ENV LEPTOS_ENV="PROD"

EXPOSE 3000

# Run the application
CMD ["/app/portfolio"]
