# Stage 1: Base with tools
FROM rust:bookworm AS chef
RUN apt-get update && apt-get install -y pkg-config libssl-dev
RUN rustup toolchain install nightly-2025-04-01 && rustup default nightly-2025-04-01
RUN rustup target add wasm32-unknown-unknown
RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
RUN cargo binstall cargo-chef -y
WORKDIR /app

# Stage 2: Planner (Computes recipe)
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# Stage 3: Builder (Builds dependencies & App)
FROM chef AS builder
RUN cargo binstall cargo-leptos -y

# Copy recipe and build dependencies (Cached Layer)
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json --features ssr --no-default-features

# Build application (Source Code Layer)
COPY . .
RUN cargo leptos build --release -vv

FROM debian:bookworm-slim AS runtime
WORKDIR /app
RUN apt-get update -y \
  && apt-get install -y --no-install-recommends openssl ca-certificates \
  && apt-get autoremove -y \
  && apt-get clean -y \
  && rm -rf /var/lib/apt/lists/*

# Copy the built binary and the site files
COPY --from=builder /app/target/release/portfolio /app/
COPY --from=builder /app/target/site /app/site
COPY --from=builder /app/Cargo.toml /app/
# Static assets (images + scripts relocated off Supabase Storage) served at /assets/*.
# cargo-leptos's assets-dir already syncs these into target/site; this explicit copy
# guarantees they land in the runtime image regardless of that behaviour.
COPY --from=builder /app/public/assets /app/site/assets

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
