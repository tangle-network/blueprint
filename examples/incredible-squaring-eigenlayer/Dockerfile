# Multi-stage Dockerfile for incredible-squaring-eigenlayer blueprint
# Builds Linux binary from source for container deployment
#
# Build time: ~5-10 minutes (first build with cold cache)

# =============================================================================
# Stage 1: Builder
# =============================================================================
FROM rust:1.86-bookworm AS builder

WORKDIR /build

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    protobuf-compiler \
    clang \
    libclang-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy the entire workspace
COPY . .

# Build the blueprint binary for Linux
RUN cargo build --release --package incredible-squaring-blueprint-eigenlayer

# =============================================================================
# Stage 2: Runtime (minimal image with just the binary)
# =============================================================================
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    procps \
    && rm -rf /var/lib/apt/lists/*

# Create a non-root user
RUN useradd -m -u 1000 blueprint && \
    mkdir -p /mnt/keystore /mnt/data && \
    chown -R blueprint:blueprint /mnt/keystore /mnt/data

# Copy the compiled binary from builder
COPY --from=builder /build/target/release/incredible-squaring-blueprint-eigenlayer /usr/local/bin/blueprint

# Make it executable
RUN chmod +x /usr/local/bin/blueprint

# Switch to non-root user
USER blueprint

# Set working directory
WORKDIR /home/blueprint

# Health check (checks if the process is responding)
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD pgrep -f blueprint || exit 1

# Default command (can be overridden)
ENTRYPOINT ["/usr/local/bin/blueprint"]
CMD ["run"]
