#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="$(git rev-parse --show-toplevel)"

"$REPO_ROOT/examples/scripts/ensure-example-toolchain.sh" "Focus trap (Dioxus)" --wasm --ssr

cargo run --bin bootstrap --manifest-path "$REPO_ROOT/examples/utils-trap-focus-dioxus/Cargo.toml"
