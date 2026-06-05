FROM rust:1.96 AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /app/target/release/bolos-api .
COPY --from=builder /app/migrations ./migrations
EXPOSE 8080
CMD ["./bolos-api"]
