#!/usr/bin/env bash
# Automates the `mui_*` -> `rustic_ui_*` crate rename workflow.
#
# 1. Runs `cargo fix` with the compatibility feature enabled so imports update
#    automatically without manual edits.
# 2. Invokes `cargo xtask clippy` with warnings denied to confirm no legacy
#    aliases remain once the feature is toggled off.
#
# Usage:
#   scripts/migrate-crate-prefix.sh --with-compat
#   scripts/migrate-crate-prefix.sh --verify-clean
#
# Run the script twice:
#   a. `--with-compat` after enabling the `compat-mui` feature flags in
#      `Cargo.toml` (this rewrites modules safely).
#   b. `--verify-clean` after removing the compatibility feature to guarantee
#      the workspace builds without deprecated aliases.
set -euo pipefail

if [[ $# -ne 1 ]]; then
  echo "usage: $0 [--with-compat|--verify-clean]" >&2
  exit 64
fi

case "$1" in
  --with-compat)
    echo "[migrate-crate-prefix] running cargo fix with compatibility shims enabled"
    cargo fix --workspace --allow-dirty --allow-staged
    ;;
  --verify-clean)
    echo "[migrate-crate-prefix] verifying workspace without compatibility shims"
    cargo xtask clippy
    ;;
  *)
    echo "usage: $0 [--with-compat|--verify-clean]" >&2
    exit 64
    ;;
 esac
