#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="$(git rev-parse --show-toplevel)"
EXAMPLE_ROOT="$REPO_ROOT/target/navigation-tabs-dioxus-demo"

rm -rf "$EXAMPLE_ROOT"
mkdir -p "$EXAMPLE_ROOT"

cat > "$EXAMPLE_ROOT/README.md" <<'MD'
# Dioxus Navigation Tabs Blueprint

- Dependencies:
  ```toml
  dioxus = { version = "0.5", features = ["web"] }
  dioxus-router = "0.5"
  wasm-bindgen = "0.2"
  mui-material = { path = "../../crates/mui-material", features = ["dioxus"] }
  mui-headless = { path = "../../crates/mui-headless" }
  mui-styled-engine = { path = "../../crates/mui-styled-engine", features = ["dioxus"] }
  ```
- Render markup via `tabs::dioxus::render_tab_list` and insert it using
  `rsx!` with `dangerous_inner_html`.
- Hook routing by intercepting click events on the wrapper and pushing new
  routes with `use_navigator` from `dioxus-router`.
MD

printf '\n📄 Dioxus navigation tabs blueprint available in %s\n' "$EXAMPLE_ROOT"
