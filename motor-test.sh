#!/usr/bin/env bash
set -euo pipefail
root=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)
export RUSTUP_TOOLCHAIN=$(sed -n 's/^channel = "\(.*\)"/\1/p' "$root/../motor-os/rust-toolchain.toml")
cargo test --locked --manifest-path "$root/Cargo.toml" -p wasmi_ir --lib
cargo test --locked --manifest-path "$root/Cargo.toml" -p wasmi --test owned_memory
cargo check --locked --manifest-path "$root/Cargo.toml" -p wasmi --no-default-features
