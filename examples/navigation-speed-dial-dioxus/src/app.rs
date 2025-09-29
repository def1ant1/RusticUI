use crate::telemetry::{TelemetryLog, TelemetryRecord};
use dioxus::prelude::*;
use rustic_ui_headless::speed_dial::{SpeedDialAnalyticsEvent, SpeedDialSelection, SpeedDialState};
use rustic_ui_headless::ControlStrategy;
use rustic_ui_material::speed_dial::{
    render_speed_dial_html, SpeedDialActionDescriptor, SpeedDialAdapterProps,
    SpeedDialTriggerDescriptor,
};
use time::OffsetDateTime;

pub const TELEMETRY_CHANNEL: &str = "navigation.speed_dial";

struct ActionDefinition {
    tag: &'static str,
    label: &'static str,
    aria_label: &'static str,
}

const ACTIONS: &[ActionDefinition] = &[
    ActionDefinition {
        tag: "action.create",
        label: "Create incident",
        aria_label: "Create a new incident",
    },
    ActionDefinition {
        tag: "action.escalate",
        label: "Escalate to on-call",
        aria_label: "Escalate the current incident to the on-call engineer",
    },
    ActionDefinition {
        tag: "action.archive",
        label: "Archive incident",
        aria_label: "Archive the resolved incident",
    },
];

pub(crate) fn build_state(open: bool, highlighted: Option<usize>) -> SpeedDialState {
    let mut state = SpeedDialState::new(
        ACTIONS.len(),
        open,
        ControlStrategy::Controlled,
        ControlStrategy::Controlled,
    );
    state.set_analytics_channel(Some(TELEMETRY_CHANNEL));
    for (index, action) in ACTIONS.iter().enumerate() {
        state.set_action_analytics_tag(index, Some(action.tag.to_string()));
    }
    state.sync_open(open);
    state.sync_highlighted(highlighted);
    state
}

pub(crate) fn render_markup(state: &SpeedDialState) -> String {
    let actions: Vec<SpeedDialActionDescriptor<'_>> = ACTIONS
        .iter()
        .enumerate()
        .map(|(index, action)| SpeedDialActionDescriptor {
            index,
            id: None,
            analytics_tag: Some(action.tag),
            aria_label: Some(action.aria_label),
            content: action.label,
        })
        .collect();
    render_speed_dial_html(SpeedDialAdapterProps {
        state,
        trigger_attributes: state
            .trigger_attributes()
            .id("speed-dial-trigger")
            .analytics_tag("trigger.create"),
        list_attributes: state.list_attributes(),
        trigger: SpeedDialTriggerDescriptor {
            id: Some("speed-dial-trigger"),
            analytics_tag: Some("trigger.create"),
            content:
                "<span aria-hidden=\"true\">＋</span><span class=\"sr-only\">Open speed dial</span>",
        },
        actions: &actions,
        on_action_event: Some("navigation.speed_dial.action"),
        on_toggle_event: Some("navigation.speed_dial.toggle"),
    })
}

fn push_event(telemetry: &UseState<TelemetryLog>, event: SpeedDialAnalyticsEvent) {
    let mut next = (*telemetry.get()).clone();
    next.push(TelemetryRecord::from_speed_dial(
        event,
        OffsetDateTime::now_utc(),
    ));
    telemetry.set(next);
}

fn toggle_speed_dial(
    open_state: &UseState<bool>,
    highlighted_state: &UseState<Option<usize>>,
    telemetry_state: &UseState<TelemetryLog>,
) {
    let mut state = build_state(*open_state.get(), *highlighted_state.get());
    if let Some(event) = state.toggle(|next| open_state.set(next)) {
        push_event(telemetry_state, event);
    }
    if *open_state.get() {
        highlighted_state.set(Some(0));
    }
}

fn activate_speed_dial(
    index: usize,
    open_state: &UseState<bool>,
    highlighted_state: &UseState<Option<usize>>,
    telemetry_state: &UseState<TelemetryLog>,
) {
    let mut state = build_state(*open_state.get(), *highlighted_state.get());
    let mut activation_event: Option<SpeedDialAnalyticsEvent> = None;
    let outcome = state.activate(index, |selection: SpeedDialSelection| {
        highlighted_state.set(Some(selection.index));
        if let Some(event) = &selection.analytics {
            activation_event = Some(event.clone());
        }
    });
    if let Some(event) = outcome.analytics.or(activation_event) {
        push_event(telemetry_state, event);
    }
    if let Some(event) = state.close(|next| open_state.set(next)) {
        push_event(telemetry_state, event);
    }
    highlighted_state.set(Some(0));
}

#[allow(non_snake_case)]
pub fn SpeedDialApp(cx: Scope) -> Element {
    let open_state = use_state(cx, || false);
    let highlighted_state = use_state(cx, || Some(0usize));
    let telemetry_state = use_state(cx, TelemetryLog::default);

    let state = build_state(*open_state.get(), *highlighted_state.get());
    let markup = render_markup(&state);

    let telemetry_text = {
        let log = telemetry_state.get();
        if log.iter().next().is_none() {
            "// Use the buttons above to open the dial or trigger actions.".to_string()
        } else {
            log.as_json_lines()
        }
    };

    cx.render(rsx! {
        div {
            style: "min-height:100vh;background:#020617;color:#f8fafc;font-family:'Inter',sans-serif;padding:32px;box-sizing:border-box;",
            "data-rustic-speed-dial-shell": TELEMETRY_CHANNEL,
            header {
                style: "max-width:860px;margin:0 auto 32px auto;",
                p {
                    style: "text-transform:uppercase;letter-spacing:0.08em;font-size:0.75rem;color:#38bdf8;margin:0 0 8px 0;",
                    "RusticUI speed dial — Dioxus"
                }
                h1 {
                    style: "font-size:2.5rem;margin:0 0 16px 0;",
                    "Instant command launcher with telemetry instrumentation"
                }
                p {
                    style: "max-width:65ch;line-height:1.6;margin:0;",
                    "The speed dial mirrors production wiring: controlled open state, analytics fan-out, deterministic SSR markup, and automation-friendly selectors."
                }
            }
            main {
                style: "max-width:860px;margin:0 auto;display:flex;flex-direction:column;gap:24px;",
                section {
                    h2 { style: "font-size:1.25rem;margin:0 0 12px 0;", "Interactive controls" }
                    div {
                        style: "display:flex;flex-wrap:wrap;gap:12px;",
                        {
                            let open_state = open_state.clone();
                            let highlighted_state = highlighted_state.clone();
                            let telemetry_state = telemetry_state.clone();
                            rsx! {
                                button {
                                    r#type: "button",
                                    onclick: move |_| toggle_speed_dial(&open_state, &highlighted_state, &telemetry_state),
                                    "data-rustic-speed-dial-trigger": "manual-toggle",
                                    style: "background:#1e293b;color:#38bdf8;border:1px solid rgba(56,189,248,0.4);padding:10px 18px;border-radius:8px;font-weight:600;",
                                    if *open_state.get() { "Close dial" } else { "Open dial" }
                                }
                            }
                        }
                        {ACTIONS.iter().enumerate().map(|(index, action)| {
                            let open_state = open_state.clone();
                            let highlighted_state = highlighted_state.clone();
                            let telemetry_state = telemetry_state.clone();
                            rsx! {
                                button {
                                    r#type: "button",
                                    "data-rustic-speed-dial-action": action.tag,
                                    style: "background:#1e293b;color:#a5b4fc;border:1px solid rgba(165,180,252,0.35);padding:10px 18px;border-radius:8px;font-weight:600;",
                                    onclick: move |_| activate_speed_dial(index, &open_state, &highlighted_state, &telemetry_state),
                                    "Trigger "{action.label}
                                }
                            }
                        })}
                    }
                }
                section {
                    h2 { style: "font-size:1.25rem;margin:0 0 12px 0;", "Rendered speed dial" }
                    div {
                        id: "speed-dial-root",
                        "data-rustic-speed-dial": TELEMETRY_CHANNEL,
                        style: "background:#0f172a;border-radius:16px;padding:32px;display:flex;justify-content:center;",
                        dangerous_inner_html: "{markup}",
                    }
                }
                section {
                    h2 { style: "font-size:1.25rem;margin:0 0 12px 0;", "Telemetry stream" }
                    pre {
                        "data-rustic-analytics-log": "",
                        style: "background:#000316;border-radius:12px;padding:16px;font-size:0.85rem;line-height:1.5;overflow:auto;max-height:220px;",
                        "{telemetry_text}"
                    }
                }
            }
        }
    })
}
