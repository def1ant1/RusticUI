# Navigation Tabs with Leptos

This guide documents how to integrate `mui-material`'s tab adapters into a
[Leptos](https://leptos.dev/) SPA while keeping routing, responsive breakpoints
and accessibility concerns centralized. The pattern mirrors the Yew example but
leans on Leptos' declarative `view!` syntax.

## Highlights

- **Router-aware** – leverages `leptos_router` to map URLs onto selected tabs.
- **Responsive** – uses `TabListLayoutOptions` to pivot into a vertical
navigation rail at desktop breakpoints.
- **Accessible by default** – the generated markup carries the WAI-ARIA
contracts emitted by `mui-headless`.
- **Automatable** – an accompanying bootstrap script provisions a runnable demo
with comments explaining every integration point.

## Usage

```bash
./examples/navigation-tabs-leptos/scripts/bootstrap.sh
cat target/navigation-tabs-leptos-demo/README.md
```

The automation script drops a ready-to-customize skeleton (Cargo manifest,
`Trunk.toml`, HTML entry point and heavily documented `main.rs`) into
`target/navigation-tabs-leptos-demo`. Running `trunk serve --open` from that
directory spins up the demo immediately.

## Key excerpt

```rust
#[component]
fn NavigationTabs() -> impl IntoView {
    let params = use_params_map();
    let navigator = use_navigate();
    let selected = match params.with(|p| p.get("section").cloned()).as_deref() {
        Some("reports") => 1,
        Some("settings") => 2,
        _ => 0,
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
        viewport: Some(theme.breakpoints.md),
        on_activate_event: Some("tab-activate"),
    };

    let markup = tabs::leptos::render_tab_list(props);
    view! { <div inner_html=markup on:click=move |ev| handle_click(ev, navigator.clone())/> }
}
```

The generated project expands this snippet with concrete `handle_click` logic,
route definitions, panel rendering, and accessibility audit helpers.
