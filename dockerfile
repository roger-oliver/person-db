# Stage 1: Build the application
FROM rust:1.85.1-alpine AS builder
WORKDIR /app

# Install build dependencies and tools
RUN apk add --no-cache \
    gcc \
    musl-dev \
    openssl-dev \
    perl \
    make

# Copy all files needed for building the application
COPY . .

# Build the Rust project
RUN cargo build --release

# Stage 2: Create the final image with only necessary files
FROM alpine:latest
WORKDIR /app

# Copy the built binary from the builder stage
COPY --from=builder /app/target/release/person-db /app

# Set the entrypoint command to run your application
CMD ["./person-db"]