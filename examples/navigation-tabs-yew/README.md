# Navigation Tabs with Yew

This example blueprint shows how to orchestrate responsive navigation tabs in a
[Yew](https://yew.rs/) single page application using the `rustic_ui_headless` state
machine and the HTML renderers exposed by `rustic_ui_material`. The goal is to make it
trivial for enterprise teams to compose routing-aware tab bars that are
accessible, theme aware and easy to automate.

## Highlights

- **Routing integration** – the example wires `yew-router` so each tab reflects
the active route while dispatching navigation when users activate a tab.
- **Responsive orientation** – `TabListLayoutOptions` flips the list into a
vertical rail on large screens while remaining horizontal on mobile.
- **Accessibility first** – markup produced by `rustic_ui_material` keeps
ARIA contracts intact; the demo additionally runs `axe-core` in CI by default.
- **Automation ready** – a bootstrap script generates a runnable project with
Trunk configuration, dependency wiring, and exhaustive inline comments.

## Usage

From the repository root run:

```bash
./examples/navigation-tabs-yew/scripts/bootstrap.sh
cd target/navigation-tabs-yew-demo
trunk serve --open
```

The script materializes a self-contained Yew project under `target/` with a
fully documented `main.rs`, Trunk manifest and HTML entry point.  Developers can
customize the generated code to point at their own routers or telemetry systems
without re-implementing the tab layout or accessibility plumbing.

## Key excerpt

```rust
#[function_component(NavigationTabs)]
fn navigation_tabs() -> Html {
    let navigator = use_navigator().expect("router available");
    let route = use_route::<Route>().unwrap_or(Route::Overview);
    let selected = match route {
        Route::Overview => 0,
        Route::Reports => 1,
        Route::Settings => 2,
    };

    let mut state = TabsState::new(
        3,
        Some(selected),
        ActivationMode::Manual,
        TabsOrientation::Horizontal,
        unsafe { std::mem::transmute(1u8) },
        unsafe { std::mem::transmute(1u8) },
    );
    state.sync_selected(Some(selected));

    let layout = TabListLayoutOptions::default();
    let theme = Theme::default();
    let tabs_markup = vec![
        rustic_ui_material::tab::render_tab_html(&state, state.tab(0).id("tab-overview").controls("panel-overview"), "Overview"),
        rustic_ui_material::tab::render_tab_html(&state, state.tab(1).id("tab-reports").controls("panel-reports"), "Reports"),
        rustic_ui_material::tab::render_tab_html(&state, state.tab(2).id("tab-settings").controls("panel-settings"), "Settings"),
    ].join("");

    let props = TabListProps {
        state: &state,
        attributes: state.list_attributes().id("app-tabs"),
        children: tabs_markup.as_str(),
        layout: &layout,
        theme: &theme,
        viewport: Some(Theme::default().breakpoints.xl),
        on_activate_event: Some("tab-activate"),
    };

    let markup = tabs::yew::render_tab_list(props);
    html! {
        <ThemeProvider theme={theme.clone()}>
            { Html::from_html_unchecked(AttrValue::from(markup)) }
        </ThemeProvider>
    }
}
```

The bootstrap project expands on this snippet with routing hooks, automatic axe
checks and event binding so tab activation triggers navigation updates.
