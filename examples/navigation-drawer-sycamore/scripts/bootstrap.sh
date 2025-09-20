#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="$(git rev-parse --show-toplevel)"
EXAMPLE_ROOT="$REPO_ROOT/target/navigation-drawer-sycamore-demo"

rm -rf "$EXAMPLE_ROOT"
mkdir -p "$EXAMPLE_ROOT"

cat > "$EXAMPLE_ROOT/README.md" <<'MD'
# Sycamore Drawer Blueprint

- Add dependencies identical to the tabs blueprint but replace the markup with
  the drawer renderer:

```rust
let mut layout = DrawerLayoutOptions::default();
layout.anchor.lg = Some(DrawerAnchor::Top);
let theme = Theme::default();
let state = DrawerState::new(
    true,
    unsafe { std::mem::transmute(1u8) },
    DrawerVariant::Modal,
    DrawerAnchor::Start,
);
let render = drawer::sycamore::render(DrawerProps {
    state: &state,
    surface: state.surface_attributes().id("drawer"),
    backdrop: state.backdrop_attributes(),
    body: "<nav>...</nav>",
    layout: &layout,
    theme: &theme,
    viewport: Some(theme.breakpoints.md),
    on_toggle_event: Some("drawer-toggle"),
});
```

- Attach `on:click` handlers to listen for `data-route` attributes and push
  navigation updates.
- Call `trunk serve --open` to preview.
MD

printf '\n📄 Sycamore navigation drawer blueprint available in %s\n' "$EXAMPLE_ROOT"
