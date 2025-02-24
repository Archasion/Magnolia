FROM rust:1.84.1-bookworm as builder

WORKDIR /usr/src/discord-bot-rs

COPY ./bot/src/ ./bot/src/
COPY ./bot/Cargo.toml ./bot/Cargo.toml
COPY ./builders/src/ ./builders/src/
COPY ./builders/Cargo.toml ./builders/Cargo.toml
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN cargo install --path ./bot

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y libssl3 ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/bot /usr/local/bin/bot

CMD ["bot"]