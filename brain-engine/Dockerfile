FROM rust:latest AS builder
WORKDIR /app
COPY . .
RUN cargo build -p brain-engine --release && ls -lh /app/target/release/brain*

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/brain-engine /usr/local/bin/brain-engine
EXPOSE 8081
CMD ["brain-engine"]
