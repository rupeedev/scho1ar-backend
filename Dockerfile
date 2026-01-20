# =============================================================================
# Stage 1: Build the application
# =============================================================================
FROM rust:1.85-bookworm AS builder

WORKDIR /app

# Copy manifests first for dependency caching
COPY Cargo.toml Cargo.lock* ./

# Create dummy src to build dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Pin dependencies requiring newer Rust to compatible versions
RUN cargo update home@0.5.12 --precise 0.5.9

# Build dependencies only (cached layer)
RUN cargo build --release && rm -rf src target/release/deps/scho1ar*

# Copy actual source code
COPY src ./src

# Build the application
RUN cargo build --release

# =============================================================================
# Stage 2: Runtime image (minimal)
# =============================================================================
FROM debian:bookworm-slim AS runtime

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Create non-root user
RUN useradd --create-home --shell /bin/bash appuser
USER appuser
WORKDIR /home/appuser

# Copy binary from builder
COPY --from=builder /app/target/release/scho1ar-backend ./scho1ar-backend

# Environment defaults
ENV HOST=0.0.0.0
ENV PORT=3001
ENV RUST_LOG=info

# Expose API port
EXPOSE 3001

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:3001/health || exit 1

# Run the application
CMD ["./scho1ar-backend"]
