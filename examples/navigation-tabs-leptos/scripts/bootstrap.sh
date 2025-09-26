#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="$(git rev-parse --show-toplevel)"
EXAMPLE_ROOT="$REPO_ROOT/target/navigation-tabs-leptos-demo"

rm -rf "$EXAMPLE_ROOT"
cargo new "$EXAMPLE_ROOT" --bin --quiet
rm "$EXAMPLE_ROOT/src/main.rs"

cat > "$EXAMPLE_ROOT/Cargo.toml" <<'TOML'
[package]
name = "navigation-tabs-leptos-demo"
version = "0.1.0"
edition = "2021"

[dependencies]
leptos = { version = "0.6", features = ["csr"] }
leptos_router = { version = "0.6", features = ["csr"] }
wasm-bindgen = "0.2"
mui-material = { path = "../../crates/rustic-ui-material", features = ["leptos"] }
mui-headless = { path = "../../crates/rustic-ui-headless" }
mui-styled-engine = { path = "../../crates/rustic-ui-styled-engine", features = ["leptos"] }
TOML

cat > "$EXAMPLE_ROOT/Trunk.toml" <<'TOML'
[build]
target = "wasm32-unknown-unknown"

[serve]
open = true
TOML

cat > "$EXAMPLE_ROOT/index.html" <<'HTML'
<!doctype html>
<html lang="en">
  <head>
    <meta charset="utf-8" />
    <title>Navigation Tabs – Leptos + mui-material</title>
  </head>
  <body>
    <div id="root"></div>
    <script type="module">import init from "./navigation-tabs-leptos-demo.js"; init();</script>
  </body>
</html>
HTML

cat > "$EXAMPLE_ROOT/src/main.rs" <<'RS'
use leptos::*;
use leptos_router::*;
use rustic_ui_headless::tabs::{ActivationMode, TabsOrientation, TabsState};
use rustic_ui_material::tab;
use rustic_ui_material::tab_panel;
use rustic_ui_material::tabs::{self, TabListLayoutOptions, TabListProps};
use rustic_ui_styled_engine::{Theme, ThemeProvider};
use wasm_bindgen::JsCast;

#[derive(Clone, Routable, PartialEq, Eq, Debug)]
enum Route {
    #[route(path = "/")]
    Overview,
    #[route(path = "/reports")]
    Reports,
    #[route(path = "/settings")]
    Settings,
}

#[component]
fn App() -> impl IntoView {
    view! {
        <Router>
            <ThemeProvider theme={Theme::default()}>
                <main class="app-shell">
                    <NavigationTabs />
                    <main class="content">
                        <Routes>
                            <Route path=Route::Overview view=|| view! { <p>{"Overview"}</p> } />
                            <Route path=Route::Reports view=|| view! { <p>{"Reports"}</p> } />
                            <Route path=Route::Settings view=|| view! { <p>{"Settings"}</p> } />
                        </Routes>
                    </main>
                </main>
            </ThemeProvider>
        </Router>
    }
}

#[component]
fn NavigationTabs() -> impl IntoView {
    let navigate = use_navigate();
    let location = use_location();
    let selected = create_memo(move |_| match location.pathname.get().as_str() {
        "/reports" => 1,
        "/settings" => 2,
        _ => 0,
    });

    let layout = TabListLayoutOptions::default();
    let theme = Theme::default();

    let mut state = TabsState::new(
        3,
        Some(selected()),
        ActivationMode::Manual,
        TabsOrientation::Horizontal,
        unsafe { std::mem::transmute(1u8) },
        unsafe { std::mem::transmute(1u8) },
    );
    state.sync_selected(Some(selected()));

    let tabs_markup = vec![
        tab::render_tab_html(
            &state,
            state
                .tab(0)
                .id("tab-overview")
                .controls("panel-overview"),
            "Overview",
        ),
        tab::render_tab_html(
            &state,
            state
                .tab(1)
                .id("tab-reports")
                .controls("panel-reports"),
            "Reports",
        ),
        tab::render_tab_html(
            &state,
            state
                .tab(2)
                .id("tab-settings")
                .controls("panel-settings"),
            "Settings",
        ),
    ]
    .join("");

    let props = TabListProps {
        state: &state,
        attributes: state.list_attributes().id("dashboard-tabs"),
        children: tabs_markup.as_str(),
        layout: &layout,
        theme: &theme,
        viewport: Some(theme.breakpoints.md),
        on_activate_event: Some("tab-activate"),
    };

    let list_markup = tabs::leptos::render_tab_list(props);
    let panels_markup = format!(
        "{}{}{}",
        tab_panel::render_tab_panel_html(
            &state,
            0,
            state
                .panel(0)
                .id("panel-overview")
                .labelled_by("tab-overview"),
            "<p>Overview</p>",
        ),
        tab_panel::render_tab_panel_html(
            &state,
            1,
            state
                .panel(1)
                .id("panel-reports")
                .labelled_by("tab-reports"),
            "<p>Reports</p>",
        ),
        tab_panel::render_tab_panel_html(
            &state,
            2,
            state
                .panel(2)
                .id("panel-settings")
                .labelled_by("tab-settings"),
            "<p>Settings</p>",
        ),
    );

    let markup = format!("{}{}", list_markup, panels_markup);
    let container = create_node_ref::<leptos::html::Div>();

    create_effect(move |_| {
        if let Some(node) = container.get() {
            node.set_inner_html(&markup);
        }
    });

    let on_click = move |ev: leptos::ev::MouseEvent| {
        if let Some(target) = ev
            .target()
            .and_then(|t| t.dyn_into::<web_sys::Element>().ok())
        {
            if let Some(id) = target.get_attribute("id") {
                match id.as_str() {
                    "tab-overview" => navigate("/", NavigateOptions::default()),
                    "tab-reports" => navigate("/reports", NavigateOptions::default()),
                    "tab-settings" => navigate("/settings", NavigateOptions::default()),
                    _ => {}
                }
            }
        }
    };

    view! {
        <div node_ref=container on:click=on_click></div>
    }
}

fn main() {
    mount_to_body(App);
}
RS

cat > "$EXAMPLE_ROOT/README.md" <<'MD'
# Navigation Tabs – Generated Leptos Demo

Created via `examples/navigation-tabs-leptos/scripts/bootstrap.sh`. Run the demo
with:

```bash
trunk serve --open
```

The scaffold mirrors the Yew variant while highlighting Leptos idioms such as
`create_memo` driven router synchronization.
MD

cat > "$EXAMPLE_ROOT/.gitignore" <<'IGNORE'
/dist
/pkg
wasm-bindgen*.js
wasm-bindgen*.ts
IGNORE

printf '\n✅ Generated Leptos navigation tabs demo in %s\n' "$EXAMPLE_ROOT"
