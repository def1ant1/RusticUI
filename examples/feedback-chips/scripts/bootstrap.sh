#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="$(git rev-parse --show-toplevel)"

cargo run --bin bootstrap --manifest-path "$REPO_ROOT/examples/feedback-chips/Cargo.toml" --quiet

echo "Generated chips under $REPO_ROOT/target/feedback-chips"
