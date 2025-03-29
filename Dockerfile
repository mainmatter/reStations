FROM lukemathwalker/cargo-chef:latest-rust-1.85.1-slim AS chef
WORKDIR /usr/src/restations-builder
COPY rust-toolchain.toml rust-toolchain.toml
# Make sure we sync the active toolchain once, if needed.
RUN rustup toolchain install
FROM chef AS planner

COPY . .

RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder

COPY --from=planner /usr/src/restations-builder/recipe.json recipe.json

# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json

COPY . .

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid 10001 \
    "restations"

ENV DATABASE_URL=sqlite:stations.sqlite.db
RUN cargo build --bin restations-web --release

FROM rust:1.85-slim AS runtime

COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group
USER restations:restations

COPY --from=builder --chown=restations:restations /usr/src/restations-builder/target/release/restations-web /usr/local/bin/restations-web
COPY --chown=restations:restations ./stations.sqlite.db .

ENV APP_ENVIRONMENT=production
ENV APP_SERVER__PORT=3000
ENV APP_SERVER__IP="0.0.0.0"
ENV APP_DATABASE__URL="sqlite:stations.sqlite.db"
ENTRYPOINT ["/usr/local/bin/restations-web"]
EXPOSE 3000
