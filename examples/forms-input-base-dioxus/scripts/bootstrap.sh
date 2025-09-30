#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="$(git rev-parse --show-toplevel)"

"$REPO_ROOT/examples/scripts/ensure-example-toolchain.sh" "InputBase (Dioxus)" --wasm --ssr

cargo run --bin bootstrap --manifest-path "$REPO_ROOT/examples/forms-input-base-dioxus/Cargo.toml"
