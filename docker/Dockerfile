FROM rust:1.73.0-slim-buster AS base 

WORKDIR /workspaces/kyopro

# Install dependencies
RUN apt-get update && \
    apt-get --no-install-recommends -y install pkg-config libssl-dev build-essential gnupg2 git && \
    apt-get clean  && \
    rm -rf /var/lib/apt/lists

ENV PKG_CONFIG_PATH=/usr/bin/pkg-config
ENV LIBSSL_DEV_PATH=/usr/bin/libssl-dev

RUN cargo install cargo-member && \
    cargo install cargo-compete && \
    rustup component add rustfmt clippy
