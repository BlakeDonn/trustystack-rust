# Stage 1: Build the application
FROM rust:1.78 AS builder
WORKDIR /app

# Copy the Cargo manifests
COPY Cargo.toml Cargo.lock ./

# Copy the source code and migrations
COPY src/ ./src
COPY migrations/ ./migrations

# Build the application and migrations binaries
RUN cargo build --release --bin rust-backend --bin migrate

# Stage 2: Create the runtime image
FROM debian:stable-slim AS runtime
WORKDIR /app

# Install necessary dependencies
RUN apt-get update && apt-get install -y libpq5 ca-certificates && rm -rf /var/lib/apt/lists/*

# Copy the compiled binaries from the builder stage
COPY --from=builder /app/target/release/rust-backend /usr/local/bin/rust-backend
COPY --from=builder /app/target/release/migrate /usr/local/bin/migrate

# Set the default command to your application
CMD ["/usr/local/bin/rust-backend"]

