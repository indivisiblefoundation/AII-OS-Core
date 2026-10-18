FROM rust:latest as builder
WORKDIR /app
COPY . .
RUN cargo build -p mainframe-core --release && ls -lh /app/target/release/ | grep mainframe
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/ /tmp/release/
RUN cp /tmp/release/mainframe-core /usr/local/bin/mainframe-core 2>/dev/null || cp /tmp/release/mainframe_core /usr/local/bin/mainframe-core 2>/dev/null || (ls -l /tmp/release/ && exit 1)
CMD ["mainframe-core"]
