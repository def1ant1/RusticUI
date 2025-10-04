#!/usr/bin/env bash
# -----------------------------------------------------------------------------
# Centralised selection control automation harness.
#
# This script keeps the selection control smoke tests and dev servers aligned
# across the Rust (Dioxus, Sycamore, Yew) and React implementations.  CI, local
# `just` recipes, and Playwright harnesses all shell out to this file so changes
# to toolchain provisioning or analytics logging only need to land in a single
# location.  The design mirrors enterprise automation pipelines where a small
# number of well-documented entry points orchestrate the full fleet of demos.
#
# The helper wires in three core capabilities:
#   * Toolchain provisioning via `ensure-example-toolchain.sh`.
#   * Consistent analytics + automation logging.
#   * Stable `data-automation-id` selectors published for downstream probes.
#
# Usage examples:
#   ./selection-controls-smoke.sh dioxus --mode smoke
#   ./selection-controls-smoke.sh yew --mode serve --port 4703
#   ./selection-controls-smoke.sh --list-automation --format json
#   ./selection-controls-smoke.sh all --mode smoke
#
# The script intentionally surfaces copious notes so future maintainers can
# understand the rationale behind each branch without spelunking through git
# history.  Enterprise rollouts frequently audit these scripts, so clarity and
# reproducibility take priority over terseness.
# -----------------------------------------------------------------------------
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/../.." && pwd)"
HELPER="${REPO_ROOT}/examples/scripts/ensure-example-toolchain.sh"

# Stable automation identifiers shared across frameworks.  Keep this list in
# sync with the IDs emitted by the individual examples so Playwright and other
# consumers can assert the DOM contract deterministically.
AUTOMATION_IDS=(
    "automation.selection-controls.checkbox"
    "automation.selection-controls.switch"
    "automation.selection-controls.radio"
    "automation.selection-controls.telemetry-log"
)

usage() {
    cat <<'USAGE' >&2
Usage: selection-controls-smoke.sh [--list-automation [--format text|json]]
       selection-controls-smoke.sh <framework|all> [--mode smoke|serve] [--port <port>]

Options:
  --list-automation        Print the canonical automation identifiers and exit.
  --format                 Rendering mode for --list-automation output (text|json).
  --mode                   Execution strategy. "smoke" runs headless validation
                           suites while "serve" launches the web demo for Playwright.
  --port                   Override the server port when --mode serve is used.

Framework aliases:
  dioxus | sycamore | yew | react | all (runs every framework sequentially).

Examples:
  selection-controls-smoke.sh dioxus --mode smoke
  selection-controls-smoke.sh react --mode serve --port 4704
  selection-controls-smoke.sh --list-automation --format json
USAGE
}

print_automation_ids() {
    local format="${1:-text}"
    case "${format}" in
        json)
            printf '['
            local first=1
            for id in "${AUTOMATION_IDS[@]}"; do
                if (( ! first )); then
                    printf ', '
                fi
                printf '"%s"' "${id}"
                first=0
            done
            printf ']\n'
            ;;
        text)
            for id in "${AUTOMATION_IDS[@]}"; do
                printf '%s\n' "${id}"
            done
            ;;
        *)
            echo "unknown automation output format: ${format}" >&2
            exit 2
            ;;
    esac
}

framework=""
mode="smoke"
port=""
list_automation=0
format="text"

while [[ $# -gt 0 ]]; do
    case "$1" in
        --list-automation)
            list_automation=1
            shift
            ;;
        --format)
            if [[ $# -lt 2 ]]; then
                usage
                exit 2
            fi
            format="$2"
            shift 2
            ;;
        --mode)
            if [[ $# -lt 2 ]]; then
                usage
                exit 2
            fi
            mode="$2"
            shift 2
            ;;
        --port)
            if [[ $# -lt 2 ]]; then
                usage
                exit 2
            fi
            port="$2"
            shift 2
            ;;
        -h|--help)
            usage
            exit 0
            ;;
        --*)
            echo "unknown flag: $1" >&2
            usage
            exit 2
            ;;
        *)
            if [[ -z "${framework}" ]]; then
                framework="$1"
            else
                echo "unexpected positional argument: $1" >&2
                usage
                exit 2
            fi
            shift
            ;;
    esac
    if (( list_automation )); then
        break
    fi
done

if (( list_automation )); then
    print_automation_ids "${format}"
    exit 0
fi

if [[ -z "${framework}" ]]; then
    usage
    exit 2
fi

normalize_framework() {
    case "$1" in
        dioxus|Dioxus) echo "dioxus" ;;
        sycamore|Sycamore) echo "sycamore" ;;
        yew|Yew) echo "yew" ;;
        react|React) echo "react" ;;
        all|All) echo "all" ;;
        *)
            echo "unknown framework: $1" >&2
            exit 2
            ;;
    esac
}

framework="$(normalize_framework "${framework}")"

log_banner() {
    local target="$1"
    echo "[selection-controls][${target}] automation ids:" >&2
    for id in "${AUTOMATION_IDS[@]}"; do
        echo "  - ${id}" >&2
    done
}

ensure_toolchain() {
    local label="$1"
    shift
    "${HELPER}" "${label}" "$@"
}

run_dioxus_smoke() {
    echo "[selection-controls][dioxus] provisioning toolchains" >&2
    ensure_toolchain "Selection Controls (Dioxus)" --wasm --ssr
    if ! command -v wasm-pack >/dev/null 2>&1; then
        echo "[selection-controls][dioxus] wasm-pack is required; install via 'cargo install wasm-pack'" >&2
        exit 1
    fi
    local example="${REPO_ROOT}/examples/selection-controls-dioxus"
    echo "[selection-controls][dioxus] executing host + wasm smoke tests" >&2
    pushd "${example}" >/dev/null
    cargo test --all-targets
    wasm-pack test --headless --chrome -- --features web
    popd >/dev/null
}

run_sycamore_smoke() {
    echo "[selection-controls][sycamore] provisioning toolchains" >&2
    ensure_toolchain "Selection Controls (Sycamore)" --wasm --ssr
    local example="${REPO_ROOT}/examples/selection-controls-sycamore"
    pushd "${example}" >/dev/null
    echo "[selection-controls][sycamore] executing cargo test suites" >&2
    cargo test --all-targets
    cargo test --target wasm32-unknown-unknown
    popd >/dev/null
}

run_yew_smoke() {
    echo "[selection-controls][yew] provisioning toolchains" >&2
    ensure_toolchain "Selection Controls (Yew)" --wasm --ssr
    local example="${REPO_ROOT}/examples/selection-controls-yew"
    pushd "${example}" >/dev/null
    echo "[selection-controls][yew] executing cargo host aliases" >&2
    cargo host-test
    echo "[selection-controls][yew] executing cargo wasm aliases" >&2
    cargo wasm-test
    popd >/dev/null
}

run_react_smoke() {
    echo "[selection-controls][react] provisioning toolchains" >&2
    ensure_toolchain "Selection Controls (React)" --wasm
    if ! command -v wasm-pack >/dev/null 2>&1; then
        echo "[selection-controls][react] wasm-pack is required; install via 'cargo install wasm-pack'" >&2
        exit 1
    fi
    if ! command -v npm >/dev/null 2>&1; then
        echo "[selection-controls][react] npm is required to execute the TypeScript harness" >&2
        exit 1
    fi
    local example="${REPO_ROOT}/examples/selection-controls-react"
    if [[ ! -d "${example}/node_modules" ]]; then
        echo "[selection-controls][react] node_modules missing. Run 'npm install' inside ${example}" >&2
        exit 1
    fi
    pushd "${example}" >/dev/null
    echo "[selection-controls][react] running Rust tests" >&2
    cargo test -p selection-controls-react
    echo "[selection-controls][react] running wasm-bindgen tests" >&2
    wasm-pack test --headless --chrome
    echo "[selection-controls][react] running Jest telemetry assertions" >&2
    npm run test:jest -- --runInBand
    popd >/dev/null
    echo "[selection-controls][react] launching Playwright telemetry verification" >&2
    node "${REPO_ROOT}/examples/scripts/selection-controls-playwright.mjs" --framework react
}

serve_dioxus() {
    ensure_toolchain "Selection Controls (Dioxus)" --wasm --ssr
    if ! command -v dx >/dev/null 2>&1; then
        echo "[selection-controls][dioxus] dx CLI is required; install via 'cargo install dioxus-cli'" >&2
        exit 1
    fi
    local example="${REPO_ROOT}/examples/selection-controls-dioxus"
    local listen_port="${1}"
    log_banner "dioxus"
    echo "[selection-controls][dioxus] starting dx serve on port ${listen_port}" >&2
    cd "${example}"
    exec dx serve --config "${example}/dx.json" --address 127.0.0.1 --port "${listen_port}"
}

serve_sycamore() {
    ensure_toolchain "Selection Controls (Sycamore)" --wasm --ssr
    if ! command -v trunk >/dev/null 2>&1; then
        echo "[selection-controls][sycamore] trunk is required; install via 'cargo install trunk'" >&2
        exit 1
    fi
    local example="${REPO_ROOT}/examples/selection-controls-sycamore"
    local listen_port="${1}"
    log_banner "sycamore"
    echo "[selection-controls][sycamore] starting Trunk on port ${listen_port}" >&2
    cd "${example}"
    exec trunk serve --config Trunk.toml --address 127.0.0.1 --port "${listen_port}" --open=false
}

serve_yew() {
    ensure_toolchain "Selection Controls (Yew)" --wasm --ssr
    if ! command -v trunk >/dev/null 2>&1; then
        echo "[selection-controls][yew] trunk is required; install via 'cargo install trunk'" >&2
        exit 1
    fi
    local example="${REPO_ROOT}/examples/selection-controls-yew"
    local listen_port="${1}"
    log_banner "yew"
    echo "[selection-controls][yew] starting Trunk on port ${listen_port}" >&2
    cd "${example}"
    exec trunk serve --config Trunk.toml --address 127.0.0.1 --port "${listen_port}" --open=false
}

serve_react() {
    ensure_toolchain "Selection Controls (React)" --wasm
    if ! command -v npm >/dev/null 2>&1; then
        echo "[selection-controls][react] npm is required to run the dev server" >&2
        exit 1
    fi
    local example="${REPO_ROOT}/examples/selection-controls-react"
    if [[ ! -d "${example}/node_modules" ]]; then
        echo "[selection-controls][react] node_modules missing. Run 'npm install' inside ${example}" >&2
        exit 1
    fi
    local listen_port="${1}"
    log_banner "react"
    echo "[selection-controls][react] starting Vite dev server on port ${listen_port}" >&2
    cd "${example}"
    exec npm run dev -- --host 127.0.0.1 --port "${listen_port}"
}

case "${mode}" in
    smoke)
        case "${framework}" in
            dioxus)
                run_dioxus_smoke
                ;;
            sycamore)
                run_sycamore_smoke
                ;;
            yew)
                run_yew_smoke
                ;;
            react)
                run_react_smoke
                ;;
            all)
                run_dioxus_smoke
                run_sycamore_smoke
                run_yew_smoke
                run_react_smoke
                ;;
            *)
                echo "unsupported framework for smoke mode: ${framework}" >&2
                exit 2
                ;;
        esac
        ;;
    serve)
        case "${framework}" in
            dioxus)
                serve_dioxus "${port:-4701}"
                ;;
            sycamore)
                serve_sycamore "${port:-4702}"
                ;;
            yew)
                serve_yew "${port:-4703}"
                ;;
            react)
                serve_react "${port:-4704}"
                ;;
            all)
                echo "serve mode does not support 'all' concurrently; invoke per framework" >&2
                exit 2
                ;;
            *)
                echo "unsupported framework for serve mode: ${framework}" >&2
                exit 2
                ;;
        esac
        ;;
    *)
        echo "unknown mode: ${mode}" >&2
        usage
        exit 2
        ;;
}
