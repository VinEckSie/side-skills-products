#Stage 1: build stage
FROM rust:1.86.0-bullseye AS builder

WORKDIR /app

# Copy entire source (Cargo.toml + src/)
COPY ../rust_api_docker .

# Build in release mode
RUN cargo build --release

#Stage 2: run stage
FROM rust:1.86.0-bullseye

WORKDIR /usr/local/bin
COPY --from=builder /app/target/release/rust_api_docker .

RUN chmod +x rust_api_docker

EXPOSE 8080
CMD ["./rust_api_docker"]
