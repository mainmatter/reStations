FROM rust:1.85 AS builder

WORKDIR /usr/src/restations-builder
COPY ./Cargo.lock .
COPY ./Cargo.toml .
COPY ./cli ./cli
COPY ./config ./config
COPY ./macros ./macros
COPY ./rust-toolchain.toml .
COPY ./web ./web

RUN adduser \
  --disabled-password \
  --gecos "" \
  --home "/nonexistent" \
  --shell "/sbin/nologin" \
  --no-create-home \
  --uid 10001 \
  "restations"

RUN rustup toolchain install
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
ENTRYPOINT ["/usr/local/bin/restations-web"]
EXPOSE 3000
