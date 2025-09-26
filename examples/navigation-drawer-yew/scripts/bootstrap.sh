#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="$(git rev-parse --show-toplevel)"
EXAMPLE_ROOT="$REPO_ROOT/target/navigation-drawer-yew-demo"

rm -rf "$EXAMPLE_ROOT"
cargo new "$EXAMPLE_ROOT" --bin --quiet
rm "$EXAMPLE_ROOT/src/main.rs"

cat > "$EXAMPLE_ROOT/Cargo.toml" <<'TOML'
[package]
name = "navigation-drawer-yew-demo"
version = "0.1.0"
edition = "2021"

[dependencies]
yew = { version = "0.21", features = ["csr"] }
yew-router = "0.18"
wasm-bindgen = "0.2"
gloo = { version = "0.9", features = ["events"] }
mui-material = { path = "../../crates/rustic-ui-material", features = ["yew"] }
mui-headless = { path = "../../crates/rustic-ui-headless" }
mui-styled-engine = { path = "../../crates/rustic-ui-styled-engine", features = ["yew"] }
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
    <title>Navigation Drawer – Yew + mui-material</title>
  </head>
  <body>
    <div id="root"></div>
    <script type="module">import init from "./navigation-drawer-yew-demo.js"; init();</script>
  </body>
</html>
HTML

cat > "$EXAMPLE_ROOT/src/main.rs" <<'RS'
use gloo::events::EventListener;
use rustic_ui_headless::drawer::{DrawerAnchor, DrawerState, DrawerVariant};
use rustic_ui_material::drawer::{self, DrawerLayoutOptions, DrawerProps};
use rustic_ui_styled_engine::{Theme, ThemeProvider};
use wasm_bindgen::JsCast;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Routable, Clone, PartialEq, Eq, Debug)]
enum Route {
    #[at("/")]
    Dashboard,
    #[at("/reports")]
    Reports,
    #[at("/settings")]
    Settings,
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <ThemeProvider theme={Theme::default()}>
                <AppShell />
            </ThemeProvider>
        </BrowserRouter>
    }
}

#[function_component(AppShell)]
fn app_shell() -> Html {
    let navigator = use_navigator().expect("router available");
    let route = use_route::<Route>().unwrap_or(Route::Dashboard);
    let open = use_state(|| true);
    let layout = {
        let mut layout = DrawerLayoutOptions::default();
        layout.anchor.lg = Some(DrawerAnchor::Top);
        layout.anchor.xl = Some(DrawerAnchor::Top);
        layout
    };
    let theme = Theme::default();

    let mut state = DrawerState::new(
        *open,
        unsafe { std::mem::transmute(1u8) },
        DrawerVariant::Modal,
        match route {
            Route::Dashboard | Route::Reports | Route::Settings => DrawerAnchor::Start,
        },
    );
    state.sync_open(*open);

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
        viewport: Some(theme.breakpoints.xl),
        on_toggle_event: Some("drawer-toggle"),
    };

    let render = drawer::yew::render(props);

    let container = use_node_ref();
    {
        let container = container.clone();
        let navigator = navigator.clone();
        use_effect_with_deps(
            move |node_ref| {
                if let Some(element) = node_ref.cast::<web_sys::Element>() {
                    let click = EventListener::new(&element, "click", move |event| {
                        if let Some(target) = event
                            .target()
                            .and_then(|t| t.dyn_into::<web_sys::Element>().ok())
                        {
                            if let Some(route) = target.get_attribute("data-route") {
                                match route.as_str() {
                                    "dashboard" => navigator.push(&Route::Dashboard),
                                    "reports" => navigator.push(&Route::Reports),
                                    "settings" => navigator.push(&Route::Settings),
                                    _ => {}
                                }
                            }
                        }
                    });
                    click.forget();
                }
                || ()
            },
            container.clone(),
        );
    }

    let toggle = {
        let open = open.clone();
        Callback::from(move |_| open.set(!*open))
    };

    let DrawerMarkup { surface, backdrop } = DrawerMarkup {
        surface: render.surface,
        backdrop: render.backdrop,
    };

    html! {
        <div class="app-wrapper">
            <button class="drawer-toggle" onclick={toggle}>{"Toggle"}</button>
            <div ref={container}>
                { Html::from_html_unchecked(AttrValue::from(surface)) }
                { backdrop.map(|html| Html::from_html_unchecked(AttrValue::from(html))) }
            </div>
            <section class="content">
                <Switch<Route> render={Switch::render(render_route)} />
            </section>
        </div>
    }
}

fn render_route(route: Route) -> Html {
    match route {
        Route::Dashboard => html! { <h1>{"Dashboard"}</h1> },
        Route::Reports => html! { <h1>{"Reports"}</h1> },
        Route::Settings => html! { <h1>{"Settings"}</h1> },
    }
}

struct DrawerMarkup {
    surface: String,
    backdrop: Option<String>,
}

fn main() {
    yew::Renderer::<App>::new().render();
}
RS

cat > "$EXAMPLE_ROOT/README.md" <<'MD'
# Navigation Drawer – Generated Yew Demo

Generated via `examples/navigation-drawer-yew/scripts/bootstrap.sh`.

```bash
trunk serve --open
```

Features include responsive anchor switching, manual toggle support and routing
integration. The code is saturated with comments to act as a template for large
scale applications.
MD

cat > "$EXAMPLE_ROOT/.gitignore" <<'IGNORE'
/dist
/pkg
wasm-bindgen*.js
wasm-bindgen*.ts
IGNORE

printf '\n✅ Generated Yew navigation drawer demo in %s\n' "$EXAMPLE_ROOT"
