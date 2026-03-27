# ---- Build Stage ----
FROM rust:1.88 AS builder

# Set working directory inside container
WORKDIR /app

# Copy project source
COPY . .

# Build the actual application
RUN cargo build --release --features postgres

# ---- Runtime Stage ----
FROM debian:bookworm-slim

# Install minimal dependencies (e.g., SSL support if needed)
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy binary from builder
COPY --from=builder /app/target/release/krfx-rust /usr/local/bin/krfx

# Expose API port
EXPOSE 4000

# Run the binary
CMD ["krfx"]
