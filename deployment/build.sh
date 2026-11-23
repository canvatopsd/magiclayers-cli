#!/bin/sh
set -eu
cd "$(dirname "$0")/.."
project=$(git rev-parse --show-toplevel)
export CARGO_TARGET_DIR="$project/.tmp/build/cli"
cargo fmt --check
cargo check --locked
cargo-clippy clippy --locked -- -D warnings
cargo build --release --locked
