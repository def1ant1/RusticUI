#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="$(git rev-parse --show-toplevel)"
EXAMPLE_ROOT="$REPO_ROOT/target/shared-dialog-state-leptos-demo"

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
    <title>Shared Dialog State – Leptos</title>
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
leptos = { version = "0.5", features = ["csr"] }
leptos_dom = "0.5"
wasm-bindgen = "0.2"
web-sys = { version = "0.3", features = ["HtmlInputElement", "KeyboardEvent"] }
shared-dialog-state-core = { path = "../../../examples/shared-dialog-state-core" }
TOML

cat > "$EXAMPLE_ROOT/app-shell/src/main.rs" <<'RS'
use leptos::ev::{Event, KeyboardEvent};
use leptos::{event_target_value, logging, For, IntoView, SignalGet, SignalSet, SignalUpdate};
use shared_dialog_state_core::{
    LifecycleLog, SharedOverlayState, ANCHOR_DIAGRAM, DIALOG_DESCRIPTION_ID,
    DIALOG_SURFACE_ANALYTICS_ID, DIALOG_TITLE_ID, POPOVER_ANCHOR_ID,
    POPOVER_SURFACE_ANALYTICS_ID, TEXT_FIELD_ANALYTICS_ID, TEXT_FIELD_STATUS_ID,
};

fn push_log(set_journal: SignalUpdate<Vec<String>>, mut log: LifecycleLog) {
    if log.entries.is_empty() {
        return;
    }
    for entry in &log.entries {
        logging::log!("{}", entry);
    }
    set_journal.update(move |entries| entries.append(&mut log.entries));
}

#[component]
fn App() -> impl IntoView {
    leptos::on_mount(|| logging::log!("{}", ANCHOR_DIAGRAM));

    let (overlay, set_overlay) = leptos::create_signal(SharedOverlayState::enterprise_defaults());
    let (journal, set_journal) = leptos::create_signal(Vec::<String>::new());
    let push = move |log: LifecycleLog| push_log(set_journal, log);

    let open_dialog = move |_| {
        set_overlay.update(|state| {
            let current = std::mem::take(state);
            let (next, log) = current.request_dialog_open();
            push(log);
            *state = next;
        });
    };

    let close_dialog = move |_| {
        set_overlay.update(|state| {
            let current = std::mem::take(state);
            let (next, log) = current.request_dialog_close();
            push(log);
            *state = next;
        });
    };

    let toggle_popover = move |_| {
        set_overlay.update(|state| {
            let current = std::mem::take(state);
            let (next, log) = current.toggle_popover();
            push(log);
            *state = next;
        });
    };

    let shift_anchor = move |_| {
        set_overlay.update(|state| {
            let current = std::mem::take(state);
            let (next, log) = current.update_anchor_geometry(shared_dialog_state_core::AnchorGeometry {
                x: 180.0,
                y: 320.0,
                width: 210.0,
                height: 44.0,
            });
            push(log);
            *state = next;
        });
    };

    let on_input = move |ev: Event| {
        let value = event_target_value(&ev);
        set_overlay.update(|state| {
            let current = std::mem::take(state);
            let (next, log) = current.change_text(value.clone());
            push(log);
            *state = next;
        });
    };

    let commit = move || {
        set_overlay.update(|state| {
            let current = std::mem::take(state);
            let (next, log) = current.commit_text();
            push(log);
            *state = next;
        });
    };

    let reset = move || {
        set_overlay.update(|state| {
            let current = std::mem::take(state);
            let (next, log) = current.reset_text();
            push(log);
            *state = next;
        });
    };

    let on_blur = move |_| commit();
    let on_key = move |ev: KeyboardEvent| {
        match ev.key().as_str() {
            "Enter" => {
                ev.prevent_default();
                commit();
            }
            "Escape" => {
                ev.prevent_default();
                reset();
            }
            _ => {}
        }
    };

    let snapshot = move || overlay.get().snapshot();

    view! {
        <main class="automation-shell">
            <header>
                <h1>{"Shared dialog state – Leptos"}</h1>
                <p>{"The shared overlay state drives every attribute and validation branch."}</p>
            </header>
            <section class="controls">
                <button class="primary" on:click=open_dialog>{"Open dialog"}</button>
                <button on:click=close_dialog>{"Close dialog"}</button>
                <button on:click=toggle_popover>{"Toggle popover"}</button>
                <button on:click=shift_anchor>{"Simulate anchor layout shift"}</button>
            </section>
            <section class="snapshot">
                <h2>{"Snapshot"}</h2>
                <pre>{move || format!("{:?}", snapshot())}</pre>
            </section>
            <section class="popover">
                {move || {
                    let state = overlay.get();
                    let anchor = state.popover().anchor_attributes();
                    let anchor_id = anchor.id().map(|(_, value)| value.to_string());
                    let placement = anchor.data_placement().1.to_string();
                    view! {
                        <button class="anchor"
                            on:click=toggle_popover
                            attr:id=anchor_id
                            attr:data-popover-placement=placement
                        >{"Shared anchor"}</button>
                    }
                }}
            </section>
            <section class="dialog">
                {move || {
                    let state = overlay.get();
                    if state.dialog().is_open() {
                        let builder = state
                            .dialog()
                            .surface_attributes()
                            .id("shared-dialog-surface")
                            .labelled_by(DIALOG_TITLE_ID)
                            .described_by(DIALOG_DESCRIPTION_ID)
                            .analytics_id(DIALOG_SURFACE_ANALYTICS_ID);
                        let aria_modal = builder.aria_modal().1.to_string();
                        let dialog_id = builder.id_attr().map(|(_, value)| value.to_string());
                        let labelled = builder.aria_labelledby().map(|(_, value)| value.to_string());
                        let described = builder.aria_describedby().map(|(_, value)| value.to_string());
                        let data_state = builder.data_state().1.to_string();
                        let data_focus = builder.data_focus_trap().1.to_string();
                        let data_transition = builder
                            .data_transition()
                            .map(|(_, value)| value.to_string());
                        let analytics = builder
                            .data_analytics_id()
                            .map(|(_, value)| value.to_string());
                        view! {
                            <section class="dialog-surface"
                                role=builder.role()
                                attr:aria-modal=aria_modal
                                attr:id=dialog_id
                                attr:aria-labelledby=labelled
                                attr:aria-describedby=described
                                attr:data-state=data_state
                                attr:data-focus-trap=data_focus
                                attr:data-transition=data_transition
                                attr:data-analytics-id=analytics
                            >
                                <header>
                                    <h2 id={DIALOG_TITLE_ID}>{"Automation review"}</h2>
                                </header>
                                <p id={DIALOG_DESCRIPTION_ID}>{"Dialogs, popovers, and text fields reuse the same state across frameworks."}</p>
                                <footer>
                                    <button on:click=close_dialog>{"Dismiss"}</button>
                                </footer>
                            </section>
                        }.into_view()
                    } else {
                        view! { <></> }.into_view()
                    }
                }}
            </section>
            <section class="popover-surface">
                {move || {
                    let state = overlay.get();
                    if state.popover().is_open() {
                        let builder = state
                            .popover()
                            .surface_attributes()
                            .analytics_id(POPOVER_SURFACE_ANALYTICS_ID);
                        let data_open = builder.data_open().1.to_string();
                        let data_preferred = builder.data_preferred().1.to_string();
                        let data_resolved = builder.data_resolved().1.to_string();
                        let analytics = builder
                            .data_analytics_id()
                            .map(|(_, value)| value.to_string());
                        view! {
                            <div class="surface"
                                attr:data-open=data_open
                                attr:data-preferred-placement=data_preferred
                                attr:data-resolved-placement=data_resolved
                                attr:data-analytics-id=analytics
                            >
                                <p>{"Popover is open. Collision resolution mirrors SSR output."}</p>
                                <button on:click=toggle_popover>{"Dismiss"}</button>
                            </div>
                        }.into_view()
                    } else {
                        view! { <></> }.into_view()
                    }
                }}
            </section>
            <section class="text-field">
                <label for="shared-text-field">{"Company"}</label>
                {move || {
                    let state = overlay.get();
                    let builder = state
                        .text_field()
                        .attributes()
                        .status_id(TEXT_FIELD_STATUS_ID)
                        .analytics_id(TEXT_FIELD_ANALYTICS_ID);
                    let aria_invalid = builder.aria_invalid().map(|(_, value)| value.to_string());
                    let described = builder.aria_describedby().map(|(_, value)| value.to_string());
                    let data_dirty = builder.data_dirty().1.to_string();
                    let data_visited = builder.data_visited().1.to_string();
                    let analytics = builder
                        .data_analytics_id()
                        .map(|(_, value)| value.to_string());
                    let status_message = builder
                        .status_message()
                        .unwrap_or_else(|| "Enter a company name with at least three characters.".into());
                    view! {
                        <>
                            <input
                                id="shared-text-field"
                                prop:value=state.text_field().value().to_string()
                                placeholder="Enterprise name"
                                on:input=on_input
                                on:blur=on_blur
                                on:keydown=on_key
                                attr:aria-invalid=aria_invalid
                                attr:aria-describedby=described
                                attr:data-dirty=data_dirty
                                attr:data-visited=data_visited
                                attr:data-analytics-id=analytics
                            />
                            <p id={TEXT_FIELD_STATUS_ID} data-role="status">{status_message}</p>
                        </>
                    }.into_view()
                }}
            </section>
            <section class="journal">
                <h2>{"Lifecycle journal"}</h2>
                <ol>
                    <For
                        each=move || journal.get().into_iter().enumerate().collect::<Vec<_>>()
                        key=|(idx, _)| *idx
                        children=move |(_, entry)| view! { <li>{entry}</li> }
                    />
                </ol>
            </section>
        </main>
    }
}

fn main() {
    leptos::mount_to_body(App);
}

printf '\n✅ Generated Leptos shared dialog state demo in %s\n' "$EXAMPLE_ROOT"
