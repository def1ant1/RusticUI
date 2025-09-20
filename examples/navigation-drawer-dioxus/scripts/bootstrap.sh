#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="$(git rev-parse --show-toplevel)"
EXAMPLE_ROOT="$REPO_ROOT/target/navigation-drawer-dioxus-demo"

rm -rf "$EXAMPLE_ROOT"
mkdir -p "$EXAMPLE_ROOT"

cat > "$EXAMPLE_ROOT/README.md" <<'MD'
# Dioxus Drawer Blueprint

- Use `drawer::dioxus::render` to obtain the serialized surface/backdrop pair.
- Inject into `rsx!` with `dangerous_inner_html` and wire `onclick` handlers to
  route updates.
- Toggle controlled mode by storing a `Signal<bool>` and calling
  `DrawerState::sync_open` prior to rendering.
MD

printf '\n📄 Dioxus navigation drawer blueprint available in %s\n' "$EXAMPLE_ROOT"
