#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")"/.. && pwd)"
cd "$ROOT"

echo "[ci-smoke] running host checks"
cargo host-check

echo "[ci-smoke] running wasm checks"
cargo wasm-check

echo "[ci-smoke] executing host tests"
cargo host-test

echo "[ci-smoke] executing wasm tests"
cargo wasm-test
