FROM rust:1.56.1

WORKDIR /app
COPY . .

RUN rustc --version
RUN cargo install --path .
RUN cargo install cargo-watch
RUN cargo install diesel_cli
