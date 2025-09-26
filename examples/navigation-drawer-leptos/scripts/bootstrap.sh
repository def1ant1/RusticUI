#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="$(git rev-parse --show-toplevel)"
EXAMPLE_ROOT="$REPO_ROOT/target/navigation-drawer-leptos-demo"

rm -rf "$EXAMPLE_ROOT"
cargo new "$EXAMPLE_ROOT" --bin --quiet
rm "$EXAMPLE_ROOT/src/main.rs"

cat > "$EXAMPLE_ROOT/Cargo.toml" <<'TOML'
[package]
name = "navigation-drawer-leptos-demo"
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
    <title>Navigation Drawer – Leptos + mui-material</title>
  </head>
  <body>
    <div id="root"></div>
    <script type="module">import init from "./navigation-drawer-leptos-demo.js"; init();</script>
  </body>
</html>
HTML

cat > "$EXAMPLE_ROOT/src/main.rs" <<'RS'
use leptos::*;
use leptos_router::*;
use mui_headless::drawer::{DrawerAnchor, DrawerState, DrawerVariant};
use mui_material::drawer::{self, DrawerLayoutOptions, DrawerProps};
use mui_styled_engine::{Theme, ThemeProvider};
use wasm_bindgen::JsCast;

#[derive(Clone, Routable, PartialEq, Eq, Debug)]
enum Route {
    #[route(path = "/")]
    Dashboard,
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
                <AppShell />
            </ThemeProvider>
        </Router>
    }
}

#[component]
fn AppShell() -> impl IntoView {
    let navigate = use_navigate();
    let location = use_location();
    let open = create_rw_signal(true);
    let layout = {
        let mut layout = DrawerLayoutOptions::default();
        layout.anchor.lg = Some(DrawerAnchor::Top);
        layout.anchor.xl = Some(DrawerAnchor::Top);
        layout
    };
    let theme = Theme::default();

    let markup = create_memo(move |_| {
        let route = location.pathname.get();
        let anchor = if route == "/" || route == "/reports" || route == "/settings" {
            DrawerAnchor::Start
        } else {
            DrawerAnchor::Start
        };
        let mut state = DrawerState::new(
            open.get(),
            unsafe { std::mem::transmute(1u8) },
            DrawerVariant::Modal,
            anchor,
        );
        state.sync_open(open.get());

        let props = DrawerProps {
            state: &state,
            surface: state
                .surface_attributes()
                .id("app-drawer")
                .labelled_by("drawer-heading"),
            backdrop: state.backdrop_attributes(),
            body: "<header id=\"drawer-heading\">Navigation</header><nav role=\"navigation\"><ul><li><a href=\"/\" data-route=\"dashboard\">Dashboard</a></li><li><a href=\"/reports\" data-route=\"reports\">Reports</a></li><li><a href=\"/settings\" data-route=\"settings\">Settings</a></li></ul></nav>",
            layout: &layout,
            theme: &theme,
            viewport: Some(theme.breakpoints.lg),
            on_toggle_event: Some("drawer-toggle"),
        };
        drawer::leptos::render(props)
    });

    let container = create_node_ref::<leptos::html::Div>();
    create_effect(move |_| {
        if let Some(node) = container.get() {
            let render = markup.get();
            node.set_inner_html(&render.surface);
            if let Some(backdrop) = render.backdrop {
                let document = web_sys::window().unwrap().document().unwrap();
                let element = document.create_element("div").unwrap();
                element.set_inner_html(&backdrop);
                if let Some(child) = element.first_element_child() {
                    node.append_child(&child).unwrap();
                }
            }
        }
    });

    let on_click = move |ev: leptos::ev::MouseEvent| {
        if let Some(target) = ev
            .target()
            .and_then(|t| t.dyn_into::<web_sys::Element>().ok())
        {
            if let Some(route) = target.get_attribute("data-route") {
                match route.as_str() {
                    "dashboard" => navigate("/", NavigateOptions::default()),
                    "reports" => navigate("/reports", NavigateOptions::default()),
                    "settings" => navigate("/settings", NavigateOptions::default()),
                    _ => {}
                }
            }
        }
    };

    let toggle = move |_| {
        open.update(|state| *state = !*state);
    };

    view! {
        <section class="app-wrapper">
            <button on:click=toggle>{"Toggle"}</button>
            <div node_ref=container on:click=on_click></div>
            <main class="content">
                <Routes>
                    <Route path=Route::Dashboard view=|| view! { <h1>{"Dashboard"}</h1> } />
                    <Route path=Route::Reports view=|| view! { <h1>{"Reports"}</h1> } />
                    <Route path=Route::Settings view=|| view! { <h1>{"Settings"}</h1> } />
                </Routes>
            </main>
        </section>
    }
}

fn main() {
    mount_to_body(App);
}
RS

cat > "$EXAMPLE_ROOT/README.md" <<'MD'
# Navigation Drawer – Generated Leptos Demo

Scaffold generated via `examples/navigation-drawer-leptos/scripts/bootstrap.sh`.
Run `trunk serve --open` from this directory to launch the sample.
MD

cat > "$EXAMPLE_ROOT/.gitignore" <<'IGNORE'
/dist
/pkg
wasm-bindgen*.js
wasm-bindgen*.ts
IGNORE

printf '\n✅ Generated Leptos navigation drawer demo in %s\n' "$EXAMPLE_ROOT"
