#!/usr/bin/env bash
set -euo pipefail
root=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)
: "${RUSTUP_TOOLCHAIN:?set RUSTUP_TOOLCHAIN to the selected Motor Rust toolchain}"
export RUSTUP_TOOLCHAIN
cargo test --locked --manifest-path "$root/Cargo.toml" -p wasmi_ir --lib
cargo test --locked --manifest-path "$root/Cargo.toml" -p wasmi --test owned_memory
cargo check --locked --manifest-path "$root/Cargo.toml" -p wasmi --no-default-features
