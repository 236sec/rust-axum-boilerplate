# Stage 1: Build the application
FROM rust:1.95 AS builder

WORKDIR /usr/src/app

# Copy the source code
COPY . .

# Enforce SQLx offline mode so it reads from the json file
ENV SQLX_OFFLINE=true

# Build the application in release mode
RUN cargo build --release

# Stage 2: Run the application
FROM debian:bookworm-slim

WORKDIR /app

# Copy the compiled binary from the builder stage
# NOTE: Replace "my_rust_app" with the actual binary name from your Cargo.toml
COPY --from=builder /usr/src/app/target/release/rust-axum-boilerplate .

# Set the startup command
CMD ["./rust-axum-boilerplate"]