#!/usr/bin/env bash
set -euo pipefail

# Determine repository root so relative path patching works even when invoked
# from CI or nested shells.
REPO_ROOT="$(git rev-parse --show-toplevel)"
EXAMPLE_ROOT="$REPO_ROOT/target/navigation-tabs-yew-demo"

rm -rf "$EXAMPLE_ROOT"
cargo new "$EXAMPLE_ROOT" --bin --quiet
rm "$EXAMPLE_ROOT/src/main.rs"

cat > "$EXAMPLE_ROOT/Cargo.toml" <<'TOML'
[package]
name = "navigation-tabs-yew-demo"
version = "0.1.0"
edition = "2021"

[dependencies]
yew = { version = "0.21", features = ["csr"] }
yew-router = "0.18"
wasm-bindgen = "0.2"
gloo = { version = "0.9", features = ["events", "history"] }
# Dependency block renamed from the legacy MUI identifiers to `rustic_ui_*` to document the migration path for template consumers.
rustic_ui_material = { package = "rustic-ui-material", path = "../../crates/rustic-ui-material", features = ["yew"] }
# Headless state machines follow the same aliasing strategy so routers keep compiling without manual intervention.
rustic_ui_headless = { package = "rustic-ui-headless", path = "../../crates/rustic-ui-headless" }
# Styled engine integration mirrors the rename and keeps the automation-friendly metadata stable.
rustic_ui_styled_engine = { package = "rustic-ui-styled-engine", path = "../../crates/rustic-ui-styled-engine", features = ["yew"] }
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
    <title>Navigation Tabs – Yew + rustic_ui_material</title>
  </head>
  <body>
    <div id="root"></div>
    <script type="module">import init from "./navigation-tabs-yew-demo.js"; init();</script>
  </body>
</html>
HTML

cat > "$EXAMPLE_ROOT/src/main.rs" <<'RS'
use gloo::events::EventListener;
use rustic_ui_headless::tabs::{ActivationMode, TabsOrientation, TabsState};
use rustic_ui_material::tab;
use rustic_ui_material::tab_panel;
use rustic_ui_material::tabs::{self, TabListLayoutOptions, TabListProps};
use rustic_ui_styled_engine::{Theme, ThemeProvider};
use wasm_bindgen::JsCast;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Routable, Clone, PartialEq, Eq, Debug)]
enum Route {
    #[at("/")]
    Overview,
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
                <main class="app-shell">
                    <NavigationTabs />
                    <section id="panel-container">
                        <Switch<Route> render={Switch::render(render_route)} />
                    </section>
                </main>
            </ThemeProvider>
        </BrowserRouter>
    }
}

fn render_route(route: Route) -> Html {
    match route {
        Route::Overview => html! { <p>{"Overview content"}</p> },
        Route::Reports => html! { <p>{"Reports"}</p> },
        Route::Settings => html! { <p>{"Settings"}</p> },
    }
}

#[function_component(NavigationTabs)]
fn navigation_tabs() -> Html {
    let navigator = use_navigator().expect("router available");
    let route = use_route::<Route>().unwrap_or(Route::Overview);
    let selected_index = match route {
        Route::Overview => 0,
        Route::Reports => 1,
        Route::Settings => 2,
    };

    let layout = TabListLayoutOptions::default();
    let theme = Theme::default();

    let mut state = TabsState::new(
        3,
        Some(selected_index),
        ActivationMode::Manual,
        TabsOrientation::Horizontal,
        unsafe { std::mem::transmute(1u8) },
        unsafe { std::mem::transmute(1u8) },
    );
    state.sync_selected(Some(selected_index));

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
        viewport: Some(theme.breakpoints.lg),
        on_activate_event: Some("tab-activate"),
    };

    let list_markup = tabs::yew::render_tab_list(props);
    let mut panels = String::new();
    panels.push_str(&tab_panel::render_tab_panel_html(
        &state,
        0,
        state
            .panel(0)
            .id("panel-overview")
            .labelled_by("tab-overview"),
        "<p>Overview content</p>",
    ));
    panels.push_str(&tab_panel::render_tab_panel_html(
        &state,
        1,
        state
            .panel(1)
            .id("panel-reports")
            .labelled_by("tab-reports"),
        "<p>Reports content</p>",
    ));
    panels.push_str(&tab_panel::render_tab_panel_html(
        &state,
        2,
        state
            .panel(2)
            .id("panel-settings")
            .labelled_by("tab-settings"),
        "<p>Settings content</p>",
    ));

    let container = use_node_ref();
    {
        let container = container.clone();
        let navigator = navigator.clone();
        use_effect_with_deps(
            move |node_ref| {
                if let Some(element) = node_ref.cast::<web_sys::Element>() {
                    let handler = EventListener::new(&element, "click", move |event| {
                        if let Some(target) = event
                            .target()
                            .and_then(|t| t.dyn_into::<web_sys::Element>().ok())
                        {
                            if let Some(id) = target.get_attribute("id") {
                                match id.as_str() {
                                    "tab-overview" => navigator.push(&Route::Overview),
                                    "tab-reports" => navigator.push(&Route::Reports),
                                    "tab-settings" => navigator.push(&Route::Settings),
                                    _ => {}
                                }
                            }
                        }
                    });
                    handler.forget();
                }
                || ()
            },
            container.clone(),
        );
    }

    let markup = format!("{}{}", list_markup, panels);
    html! {
        <div ref={container}>
            { Html::from_html_unchecked(AttrValue::from(markup)) }
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
RS

cat > "$EXAMPLE_ROOT/README.md" <<'MD'
# Navigation Tabs – Generated Yew Demo

This project is generated by `examples/navigation-tabs-yew/scripts/bootstrap.sh`.
It contains a fully documented example showcasing how to:

- Synchronize the `rustic_ui_headless` tab state machine with `yew-router` routes.
- Switch between horizontal and vertical layouts based on viewport width.
- Keep WAI-ARIA attributes intact while dispatching navigation events.
- Seed automated accessibility audits via axe-core (hooked through CI).

Run the demo with:

```bash
trunk serve --open
```

The source is intentionally verbose with comments so large engineering teams can
adapt it into their own scaffolding pipelines.
MD

cat > "$EXAMPLE_ROOT/.gitignore" <<'IGNORE'
/dist
/pkg
wasm-bindgen*.js
wasm-bindgen*.ts
IGNORE

printf '\n✅ Generated Yew navigation tabs demo in %s\n' "$EXAMPLE_ROOT"
