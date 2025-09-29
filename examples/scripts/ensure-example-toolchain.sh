#!/usr/bin/env bash
# -----------------------------------------------------------------------------
# Shared automation entry point consumed by example Justfiles.
#
# The goal is to keep repetitive setup steps (ensuring `wasm32-unknown-unknown`
# is installed, verifying that `trunk` is on PATH, etc.) in one centrally
# maintained location.  This mirrors the curated developer experience provided
# by the upstream JavaScript monorepo where `yarn` scripts abstract framework
# quirks away from new contributors.
#
# Usage:
#   ./ensure-example-toolchain.sh <framework> [--wasm] [--ssr]
#
# Arguments:
#   <framework>  Human friendly label used for log output so CI operators can
#                quickly identify which adapter failed to bootstrap.
#   --wasm       Require WebAssembly tooling. The script will auto-install the
#                `wasm32-unknown-unknown` target if it is missing and ensure the
#                `trunk` bundler is available.
#   --ssr        Require native server side rendering capabilities. We validate
#                that `cargo` can compile host binaries so CI can fail fast when
#                cross compilation toolchains go missing.
#
# The script intentionally uses `set -euo pipefail` to fail on the first error.
# Enterprise CI systems rely on explicit failure signals to short circuit
# expensive matrix jobs when prerequisites disappear.
# -----------------------------------------------------------------------------
set -euo pipefail

if [[ $# -lt 1 ]]; then
    echo "usage: $0 <framework> [--wasm] [--ssr]" >&2
    exit 2
fi

framework="$1"
shift

need_wasm=0
need_ssr=0

while [[ $# -gt 0 ]]; do
    case "$1" in
        --wasm)
            need_wasm=1
            ;;
        --ssr)
            need_ssr=1
            ;;
        *)
            echo "Unknown flag '$1'" >&2
            exit 2
            ;;
    esac
    shift
done

if ! command -v cargo >/dev/null 2>&1; then
    echo "[toolchain] cargo is required to build the ${framework} examples." >&2
    exit 1
fi

if ! command -v rustup >/dev/null 2>&1; then
    echo "[toolchain] rustup is required so we can install additional targets on demand." >&2
    exit 1
fi

if (( need_wasm )); then
    echo "[toolchain] ensuring wasm32 target for ${framework} workflows"
    if ! rustup target list --installed | grep -q "wasm32-unknown-unknown"; then
        rustup target add wasm32-unknown-unknown
    fi

    if ! command -v trunk >/dev/null 2>&1; then
        echo "[toolchain] trunk is required for ${framework} WebAssembly bundles. Install it via 'cargo install trunk'." >&2
        exit 1
    fi
fi

if (( need_ssr )); then
    echo "[toolchain] validating native compilation pipeline for ${framework} SSR"
    # We rely on `cargo metadata` as a lightweight smoke test that the host
    # toolchain is functional.  It exercises dependency resolution without
    # forcing a full build.
    cargo metadata >/dev/null
fi

