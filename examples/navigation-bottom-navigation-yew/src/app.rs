use crate::telemetry::{TelemetryLog, TelemetryRecord};
use rustic_ui_headless::bottom_navigation::{
    BottomNavigationActivationMode, BottomNavigationSelection, BottomNavigationState,
};
use rustic_ui_headless::ControlStrategy;
use rustic_ui_material::bottom_navigation::{
    render_bottom_navigation_html, BottomNavigationAdapterProps, BottomNavigationItemDescriptor,
};
use time::OffsetDateTime;
use yew::prelude::*;

/// Container id shared between SSR output and CSR hydration.
pub const HYDRATION_CONTAINER_ID: &str = "navigation-bottom-navigation-root";

/// Human-friendly descriptor for each navigation destination used in telemetry.
pub(crate) const DESTINATIONS: &[(&str, &str)] = &[
    ("overview", "Overview"),
    ("analytics", "Analytics"),
    ("deployments", "Deployments"),
    ("billing", "Billing"),
];

pub(crate) fn configure_state(
    selected: Option<usize>,
    focused: Option<usize>,
) -> BottomNavigationState {
    let mut state = BottomNavigationState::new(
        DESTINATIONS.len(),
        selected,
        BottomNavigationActivationMode::Automatic,
        ControlStrategy::Controlled,
        ControlStrategy::Controlled,
    );
    state.set_analytics_channel(Some("navigation.bottom"));
    for (index, (tag, _label)) in DESTINATIONS.iter().enumerate() {
        state.set_item_analytics_tag(index, Some(format!("destination.{tag}")));
    }
    if let Some(index) = focused {
        state.sync_focused(Some(index));
    }
    state
}

/// Main Yew component rendering the navigation, automation hooks, and telemetry log.
#[function_component(BottomNavigationApp)]
pub fn bottom_navigation_app() -> Html {
    // Controlled selection and focus mirror the enterprise pattern where the shell owns state and
    // the component emits intents. Analytics are also forwarded upstream via the telemetry log.
    let selected = use_state(|| Some(0usize));
    let focused = use_state(|| Some(0usize));
    let telemetry = use_state(TelemetryLog::default);

    let markup = {
        let selected = selected.clone();
        let focused = focused.clone();
        use_memo((selected.clone(), focused.clone()), move |_| {
            let state = configure_state(*selected, *focused);
            let labels: Vec<String> = DESTINATIONS
                .iter()
                .map(|(_, label)| (*label).to_string())
                .collect();
            let descriptors: Vec<BottomNavigationItemDescriptor<'_>> = labels
                .iter()
                .map(|label| BottomNavigationItemDescriptor {
                    id: None,
                    controls: None,
                    content: label.as_str(),
                })
                .collect();
            render_bottom_navigation_html(BottomNavigationAdapterProps {
                state: &state,
                attributes: state
                    .root_attributes()
                    .id(HYDRATION_CONTAINER_ID)
                    .labelled_by("navigation-bottom-label"),
                items: &descriptors,
                on_select_event: Some("navigation.bottom.select"),
            })
        })
    };

    let on_select_destination = {
        let selected = selected.clone();
        let focused = focused.clone();
        let telemetry = telemetry.clone();
        Callback::from(move |index: usize| {
            let mut state = configure_state(*selected, *focused);
            let mut next_selected = *selected;
            let mut next_focused = *focused;
            let mut new_record: Option<TelemetryRecord> = None;

            let analytics_outcome =
                state.select_index(index, |selection: BottomNavigationSelection| {
                    next_selected = Some(selection.index);
                    next_focused = Some(selection.index);
                    if let Some(event) = &selection.analytics {
                        new_record = Some(TelemetryRecord::from_bottom_nav(
                            event.clone(),
                            OffsetDateTime::now_utc(),
                        ));
                    }
                });

            selected.set(next_selected);
            focused.set(next_focused);

            if let Some(event) = analytics_outcome
                .map(|payload| TelemetryRecord::from_bottom_nav(payload, OffsetDateTime::now_utc()))
                .or(new_record)
            {
                let mut next_log = (*telemetry).clone();
                next_log.push(event);
                telemetry.set(next_log);
            }
        })
    };

    let markup_value = (*markup).clone();
    let automation_channel = "navigation.bottom";
    let telemetry_lines = telemetry
        .iter()
        .map(|record| record.to_json_line())
        .collect::<Vec<_>>();

    html! {
        <div
            style="min-height:100vh;background:#0b1120;color:#f1f5f9;font-family:'Inter',sans-serif;padding:32px;box-sizing:border-box;"
            data-rustic-analytics-shell={automation_channel}
        >
            <header style="max-width:860px;margin:0 auto 32px auto;">
                <p id="navigation-bottom-label" style="text-transform:uppercase;letter-spacing:0.08em;font-size:0.75rem;color:#38bdf8;margin:0 0 8px 0;">
                    {"RusticUI bottom navigation — Yew"}
                </p>
                <h1 style="font-size:2.5rem;margin:0 0 16px 0;">
                    {"Persistent navigation for observability-first dashboards"}
                </h1>
                <p style="max-width:60ch;line-height:1.6;margin:0;">
                    {"Selections stream to a shared analytics channel while SSR and CSR builds reuse the exact markup."}
                </p>
            </header>
            <main style="max-width:860px;margin:0 auto;display:flex;flex-direction:column;gap:24px;">
                <section>
                    <h2 style="font-size:1.25rem;margin:0 0 12px 0;">{"Interactive controls"}</h2>
                    <div style="display:flex;flex-wrap:wrap;gap:12px;">
                        { for DESTINATIONS.iter().enumerate().map(|(index, (tag, label))| {
                            let on_select_destination = on_select_destination.clone();
                            let is_active = *selected == Some(index);
                            let button_style = if is_active {
                                "background:#38bdf8;color:#0b1120;border:none;padding:10px 18px;border-radius:999px;font-weight:600;"
                            } else {
                                "background:rgba(56,189,248,0.2);color:#38bdf8;border:1px solid rgba(56,189,248,0.4);padding:10px 18px;border-radius:999px;font-weight:600;"
                            };
                            html! {
                                <button
                                    type="button"
                                    style={button_style}
                                    onclick={Callback::from(move |_| on_select_destination.emit(index))}
                                    data-rustic-analytics-trigger={format!("destination.{tag}")}
                                >
                                    {(*label).to_string()}
                                </button>
                            }
                        }) }
                    </div>
                </section>
                <section>
                    <h2 style="font-size:1.25rem;margin:0 0 12px 0;">{"Rendered navigation"}</h2>
                    <div
                        id={HYDRATION_CONTAINER_ID}
                        data-rustic-bottom-nav={automation_channel}
                        dangerously_set_inner_html={markup_value}
                        style="background:#111827;border-radius:16px;padding:24px;display:flex;justify-content:center;"
                    />
                </section>
                <section>
                    <h2 style="font-size:1.25rem;margin:0 0 12px 0;">{"Telemetry stream"}</h2>
                    <pre
                        data-rustic-analytics-log={automation_channel}
                        style="background:#020617;border-radius:12px;padding:16px;font-size:0.85rem;line-height:1.5;overflow:auto;max-height:220px;"
                    >
                        { if telemetry_lines.is_empty() {
                            "// Interact with the controls above to stream analytics events.".to_string()
                        } else {
                            telemetry_lines.join("\n")
                        }}
                    </pre>
                </section>
            </main>
        </div>
    }
}
