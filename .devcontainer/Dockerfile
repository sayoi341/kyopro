FROM mcr.microsoft.com/devcontainers/base:ubuntu

RUN apt-get update && \
    apt-get upgrade -y && \
    apt-get -y install pkg-config build-essential gnupg2

ENV CARGO_BUILD_TARGET_DIR=/tmp/target