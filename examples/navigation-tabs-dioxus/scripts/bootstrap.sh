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
  rustic_ui_material = { path = "../../crates/rustic-ui-material", features = ["dioxus"] }
  rustic_ui_headless = { path = "../../crates/rustic-ui-headless" }
  rustic_ui_styled_engine = { path = "../../crates/rustic-ui-styled-engine", features = ["dioxus"] }
  ```
- Render markup via `tabs::dioxus::render_tab_list` and insert it using
  `rsx!` with `dangerous_inner_html`.
- Hook routing by intercepting click events on the wrapper and pushing new
  routes with `use_navigator` from `dioxus-router`.
MD

printf '\n📄 Dioxus navigation tabs blueprint available in %s\n' "$EXAMPLE_ROOT"
