#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="$(git rev-parse --show-toplevel)"

"$REPO_ROOT/examples/scripts/ensure-example-toolchain.sh" "Focus trap (Sycamore)" --wasm --ssr

cargo run --bin bootstrap --manifest-path "$REPO_ROOT/examples/utils-trap-focus-sycamore/Cargo.toml"
