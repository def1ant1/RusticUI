#!/usr/bin/env bash
# One-stop entrypoint for CI runners to validate the Sycamore selection controls.
set -euo pipefail

REPO_ROOT="$(git rev-parse --show-toplevel)"

"${REPO_ROOT}/examples/scripts/selection-controls-smoke.sh" sycamore --mode smoke
