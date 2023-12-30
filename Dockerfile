FROM rust:1.74-slim

WORKDIR /app

COPY . .

# Define Healthcheck fot the container
# HEALTHCHECK --interval=5s --timeout=5s --start-period=5s --retries=20 CMD [ "curl","http://0.0.0.0:8080/api/healthcheck" ]

# Upgrade the system and install dependencies for PostgreSQL
RUN apt-get update && \
  apt-get upgrade -y -o DPkg::Options::=--force-confold && \
  apt-get install -y -o DPkg::Options::=--force-confold \
  curl unzip build-essential pkg-config libssl-dev \
  postgresql-client libpq-dev

# Install cargo-watch
RUN cargo install cargo-watch

# Install diesel_cli for PostgreSQL
RUN cargo install diesel_cli --no-default-features --features "postgres"
