#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="$(git rev-parse --show-toplevel)"

"${REPO_ROOT}/examples/scripts/selection-controls-smoke.sh" react --mode smoke
