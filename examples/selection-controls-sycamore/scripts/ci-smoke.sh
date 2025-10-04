#!/usr/bin/env bash
# One-stop entrypoint for CI runners to validate the Sycamore selection controls.
set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
EXAMPLE_ROOT="$(cd -- "${SCRIPT_DIR}/.." && pwd)"

pushd "${EXAMPLE_ROOT}" >/dev/null
../scripts/ensure-example-toolchain.sh sycamore --wasm --ssr
cargo test --all-targets
cargo test --target wasm32-unknown-unknown
popd >/dev/null
