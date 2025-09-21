#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="$(git rev-parse --show-toplevel)"
EXAMPLE_ROOT="$REPO_ROOT/target/shared-dialog-state-sycamore-demo"

rm -rf "$EXAMPLE_ROOT"
mkdir -p "$EXAMPLE_ROOT/app-shell/src"

cat > "$EXAMPLE_ROOT/Cargo.toml" <<'TOML'
[workspace]
members = ["app-shell"]
resolver = "2"
TOML

cat > "$EXAMPLE_ROOT/Trunk.toml" <<'TOML'
[build]
target = "wasm32-unknown-unknown"

[serve]
open = false
TOML

cat > "$EXAMPLE_ROOT/index.html" <<'HTML'
<!doctype html>
<html lang="en">
  <head>
    <meta charset="utf-8" />
    <title>Shared Dialog State – Sycamore</title>
  </head>
  <body>
    <div id="app"></div>
    <script type="module">import init from "./app-shell.js"; init();</script>
  </body>
</html>
HTML

cat > "$EXAMPLE_ROOT/app-shell/Cargo.toml" <<'TOML'
[package]
name = "app-shell"
version = "0.1.0"
edition = "2021"

[dependencies]
sycamore = { version = "0.9", features = ["web"] }
wasm-bindgen = "0.2"
web-sys = { version = "0.3", features = ["HtmlInputElement", "KeyboardEvent"] }
shared-dialog-state-core = { path = "../../../examples/shared-dialog-state-core" }
TOML

cat > "$EXAMPLE_ROOT/app-shell/src/main.rs" <<'RS'
use shared_dialog_state_core::{
    LifecycleLog, SharedOverlayState, ANCHOR_DIAGRAM, DIALOG_DESCRIPTION_ID,
    DIALOG_SURFACE_ANALYTICS_ID, DIALOG_TITLE_ID, POPOVER_ANCHOR_ID,
    POPOVER_SURFACE_ANALYTICS_ID, TEXT_FIELD_ANALYTICS_ID, TEXT_FIELD_STATUS_ID,
};
use sycamore::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Event, HtmlInputElement, KeyboardEvent};

fn input_value(event: &Event) -> String {
    event
        .target()
        .and_then(|target| target.dyn_into::<HtmlInputElement>().ok())
        .map(|input| input.value())
        .unwrap_or_default()
}

fn push_log(logs: &Signal<Vec<String>>, mut log: LifecycleLog) {
    if log.entries.is_empty() {
        return;
    }
    for entry in &log.entries {
        web_sys::console::log_1(&entry.clone().into());
    }
    logs.modify().append(&mut log.entries);
}

#[component]
fn App<G: Html>(cx: Scope) -> View<G> {
    sycamore::log!("{}", ANCHOR_DIAGRAM);

    let overlay = create_signal(cx, SharedOverlayState::enterprise_defaults());
    let journal = create_signal(cx, Vec::<String>::new());

    let open_dialog = move |_| {
        let (next, log) = overlay.get().clone().request_dialog_open();
        overlay.set(next);
        push_log(&journal, log);
    };

    let close_dialog = move |_| {
        let (next, log) = overlay.get().clone().request_dialog_close();
        overlay.set(next);
        push_log(&journal, log);
    };

    let toggle_popover = move |_| {
        let (next, log) = overlay.get().clone().toggle_popover();
        overlay.set(next);
        push_log(&journal, log);
    };

    let shift_anchor = move |_| {
        let (next, log) = overlay
            .get()
            .clone()
            .update_anchor_geometry(shared_dialog_state_core::AnchorGeometry {
                x: 220.0,
                y: 360.0,
                width: 210.0,
                height: 44.0,
            });
        overlay.set(next);
        push_log(&journal, log);
    };

    let on_input = move |event: Event| {
        let value = input_value(&event);
        let (next, log) = overlay.get().clone().change_text(value);
        overlay.set(next);
        push_log(&journal, log);
    };

    let commit_text = move || {
        let (next, log) = overlay.get().clone().commit_text();
        overlay.set(next);
        push_log(&journal, log);
    };

    let reset_text = move || {
        let (next, log) = overlay.get().clone().reset_text();
        overlay.set(next);
        push_log(&journal, log);
    };

    let on_blur = move |_| commit_text();
    let on_key = move |event: KeyboardEvent| {
        match event.key().as_str() {
            "Enter" => {
                event.prevent_default();
                commit_text();
            }
            "Escape" => {
                event.prevent_default();
                reset_text();
            }
            _ => {}
        }
    };

    view! { cx,
        main(class="automation-shell") {
            header {
                h1 { "Shared dialog state – Sycamore" }
                p { "Signals wrap the shared overlay state so automation hooks remain deterministic." }
            }
            section(class="controls") {
                button(class="primary", on:click=open_dialog) { "Open dialog" }
                button(on:click=close_dialog) { "Close dialog" }
                button(on:click=toggle_popover) { "Toggle popover" }
                button(on:click=shift_anchor) { "Simulate anchor layout shift" }
            }
            section(class="snapshot") {
                h2 { "Snapshot" }
                pre { (format!("{:?}", overlay.get().snapshot())) }
            }
            section(class="popover") {
                let anchor_builder = overlay.get().popover().anchor_attributes();
                let anchor_id = anchor_builder.id().map(|(_, value)| value.to_string()).unwrap_or_else(|| POPOVER_ANCHOR_ID.to_string());
                let placement = anchor_builder.data_placement().1.to_string();
                button(on:click=toggle_popover, id=anchor_id, data-popover-placement=placement) { "Shared anchor" }
            }
            section(class="dialog") {
                if overlay.get().dialog().is_open() {
                    let builder = overlay
                        .get()
                        .dialog()
                        .surface_attributes()
                        .id("shared-dialog-surface")
                        .labelled_by(DIALOG_TITLE_ID)
                        .described_by(DIALOG_DESCRIPTION_ID)
                        .analytics_id(DIALOG_SURFACE_ANALYTICS_ID);
                    let aria_modal = builder.aria_modal().1.to_string();
                    let dialog_id = builder.id_attr().map(|(_, value)| value.to_string()).unwrap_or_default();
                    let labelled = builder.aria_labelledby().map(|(_, value)| value.to_string()).unwrap_or_default();
                    let described = builder.aria_describedby().map(|(_, value)| value.to_string()).unwrap_or_default();
                    let data_state = builder.data_state().1.to_string();
                    let data_focus = builder.data_focus_trap().1.to_string();
                    let data_transition = builder.data_transition().map(|(_, value)| value.to_string()).unwrap_or_default();
                    let analytics = builder.data_analytics_id().map(|(_, value)| value.to_string()).unwrap_or_default();
                    view! { cx,
                        section(class="dialog-surface",
                            role=builder.role(),
                            aria-modal=aria_modal,
                            id=dialog_id,
                            aria-labelledby=labelled,
                            aria-describedby=described,
                            data-state=data_state,
                            data-focus-trap=data_focus,
                            data-transition=data_transition,
                            data-analytics-id=analytics) {
                            header { h2(id=DIALOG_TITLE_ID) { "Automation review" } }
                            p(id=DIALOG_DESCRIPTION_ID) { "Dialogs, popovers, and text fields reuse the shared state across frameworks." }
                            footer { button(on:click=close_dialog) { "Dismiss" } }
                        }
                    }
                } else {
                    View::empty()
                }
            }
            section(class="popover-surface") {
                if overlay.get().popover().is_open() {
                    let builder = overlay
                        .get()
                        .popover()
                        .surface_attributes()
                        .analytics_id(POPOVER_SURFACE_ANALYTICS_ID);
                    let data_open = builder.data_open().1.to_string();
                    let data_preferred = builder.data_preferred().1.to_string();
                    let data_resolved = builder.data_resolved().1.to_string();
                    let analytics = builder.data_analytics_id().map(|(_, value)| value.to_string()).unwrap_or_default();
                    view! { cx,
                        div(class="surface",
                            data-open=data_open,
                            data-preferred-placement=data_preferred,
                            data-resolved-placement=data_resolved,
                            data-analytics-id=analytics) {
                            p { "Popover is open. Collision logic mirrors SSR output." }
                            button(on:click=toggle_popover) { "Dismiss" }
                        }
                    }
                } else {
                    View::empty()
                }
            }
            section(class="text-field") {
                let builder = overlay
                    .get()
                    .text_field()
                    .attributes()
                    .status_id(TEXT_FIELD_STATUS_ID)
                    .analytics_id(TEXT_FIELD_ANALYTICS_ID);
                let aria_invalid = builder.aria_invalid().map(|(_, value)| value.to_string()).unwrap_or_default();
                let described = builder.aria_describedby().map(|(_, value)| value.to_string()).unwrap_or(TEXT_FIELD_STATUS_ID.to_string());
                let data_dirty = builder.data_dirty().1.to_string();
                let data_visited = builder.data_visited().1.to_string();
                let analytics = builder.data_analytics_id().map(|(_, value)| value.to_string()).unwrap_or_default();
                let status = builder
                    .status_message()
                    .unwrap_or_else(|| "Enter a company name with at least three characters.".into());
                label(r#for="shared-text-field") { "Company" }
                input(id="shared-text-field",
                    value=overlay.get().text_field().value().to_string(),
                    placeholder="Enterprise name",
                    on:input=on_input.clone(),
                    on:blur=on_blur.clone(),
                    on:keydown=on_key.clone(),
                    aria-invalid=aria_invalid,
                    aria-describedby=described,
                    data-dirty=data_dirty,
                    data-visited=data_visited,
                    data-analytics-id=analytics)
                p(id=TEXT_FIELD_STATUS_ID, data-role="status") { (status) }
            }
            section(class="anchor-attributes") {
                h2 { "Anchor metadata" }
                let anchor_builder = overlay.get().popover().anchor_attributes();
                let anchor_id = anchor_builder.id().map(|(_, value)| value.to_string()).unwrap_or_else(|| POPOVER_ANCHOR_ID.to_string());
                let placement = anchor_builder.data_placement().1.to_string();
                ul {
                    li { (format!("id={}", anchor_id)) }
                    li { (format!("data-popover-placement={}", placement)) }
                }
            }
            section(class="journal") {
                h2 { "Lifecycle journal" }
                ol {
                    Indexed(iterable=journal, view=move |cx, idx, line| view! { cx, li { (format!("{}", idx.get())) ": " (line.get()) } })
                }
            }
        }
    }
}

fn main() {
    sycamore::render(App);
}

printf '\n✅ Generated Sycamore shared dialog state demo in %s\n' "$EXAMPLE_ROOT"
