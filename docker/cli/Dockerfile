FROM rust:slim-buster

RUN apt update && \
    apt install -y libpq-dev --no-install-recommends libmariadb-dev-compat libsqlite3-dev && \
    cargo install diesel_cli

WORKDIR /app
