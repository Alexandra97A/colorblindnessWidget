# Note: Docker is NOT recommended for GUI desktop applications like this widget
# This Dockerfile is provided for educational purposes, but native execution is preferred

FROM rust:1.75 as builder

# Install dependencies
RUN apt-get update && apt-get install -y \
    libfontconfig1-dev \
    libfreetype6-dev \
    libxcb-render0-dev \
    libxcb-shape0-dev \
    libxcb-xfixes0-dev \
    libxkbcommon-dev \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy project files
COPY Cargo.toml Cargo.lock ./
COPY build.rs ./
COPY src ./src
COPY ui ./ui

# Build the application
RUN cargo build --release

# Runtime image
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    libfontconfig1 \
    libfreetype6 \
    libxcb1 \
    libxkbcommon0 \
    libgl1 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy binary from builder
COPY --from=builder /app/target/release/colorblind-widget /app/

# Set display environment variable
ENV DISPLAY=:0

CMD ["/app/colorblind-widget"]
