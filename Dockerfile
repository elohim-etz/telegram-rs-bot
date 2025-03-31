# Stage 1: Build the application
FROM rust:1.70-slim-bullseye as builder

WORKDIR /app

# Install build dependencies
RUN apt-get update && \
    apt-get install -y pkg-config libssl-dev && \
    rm -rf /var/lib/apt/lists/*

# Copy source files
COPY . .

# Build in release mode
RUN cargo build --release

# Stage 2: Create the runtime image
FROM debian:bullseye-slim

WORKDIR /app

# Install runtime dependencies
RUN apt-get update && \
    apt-get install -y libssl1.1 ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# Copy the built binary from the builder stage
COPY --from=builder /app/target/release/telegram-bot-rs /app/telegram-bot-rs
COPY .env /app/

# Set environment variables
ENV RUST_LOG=info
ENV TZ=UTC

# Run the application
CMD ["/app/telegram-bot-rs"]
