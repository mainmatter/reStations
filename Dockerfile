# Heroku doesn't have a great support for rust, their buildpacks are pretty stale and hard to use.
# Instead of using buildpacks, we're using container environment for building and running the application.
# This builds a server binary and copies a start.sh script.

FROM rust:1.84 as builder
WORKDIR /usr/src/restations-builder
COPY . .

RUN cargo build --bin restations-web --release

FROM debian:bookworm-slim as runtime
RUN apt-get update && apt-get install -y ca-certificates openssl && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/restations-builder/target/release/restations-web /usr/local/bin/restations-web
COPY --from=builder /usr/src/restations-builder/scripts/start.sh /usr/local/bin/start.sh

ENTRYPOINT /usr/local/bin/restations-web
