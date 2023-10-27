#!/bin/bash

set -xe

/usr/local/cargo/bin/cargo install cargo-compete
/usr/local/cargo/bin/cargo install cargo-member
/usr/local/cargo/bin/cargo compete i atcoder 2

/usr/local/cargo/bin/rustup component add rustfmt
/usr/local/cargo/bin/cargo install cargo-snippet --features="binaries"

echo login to atcoder
# cargo compete l atcoder
