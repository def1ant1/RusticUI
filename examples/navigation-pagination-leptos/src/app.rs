use crate::telemetry::{TelemetryLog, TelemetryRecord};
use leptos::*;
use rustic_ui_headless::pagination::{PaginationItemKind, PaginationSelection, PaginationState};
use rustic_ui_headless::ControlStrategy;
use rustic_ui_material::pagination::{
    render_pagination_html, PaginationAdapterProps, PaginationItemDescriptor,
};
use time::OffsetDateTime;

pub const HYDRATION_CONTAINER_ID: &str = "navigation-pagination-root";
const PAGE_COUNT: usize = 7;
pub(crate) const PAGE_TAGS: [&str; PAGE_COUNT] = [
    "page.0", "page.1", "page.2", "page.3", "page.4", "page.5", "page.6",
];
pub(crate) const PAGE_LABELS: [&str; PAGE_COUNT] = [
    "Incidents",
    "Deployments",
    "Traffic",
    "Latency",
    "Synthetic checks",
    "Cloud costs",
    "Experiments",
];

pub(crate) fn configure_state(current: Option<usize>) -> PaginationState {
    let mut state = PaginationState::new(
        PAGE_COUNT,
        current,
        ControlStrategy::Controlled,
        ControlStrategy::Controlled,
    );
    state.set_analytics_channel(Some("navigation.pagination"));
    for (index, tag) in PAGE_TAGS.iter().enumerate() {
        state.set_page_analytics_tag(index, Some(tag.to_string()));
    }
    state
}

pub(crate) fn build_descriptors<'a>(labels: &'a [String]) -> Vec<PaginationItemDescriptor<'a>> {
    let mut descriptors = Vec::new();
    descriptors.push(PaginationItemDescriptor {
        kind: PaginationItemKind::First,
        id: None,
        aria_label: Some("First page"),
        content: "«",
    });
    descriptors.push(PaginationItemDescriptor {
        kind: PaginationItemKind::Previous,
        id: None,
        aria_label: Some("Previous page"),
        content: "‹",
    });
    for (index, label) in labels.iter().enumerate() {
        descriptors.push(PaginationItemDescriptor {
            kind: PaginationItemKind::Page(index),
            id: None,
            aria_label: None,
            content: label.as_str(),
        });
    }
    descriptors.push(PaginationItemDescriptor {
        kind: PaginationItemKind::Next,
        id: None,
        aria_label: Some("Next page"),
        content: "›",
    });
    descriptors.push(PaginationItemDescriptor {
        kind: PaginationItemKind::Last,
        id: None,
        aria_label: Some("Last page"),
        content: "»",
    });
    descriptors
}

#[component]
pub fn PaginationApp() -> impl IntoView {
    let current_page = create_rw_signal(Some(0usize));
    let telemetry = create_rw_signal(TelemetryLog::default());

    let markup = create_memo(move |_| {
        let state = configure_state(current_page.get());
        let labels = PAGE_LABELS
            .iter()
            .map(|label| label.to_string())
            .collect::<Vec<_>>();
        let descriptors = build_descriptors(&labels);
        render_pagination_html(PaginationAdapterProps {
            state: &state,
            root_attributes: state
                .root_attributes()
                .id(HYDRATION_CONTAINER_ID)
                .labelled_by("navigation-pagination-label"),
            list_attributes: state.list_attributes(),
            items: &descriptors,
            on_select_event: Some("navigation.pagination.select"),
        })
    });

    let activate = move |kind: PaginationItemKind| {
        let mut state = configure_state(current_page.get());
        let mut next_page = current_page.get();
        let mut telemetry_record: Option<TelemetryRecord> = None;

        let outcome = state.activate(kind, |selection: PaginationSelection| {
            next_page = Some(selection.page_index);
            if let Some(event) = &selection.analytics {
                telemetry_record = Some(TelemetryRecord::from_pagination(
                    event.clone(),
                    OffsetDateTime::now_utc(),
                ));
            }
        });

        current_page.set(next_page);

        if let Some(event) = outcome
            .analytics
            .map(|payload| TelemetryRecord::from_pagination(payload, OffsetDateTime::now_utc()))
            .or(telemetry_record)
        {
            telemetry.update(|log| log.push(event));
        }
    };

    let telemetry_lines = move || {
        let log = telemetry.get();
        if log.iter().count() == 0 {
            "// Interact with the pagination controls to stream analytics.".to_string()
        } else {
            log.as_json_lines()
        }
    };

    view! {
        <div
            style="min-height:100vh;background:#030712;color:#e2e8f0;font-family:'Inter',sans-serif;padding:32px;box-sizing:border-box;"
            data-rustic-pagination-shell="navigation.pagination"
        >
            <header style="max-width:900px;margin:0 auto 32px auto;">
                <p id="navigation-pagination-label" style="text-transform:uppercase;letter-spacing:0.08em;font-size:0.75rem;color:#60a5fa;margin:0 0 8px 0;">
                    {"RusticUI pagination — Leptos"}
                </p>
                <h1 style="font-size:2.5rem;margin:0 0 16px 0;">
                    {"Telemetry-friendly pagination for SSR + CSR parity"}
                </h1>
                <p style="max-width:68ch;line-height:1.6;margin:0;">
                    {"Selections stream to a shared analytics channel and mirror the SSR snapshot so enterprise QA can diff hydration with confidence."}
                </p>
            </header>
            <main style="max-width:900px;margin:0 auto;display:flex;flex-direction:column;gap:24px;">
                <section>
                    <h2 style="font-size:1.25rem;margin:0 0 12px 0;">{"Interactive drivers"}</h2>
                    <div style="display:flex;flex-wrap:wrap;gap:12px;">
                        <button
                            type="button"
                            on:click=move |_| activate(PaginationItemKind::First)
                            data-rustic-pagination-trigger="first"
                            style="background:#1e293b;color:#60a5fa;border:1px solid rgba(96,165,250,0.4);padding:10px 18px;border-radius:8px;font-weight:600;"
                        >"First"</button>
                        <button
                            type="button"
                            on:click=move |_| activate(PaginationItemKind::Previous)
                            data-rustic-pagination-trigger="previous"
                            style="background:#1e293b;color:#60a5fa;border:1px solid rgba(96,165,250,0.4);padding:10px 18px;border-radius:8px;font-weight:600;"
                        >"Previous"</button>
                        <button
                            type="button"
                            on:click=move |_| activate(PaginationItemKind::Next)
                            data-rustic-pagination-trigger="next"
                            style="background:#1e293b;color:#60a5fa;border:1px solid rgba(96,165,250,0.4);padding:10px 18px;border-radius:8px;font-weight:600;"
                        >"Next"</button>
                        <button
                            type="button"
                            on:click=move |_| activate(PaginationItemKind::Last)
                            data-rustic-pagination-trigger="last"
                            style="background:#1e293b;color:#60a5fa;border:1px solid rgba(96,165,250,0.4);padding:10px 18px;border-radius:8px;font-weight:600;"
                        >"Last"</button>
                    </div>
                </section>
                <section>
                    <h2 style="font-size:1.25rem;margin:0 0 12px 0;">{"Rendered pagination"}</h2>
                    <div
                        id=HYDRATION_CONTAINER_ID
                        data-rustic-pagination="navigation.pagination"
                        style="background:#0f172a;border-radius:16px;padding:24px;display:flex;justify-content:center;"
                        inner_html=move || markup.get().clone()
                    />
                </section>
                <section>
                    <h2 style="font-size:1.25rem;margin:0 0 12px 0;">{"Telemetry stream"}</h2>
                    <pre
                        data-rustic-analytics-log
                        style="background:#020617;border-radius:12px;padding:16px;font-size:0.85rem;line-height:1.5;overflow:auto;max-height:220px;"
                    >{telemetry_lines}</pre>
                </section>
            </main>
        </div>
    }
}
