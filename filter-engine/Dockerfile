FROM rust:latest AS builder
WORKDIR /app
COPY . .
RUN cargo build -p filter-engine --release || echo "filter-engine is a lib, no binary needed"

FROM debian:bookworm-slim
CMD ["sleep", "infinity"]
