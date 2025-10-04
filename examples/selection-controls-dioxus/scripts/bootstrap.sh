#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="$(git rev-parse --show-toplevel)"
EXAMPLE_ROOT="$REPO_ROOT/examples/selection-controls-dioxus"

"$REPO_ROOT/examples/scripts/ensure-example-toolchain.sh" "Selection Controls (Dioxus)" --wasm --ssr

pushd "$EXAMPLE_ROOT" >/dev/null
cargo check --all-targets
wasm-pack test --headless --chrome -- --features web
popd >/dev/null

echo "[selection-controls-dioxus] host and wasm pipelines validated"
