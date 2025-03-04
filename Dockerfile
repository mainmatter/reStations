FROM rust:1.84 as builder

WORKDIR /usr/src/restations-builder
COPY . .

# Install required packages for the sync-data script
RUN apt-get update && \
    apt-get install -y wget sqlite3 && \
    rm -rf /var/lib/apt/lists/*

# Run the sync-data script to prepare the SQLite database
RUN ./scripts/sync-data

RUN cargo build --bin restations-web --release

# Example: set the server port to 3000 (the default) in the container
ENV APP_SERVER__PORT=3000
EXPOSE 3000

ENTRYPOINT ["./target/release/restations-web"]