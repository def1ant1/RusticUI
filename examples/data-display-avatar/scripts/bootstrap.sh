#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="$(git rev-parse --show-toplevel)"

cargo run --bin bootstrap --manifest-path "$REPO_ROOT/examples/data-display-avatar/Cargo.toml" --quiet

echo "Generated avatars under $REPO_ROOT/target/data-display-avatar"
