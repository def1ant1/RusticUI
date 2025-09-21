#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="$(git rev-parse --show-toplevel)"
EXAMPLE_ROOT="$REPO_ROOT/target/shared-dialog-state-dioxus-demo"

rm -rf "$EXAMPLE_ROOT"
mkdir -p "$EXAMPLE_ROOT/app-shell/src"

cat > "$EXAMPLE_ROOT/Cargo.toml" <<'TOML'
[workspace]
members = ["app-shell"]
resolver = "2"
TOML

cat > "$EXAMPLE_ROOT/index.html" <<'HTML'
<!doctype html>
<html lang="en">
  <head>
    <meta charset="utf-8" />
    <title>Shared Dialog State – Dioxus</title>
  </head>
  <body>
    <div id="main"></div>
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
dioxus = { version = "0.4", features = ["web"] }
dioxus-logger = "0.4"
shared-dialog-state-core = { path = "../../../examples/shared-dialog-state-core" }
TOML

cat > "$EXAMPLE_ROOT/app-shell/src/main.rs" <<'RS'
use dioxus::events::{FormEvent, KeyboardEvent};
use dioxus::prelude::*;
use shared_dialog_state_core::{
    LifecycleLog, SharedOverlayState, ANCHOR_DIAGRAM, DIALOG_DESCRIPTION_ID,
    DIALOG_SURFACE_ANALYTICS_ID, DIALOG_TITLE_ID, POPOVER_ANCHOR_ID,
    POPOVER_SURFACE_ANALYTICS_ID, TEXT_FIELD_ANALYTICS_ID, TEXT_FIELD_STATUS_ID,
};

fn push_log(state: &UseState<Vec<String>>, mut log: LifecycleLog) {
    if log.entries.is_empty() {
        return;
    }
    for entry in &log.entries {
        dioxus_logger::tracing::info!("{}", entry);
    }
    let mut next = state.get().clone();
    next.append(&mut log.entries);
    state.set(next);
}

fn main() {
    dioxus_logger::init(dioxus_logger::tracing::Level::INFO).ok();
    dioxus::launch(App);
}

fn App(cx: Scope) -> Element {
    use_future(cx, (), |_| async move {
        println!("{}", ANCHOR_DIAGRAM);
    });

    let overlay = use_state(cx, SharedOverlayState::enterprise_defaults);
    let journal = use_state(cx, Vec::<String>::new);

    let open_dialog = {
        let overlay = overlay.clone();
        let journal = journal.clone();
        move |_| {
            let (next, log) = overlay.get().clone().request_dialog_open();
            overlay.set(next);
            push_log(&journal, log);
        }
    };

    let close_dialog = {
        let overlay = overlay.clone();
        let journal = journal.clone();
        move |_| {
            let (next, log) = overlay.get().clone().request_dialog_close();
            overlay.set(next);
            push_log(&journal, log);
        }
    };

    let toggle_popover = {
        let overlay = overlay.clone();
        let journal = journal.clone();
        move |_| {
            let (next, log) = overlay.get().clone().toggle_popover();
            overlay.set(next);
            push_log(&journal, log);
        }
    };

    let shift_anchor = {
        let overlay = overlay.clone();
        let journal = journal.clone();
        move |_| {
            let (next, log) = overlay
                .get()
                .clone()
                .update_anchor_geometry(shared_dialog_state_core::AnchorGeometry {
                    x: 200.0,
                    y: 340.0,
                    width: 230.0,
                    height: 48.0,
                });
            overlay.set(next);
            push_log(&journal, log);
        }
    };

    let on_input = {
        let overlay = overlay.clone();
        let journal = journal.clone();
        move |evt: FormEvent| {
            let (next, log) = overlay.get().clone().change_text(evt.value.clone());
            overlay.set(next);
            push_log(&journal, log);
        }
    };

    let commit_text = {
        let overlay = overlay.clone();
        let journal = journal.clone();
        move || {
            let (next, log) = overlay.get().clone().commit_text();
            overlay.set(next);
            push_log(&journal, log);
        }
    };

    let reset_text = {
        let overlay = overlay.clone();
        let journal = journal.clone();
        move || {
            let (next, log) = overlay.get().clone().reset_text();
            overlay.set(next);
            push_log(&journal, log);
        }
    };

    let on_blur = {
        let commit_text = commit_text.clone();
        move |_| commit_text()
    };

    let on_key = move |evt: KeyboardEvent| {
        match evt.key().as_str() {
            "Enter" => {
                evt.prevent_default();
                commit_text();
            }
            "Escape" => {
                evt.prevent_default();
                reset_text();
            }
            _ => {}
        }
    };

    let state_snapshot = overlay.get().clone();
    let snapshot = state_snapshot.snapshot();

    let dialog_builder = state_snapshot
        .dialog()
        .surface_attributes()
        .id("shared-dialog-surface")
        .labelled_by(DIALOG_TITLE_ID)
        .described_by(DIALOG_DESCRIPTION_ID)
        .analytics_id(DIALOG_SURFACE_ANALYTICS_ID);
    let dialog_role = dialog_builder.role();
    let aria_modal = dialog_builder.aria_modal().1.to_string();
    let dialog_id = dialog_builder.id_attr().map(|(_, value)| value.to_string()).unwrap_or_default();
    let aria_labelledby = dialog_builder
        .aria_labelledby()
        .map(|(_, value)| value.to_string())
        .unwrap_or_default();
    let aria_describedby = dialog_builder
        .aria_describedby()
        .map(|(_, value)| value.to_string())
        .unwrap_or_default();
    let data_state = dialog_builder.data_state().1.to_string();
    let data_focus = dialog_builder.data_focus_trap().1.to_string();
    let data_transition = dialog_builder
        .data_transition()
        .map(|(_, value)| value.to_string())
        .unwrap_or_default();
    let dialog_analytics = dialog_builder
        .data_analytics_id()
        .map(|(_, value)| value.to_string())
        .unwrap_or_default();

    let anchor_builder = state_snapshot.popover().anchor_attributes();
    let anchor_id = anchor_builder
        .id()
        .map(|(_, value)| value.to_string())
        .unwrap_or_else(|| POPOVER_ANCHOR_ID.to_string());
    let anchor_placement = anchor_builder.data_placement().1.to_string();

    let surface_builder = state_snapshot
        .popover()
        .surface_attributes()
        .analytics_id(POPOVER_SURFACE_ANALYTICS_ID);
    let popover_open = surface_builder.data_open().1.to_string();
    let popover_preferred = surface_builder.data_preferred().1.to_string();
    let popover_resolved = surface_builder.data_resolved().1.to_string();
    let popover_analytics = surface_builder
        .data_analytics_id()
        .map(|(_, value)| value.to_string())
        .unwrap_or_default();

    let text_builder = state_snapshot
        .text_field()
        .attributes()
        .status_id(TEXT_FIELD_STATUS_ID)
        .analytics_id(TEXT_FIELD_ANALYTICS_ID);
    let aria_invalid = text_builder
        .aria_invalid()
        .map(|(_, value)| value.to_string())
        .unwrap_or_default();
    let text_describedby = text_builder
        .aria_describedby()
        .map(|(_, value)| value.to_string())
        .unwrap_or(TEXT_FIELD_STATUS_ID.to_string());
    let data_dirty = text_builder.data_dirty().1.to_string();
    let data_visited = text_builder.data_visited().1.to_string();
    let text_analytics = text_builder
        .data_analytics_id()
        .map(|(_, value)| value.to_string())
        .unwrap_or_default();
    let status_message = text_builder
        .status_message()
        .unwrap_or_else(|| "Enter a company name with at least three characters.".into());

    let anchor_metadata = vec![
        format!("id={anchor_id}"),
        format!("data-popover-placement={anchor_placement}"),
    ];

    let journal_entries = journal.get().clone();

    cx.render(rsx! {
        main { class: "automation-shell",
            header {
                h1 { "Shared dialog state – Dioxus" }
                p { "The Dioxus adapter consumes the same shared state as the other frameworks." }
            }
            section { class: "controls",
                button { class: "primary", onclick: open_dialog, "Open dialog" }
                button { onclick: close_dialog, "Close dialog" }
                button { onclick: toggle_popover, "Toggle popover" }
                button { onclick: shift_anchor, "Simulate anchor layout shift" }
            }
            section { class: "snapshot",
                h2 { "Snapshot" }
                pre { "{snapshot:?}" }
            }
            section { class: "popover",
                button {
                    onclick: toggle_popover,
                    id: "{anchor_id}",
                    "data-popover-placement": "{anchor_placement}",
                    "Shared anchor"
                }
            }
            section { class: "dialog",
                if state_snapshot.dialog().is_open() {
                    rsx! {
                        section {
                            class: "dialog-surface",
                            role: "{dialog_role}",
                            "aria-modal": "{aria_modal}",
                            id: "{dialog_id}",
                            "aria-labelledby": "{aria_labelledby}",
                            "aria-describedby": "{aria_describedby}",
                            "data-state": "{data_state}",
                            "data-focus-trap": "{data_focus}",
                            "data-transition": "{data_transition}",
                            "data-analytics-id": "{dialog_analytics}",
                            header { h2 { id: "{DIALOG_TITLE_ID}", "Automation review" } }
                            p { id: "{DIALOG_DESCRIPTION_ID}", "Dialogs, popovers, and text fields reuse the shared state across frameworks." }
                            footer { button { onclick: close_dialog, "Dismiss" } }
                        }
                    }
                } else {
                    rsx! { div {} }
                }
            }
            section { class: "popover-surface",
                if state_snapshot.popover().is_open() {
                    rsx! {
                        div {
                            class: "surface",
                            "data-open": "{popover_open}",
                            "data-preferred-placement": "{popover_preferred}",
                            "data-resolved-placement": "{popover_resolved}",
                            "data-analytics-id": "{popover_analytics}",
                            p { "Popover is open. Collision logic mirrors SSR output." }
                            button { onclick: toggle_popover, "Dismiss" }
                        }
                    }
                } else {
                    rsx! { div {} }
                }
            }
            section { class: "text-field",
                label { r#for: "shared-text-field", "Company" }
                input {
                    id: "shared-text-field",
                    value: "{state_snapshot.text_field().value()}",
                    placeholder: "Enterprise name",
                    oninput: on_input,
                    onblur: on_blur,
                    onkeydown: on_key,
                    "aria-invalid": "{aria_invalid}",
                    "aria-describedby": "{text_describedby}",
                    "data-dirty": "{data_dirty}",
                    "data-visited": "{data_visited}",
                    "data-analytics-id": "{text_analytics}"
                }
                p { id: "{TEXT_FIELD_STATUS_ID}", "data-role": "status", "{status_message}" }
            }
            section { class: "anchor-attributes",
                h2 { "Anchor metadata" }
                ul {
                    {anchor_metadata.iter().enumerate().map(|(idx, item)| rsx!{ li { key: "{idx}", "{item}" } })}
                }
            }
            section { class: "journal",
                h2 { "Lifecycle journal" }
                ol {
                    {journal_entries.iter().enumerate().map(|(idx, entry)| rsx!{ li { key: "{idx}", "{entry}" } })}
                }
            }
        }
    })
}

printf '\n✅ Generated Dioxus shared dialog state demo in %s\n' "$EXAMPLE_ROOT"
