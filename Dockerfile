FROM rust:1.85 AS builder

WORKDIR /usr/src/restations-builder
COPY ./Cargo.lock .
COPY ./Cargo.toml .
COPY ./cli ./cli
COPY ./config ./config
COPY ./macros ./macros
COPY ./rust-toolchain.toml .
COPY ./web ./web

RUN rustup toolchain install
RUN rustup target add x86_64-unknown-linux-gnu
RUN cargo build --bin restations-web --release

FROM debian:bookworm-slim AS runtime
RUN apt-get update && apt-get install -y ca-certificates openssl && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/restations-builder/target/release/restations-web /usr/local/bin/restations-web
COPY ./stations.sqlite.db .

ENV APP_ENVIRONMENT=production
ENV APP_SERVER__PORT=3000
ENV APP_SERVER__IP="0.0.0.0"
ENTRYPOINT ["/usr/local/bin/restations-web"]
EXPOSE 3000
