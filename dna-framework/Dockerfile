FROM rust:latest as builder
WORKDIR /app
COPY . .
RUN cargo build -p dna-framework --release --verbose 2>&1 | tail -40
RUN echo "=== RELEASE DIR ===" && ls -lh /app/target/release/ && echo "=== BINARIES ===" && file /app/target/release/* | grep executable | head -20
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
# Copy whatever binary cargo made
COPY --from=builder /app/target/release/dna-framework /usr/local/bin/dna-framework
EXPOSE 8080
CMD ["dna-framework"]
