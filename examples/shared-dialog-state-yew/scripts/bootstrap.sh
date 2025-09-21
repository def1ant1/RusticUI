#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="$(git rev-parse --show-toplevel)"
EXAMPLE_ROOT="$REPO_ROOT/target/shared-dialog-state-yew-demo"

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
release = false

[serve]
open = false
TOML

cat > "$EXAMPLE_ROOT/index.html" <<'HTML'
<!doctype html>
<html lang="en">
  <head>
    <meta charset="utf-8" />
    <title>Shared Dialog State – Yew</title>
  </head>
  <body>
    <div id="root"></div>
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
yew = { version = "0.21", features = ["csr"] }
wasm-bindgen = "0.2"
web-sys = { version = "0.3", features = ["HtmlInputElement", "KeyboardEvent"] }
gloo = { version = "0.9", features = ["console"] }
shared-dialog-state-core = { path = "../../../examples/shared-dialog-state-core" }
TOML

cat > "$EXAMPLE_ROOT/app-shell/src/main.rs" <<'RS'
use gloo::console::log;
use shared_dialog_state_core::{
    LifecycleLog, SharedOverlayState, ANCHOR_DIAGRAM, DIALOG_DESCRIPTION_ID,
    DIALOG_SURFACE_ANALYTICS_ID, DIALOG_TITLE_ID, POPOVER_ANCHOR_ID,
    POPOVER_SURFACE_ANALYTICS_ID, TEXT_FIELD_ANALYTICS_ID, TEXT_FIELD_STATUS_ID,
};
use wasm_bindgen::JsCast;
use web_sys::{HtmlInputElement, KeyboardEvent};
use yew::prelude::*;

fn push_log(handle: &UseStateHandle<Vec<String>>, mut log: LifecycleLog) {
    if log.entries.is_empty() {
        return;
    }
    for entry in &log.entries {
        log!(entry.clone());
    }
    let mut next = (**handle).clone();
    next.append(&mut log.entries);
    handle.set(next);
}

fn dialog_surface_attributes(state: &SharedOverlayState) -> Vec<(String, String)> {
    let builder = state
        .dialog()
        .surface_attributes()
        .id("shared-dialog-surface")
        .labelled_by(DIALOG_TITLE_ID)
        .described_by(DIALOG_DESCRIPTION_ID)
        .analytics_id(DIALOG_SURFACE_ANALYTICS_ID);
    let mut attrs = Vec::new();
    attrs.push(("role".into(), builder.role().into()));
    let (aria_modal_key, aria_modal_value) = builder.aria_modal();
    attrs.push((aria_modal_key.into(), aria_modal_value.into()));
    if let Some((key, value)) = builder.id_attr() {
        attrs.push((key.into(), value.into()));
    }
    if let Some((key, value)) = builder.aria_labelledby() {
        attrs.push((key.into(), value.into()));
    }
    if let Some((key, value)) = builder.aria_describedby() {
        attrs.push((key.into(), value.into()));
    }
    let (state_key, state_value) = builder.data_state();
    attrs.push((state_key.into(), state_value.into()));
    if let Some((transition_key, transition_value)) = builder.data_transition() {
        attrs.push((transition_key.into(), transition_value.into()));
    }
    let (trap_key, trap_value) = builder.data_focus_trap();
    attrs.push((trap_key.into(), trap_value.into()));
    if let Some((analytics_key, analytics_value)) = builder.data_analytics_id() {
        attrs.push((analytics_key.into(), analytics_value.into()));
    }
    attrs
}

fn popover_anchor_attributes(state: &SharedOverlayState) -> Vec<(String, String)> {
    let builder = state.popover().anchor_attributes();
    let mut attrs = Vec::new();
    if let Some((key, value)) = builder.id() {
        attrs.push((key.into(), value.into()));
    }
    let (placement_key, placement_value) = builder.data_placement();
    attrs.push((placement_key.into(), placement_value.into()));
    attrs
}

fn popover_surface_attributes(state: &SharedOverlayState) -> Vec<(String, String)> {
    let builder = state
        .popover()
        .surface_attributes()
        .analytics_id(POPOVER_SURFACE_ANALYTICS_ID);
    let mut attrs = Vec::new();
    let (open_key, open_value) = builder.data_open();
    attrs.push((open_key.into(), open_value.into()));
    let (preferred_key, preferred_value) = builder.data_preferred();
    attrs.push((preferred_key.into(), preferred_value.into()));
    let (resolved_key, resolved_value) = builder.data_resolved();
    attrs.push((resolved_key.into(), resolved_value.into()));
    if let Some((analytics_key, analytics_value)) = builder.data_analytics_id() {
        attrs.push((analytics_key.into(), analytics_value.into()));
    }
    attrs
}

fn text_field_attributes(
    state: &SharedOverlayState,
) -> (Vec<(String, String)>, Option<String>) {
    let builder = state
        .text_field()
        .attributes()
        .status_id(TEXT_FIELD_STATUS_ID)
        .analytics_id(TEXT_FIELD_ANALYTICS_ID);
    let mut attrs = Vec::new();
    if let Some((key, value)) = builder.aria_invalid() {
        attrs.push((key.into(), value.into()));
    }
    if let Some((key, value)) = builder.aria_describedby() {
        attrs.push((key.into(), value.into()));
    }
    let (dirty_key, dirty_value) = builder.data_dirty();
    attrs.push((dirty_key.into(), dirty_value.into()));
    let (visited_key, visited_value) = builder.data_visited();
    attrs.push((visited_key.into(), visited_value.into()));
    if let Some((key, value)) = builder.data_analytics_id() {
        attrs.push((key.into(), value.into()));
    }
    (attrs, builder.status_message())
}

#[function_component(App)]
fn app() -> Html {
    use_effect_once(|| {
        log!(ANCHOR_DIAGRAM);
        || ()
    });

    let overlay = use_state(SharedOverlayState::enterprise_defaults);
    let journal = use_state(Vec::<String>::new);

    let open_dialog = {
        let overlay = overlay.clone();
        let journal = journal.clone();
        Callback::from(move |_| {
            let (next, log) = (*overlay).clone().request_dialog_open();
            overlay.set(next);
            push_log(&journal, log);
        })
    };

    let close_dialog = {
        let overlay = overlay.clone();
        let journal = journal.clone();
        Callback::from(move |_| {
            let (next, log) = (*overlay).clone().request_dialog_close();
            overlay.set(next);
            push_log(&journal, log);
        })
    };

    let toggle_popover = {
        let overlay = overlay.clone();
        let journal = journal.clone();
        Callback::from(move |_| {
            let (next, log) = (*overlay).clone().toggle_popover();
            overlay.set(next);
            push_log(&journal, log);
        })
    };

    let shift_anchor = {
        let overlay = overlay.clone();
        let journal = journal.clone();
        Callback::from(move |_| {
            let (next, log) = (*overlay)
                .clone()
                .update_anchor_geometry(shared_dialog_state_core::AnchorGeometry {
                    x: 240.0,
                    y: 360.0,
                    width: 220.0,
                    height: 40.0,
                });
            overlay.set(next);
            push_log(&journal, log);
        })
    };

    let on_input = {
        let overlay = overlay.clone();
        let journal = journal.clone();
        Callback::from(move |event: InputEvent| {
            let value = event
                .target()
                .and_then(|target| target.dyn_into::<HtmlInputElement>().ok())
                .map(|input| input.value())
                .unwrap_or_default();
            let (next, log) = (*overlay).clone().change_text(value);
            overlay.set(next);
            push_log(&journal, log);
        })
    };

    let on_blur = {
        let overlay = overlay.clone();
        let journal = journal.clone();
        Callback::from(move |_| {
            let (next, log) = (*overlay).clone().commit_text();
            overlay.set(next);
            push_log(&journal, log);
        })
    };

    let on_keydown = {
        let overlay = overlay.clone();
        let journal = journal.clone();
        Callback::from(move |event: KeyboardEvent| {
            match event.key().as_str() {
                "Enter" => {
                    event.prevent_default();
                    let (next, log) = (*overlay).clone().commit_text();
                    overlay.set(next);
                    push_log(&journal, log);
                }
                "Escape" => {
                    event.prevent_default();
                    let (next, log) = (*overlay).clone().reset_text();
                    overlay.set(next);
                    push_log(&journal, log);
                }
                _ => {}
            }
        })
    };

    let snapshot = overlay.snapshot();
    let (text_attrs, status_message) = text_field_attributes(&overlay);
    let dialog_attrs = dialog_surface_attributes(&overlay);
    let anchor_attrs = popover_anchor_attributes(&overlay);
    let surface_attrs = popover_surface_attributes(&overlay);

    html! {
        <main class="automation-shell">
            <header>
                <h1>{"Shared dialog state – Yew"}</h1>
                <p>{"Every control reads from the same shared state container exported by the core crate."}</p>
            </header>
            <section class="controls">
                <button class="primary" onclick={open_dialog.clone()}>{"Open dialog"}</button>
                <button onclick={close_dialog}>{"Close dialog"}</button>
                <button onclick={toggle_popover}>{"Toggle popover"}</button>
                <button onclick={shift_anchor}>{"Simulate anchor layout shift"}</button>
            </section>
            <section class="snapshot">
                <h2>{"Snapshot"}</h2>
                <pre>{format!("{:?}", snapshot)}</pre>
            </section>
            <section class="popover">
                <button class="anchor" onclick={toggle_popover.clone()}>
                    {"Shared anchor"}
                </button>
            </section>
            <section class="dialog">
                {
                    if overlay.dialog().is_open() {
                        let mut node = html! {
                            <section class="dialog-surface">
                                <header>
                                    <h2 id={DIALOG_TITLE_ID}>{"Automation review"}</h2>
                                </header>
                                <p id={DIALOG_DESCRIPTION_ID}>{"Dialogs, popovers, and text fields reuse the same controlled state across frameworks."}</p>
                                <footer>
                                    <button onclick={close_dialog.clone()}>{"Dismiss"}</button>
                                </footer>
                            </section>
                        };
                        if let VNode::VTag(ref mut tag) = node {
                            for (key, value) in dialog_attrs.clone() {
                                tag.add_attribute(key, value);
                            }
                        }
                        node
                    } else {
                        Html::default()
                    }
                }
            </section>
            <section class="popover-surface">
                {
                    if overlay.popover().is_open() {
                        let mut node = html! {
                            <div class="surface">
                                <p>{"Popover is open. Toggling uses shared state to track collisions."}</p>
                                <button onclick={toggle_popover.clone()}>{"Dismiss"}</button>
                            </div>
                        };
                        if let VNode::VTag(ref mut tag) = node {
                            for (key, value) in surface_attrs.clone() {
                                tag.add_attribute(key, value);
                            }
                        }
                        node
                    } else {
                        Html::default()
                    }
                }
            </section>
            <section class="text-field">
                <label for="shared-text-field">{"Company"}</label>
                {
                    let mut input = html! {
                        <input
                            id="shared-text-field"
                            value={overlay.text_field().value().to_string()}
                            placeholder="Enterprise name"
                            oninput={on_input}
                            onblur={on_blur.clone()}
                            onkeydown={on_keydown.clone()}
                        />
                    };
                    if let VNode::VTag(ref mut tag) = input {
                        for (key, value) in text_attrs.clone() {
                            tag.add_attribute(key, value);
                        }
                    }
                    input
                }
                <p id={TEXT_FIELD_STATUS_ID} data-role="status">
                    { status_message.unwrap_or_else(|| "Enter a company name with at least three characters.".into()) }
                </p>
            </section>
            <section class="anchor-attributes">
                <h2>{"Anchor metadata"}</h2>
                <ul>
                    { for anchor_attrs.iter().map(|(key, value)| html! { <li>{format!("{key}={value}")}</li> }) }
                </ul>
            </section>
            <section class="journal">
                <h2>{"Lifecycle journal"}</h2>
                <ol>
                    { for journal.iter().enumerate().map(|(idx, entry)| html! { <li key={idx}>{entry.clone()}</li> }) }
                </ol>
            </section>
        </main>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

printf '\n✅ Generated Yew shared dialog state demo in %s\n' "$EXAMPLE_ROOT"
