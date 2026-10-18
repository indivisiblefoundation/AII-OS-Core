FROM rust:latest as builder
WORKDIR /app
COPY . .
RUN cargo build -p guerillanet-p2p --release && ls -lh /app/target/release/ | grep -E "guerilla|p2p"
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/guerillanet-p2p /usr/local/bin/guerillanet-p2p
COPY --from=builder /app/target/release/ /tmp/release/
RUN cp /tmp/release/guerillanet-p2p /usr/local/bin/guerillanet-p2p 2>/dev/null || cp /tmp/release/guerillanet_p2p /usr/local/bin/guerillanet-p2p 2>/dev/null || (ls -l /tmp/release/ && exit 1)
CMD ["guerillanet-p2p"]
