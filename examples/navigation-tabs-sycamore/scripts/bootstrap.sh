#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="$(git rev-parse --show-toplevel)"
EXAMPLE_ROOT="$REPO_ROOT/target/navigation-tabs-sycamore-demo"

rm -rf "$EXAMPLE_ROOT"
mkdir -p "$EXAMPLE_ROOT"

cat > "$EXAMPLE_ROOT/README.md" <<'MD'
# Sycamore Navigation Tabs Blueprint

1. `cargo new sycamore-tabs --bin` and add the following dependencies:
   ```toml
   sycamore = { version = "0.9", features = ["web"] }
   sycamore-router = "0.9"
   wasm-bindgen = "0.2"
   mui-material = { path = "../../crates/mui-material", features = ["sycamore"] }
   mui-headless = { path = "../../crates/mui-headless" }
   mui-styled-engine = { path = "../../crates/mui-styled-engine", features = ["sycamore"] }
   ```
2. Use the snippet below inside `main.rs` to render responsive tabs while
   dispatching router navigation events:

```rust
let mut layout = TabListLayoutOptions::default();
let theme = Theme::default();
let state = TabsState::new(
    3,
    Some(0),
    ActivationMode::Manual,
    TabsOrientation::Horizontal,
    unsafe { std::mem::transmute(1u8) },
    unsafe { std::mem::transmute(1u8) },
);
let markup = tabs::sycamore::render_tab_list(TabListProps {
    state: &state,
    attributes: state.list_attributes().id("tabs"),
    children: "<button role=\"tab\" id=\"tab-home\" aria-controls=\"panel-home\">Home</button>...",
    layout: &layout,
    theme: &theme,
    viewport: Some(theme.breakpoints.md),
    on_activate_event: Some("tab-activate"),
});
```

3. Wire click handlers with `sycamore::futures::spawn_local` to push routes when
   a tab button is activated.
4. Execute `trunk serve --open` to run the demo.
MD

printf '\n📄 Sycamore navigation tabs blueprint available in %s\n' "$EXAMPLE_ROOT"
