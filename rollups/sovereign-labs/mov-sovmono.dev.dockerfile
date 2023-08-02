# Use an official Rust runtime as a builder
FROM rust:1.56 as builder

# Set environment variable
ENV RUSTFLAGS="--cfg tokio_unstable"

WORKDIR /app

# Copy all files from the context to the working directory
COPY . .

# Change to the rollup directory
WORKDIR /app/movement-sovereign-labs-rollup

RUN cargo build 

# Start a new stage for a smaller image size
FROM debian:buster-slim

# Copy the binary to the new container
COPY --from=builder /app/movement-sovereign-labs-rollup/target/debug/rollup /rollup

# Command to run the executable
CMD ["./rollup"]
