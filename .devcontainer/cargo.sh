#!/bin/bash

set -xe

CARGO_BUILD_TARGET_DIR=/tmp/target /usr/local/cargo/bin/cargo install cargo-compete
CARGO_BUILD_TARGET_DIR=/tmp/target /usr/local/cargo/bin/cargo install cargo-member

/usr/local/cargo/bin/rustup component add rustfmt
CARGO_BUILD_TARGET_DIR=/tmp/target /usr/local/cargo/bin/cargo install cargo-snippet --features="binaries"

CARGO_BUILD_TARGET_DIR=/tmp/target /usr/local/cargo/bin/cargo compete l atcoder