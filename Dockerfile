# Stage 1: Prepare the build environment using Cargo Chef
FROM lukemathwalker/cargo-chef:latest AS chef
WORKDIR /app

# Stage 2: Plan the build
FROM chef AS planner
COPY ./Cargo.toml ./Cargo.lock ./
COPY ./src ./src
RUN cargo chef prepare --recipe-path recipe.json

# Stage 3: Build dependencies and application
FROM chef AS builder

# Install diesel_cli
RUN cargo install diesel_cli --no-default-features --features postgres

# Copy over the build plan and build the dependencies
COPY --from=planner /app/recipe.json .
RUN cargo chef cook --release --recipe-path recipe.json

# Copy the rest of the application code, including migrations
COPY . .

# Build the application
RUN cargo build --release

# Move the built binary to a known location
RUN mv ./target/release/rust-backend ./app

# Stage 4: Create the runtime image
FROM debian:stable-slim AS runtime
WORKDIR /app

# Install necessary PostgreSQL client libraries
RUN apt-get update && apt-get install -y libpq-dev && rm -rf /var/lib/apt/lists/*

# Copy the compiled binary and diesel_cli from the builder stage
COPY --from=builder /app/app /usr/local/bin/
COPY --from=builder /app/migrations /app/migrations
COPY --from=builder /usr/local/cargo/bin/diesel /usr/local/bin/diesel

# Set the entry point to your application
ENTRYPOINT ["/usr/local/bin/app"]

