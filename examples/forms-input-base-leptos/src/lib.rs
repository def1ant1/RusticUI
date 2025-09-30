//! Leptos implementation of the shared InputBase blueprint.
//!
//! Signals keep the example reactive while the shared crate centralises
//! automation namespaces, analytics identifiers, and SSR markup.  The
//! component mirrors the Yew variant so QA teams can rely on identical
//! `data-rustic-input-base-*` selectors regardless of renderer.

use std::rc::Rc;

use forms_input_base_shared::{
    automation_value, InputBaseBlueprint, CONTROLLED_ANALYTICS_ID, CONTROLLED_STATUS_ID,
    HYDRATION_NOTE, PLACEHOLDER, UNCONTROLLED_ANALYTICS_ID, UNCONTROLLED_STATUS_ID,
};
use leptos::*;
use rustic_ui_headless::input_base::{InputChangeEvent, InputCommitEvent, InputResetEvent};
use rustic_ui_material::input_base::leptos::InputBase;
use rustic_ui_material::input_base::{
    InputBaseColor, InputBaseSize, InputBaseStateHandle, InputBaseVariant,
};
use rustic_ui_system::theme::Theme;

fn push_log(signal: &RwSignal<Vec<String>>, line: String) {
    signal.update(|entries| entries.push(line));
}

fn normalise_log(entries: &[String]) -> String {
    entries.join("\n")
}

#[component]
pub fn App() -> impl IntoView {
    let blueprint = InputBaseBlueprint::new();
    let theme: Theme = blueprint.enterprise_theme();
    let automation_attributes = blueprint.automation_attributes();
    let palette = theme.palette.active().clone();
    let container_style = format!(
        "min-height:100vh;padding:48px;box-sizing:border-box;background:{};display:flex;justify-content:center;align-items:flex-start;",
        palette.background_default
    );
    let panel_style = format!(
        "width:100%;max-width:920px;background:{};color:{};padding:40px;border-radius:16px;box-shadow:0 24px 64px rgba(15,23,42,0.45);display:flex;flex-direction:column;gap:24px;",
        palette.background_paper, palette.text_primary
    );

    let controlled_state =
        create_rw_signal(InputBaseStateHandle::from(blueprint.controlled_state()));
    let uncontrolled_state =
        create_rw_signal(InputBaseStateHandle::from(blueprint.uncontrolled_state()));
    let controlled_value = create_rw_signal("ops@rusticui.dev".to_string());
    let analytics_log = create_rw_signal(Vec::<String>::new());

    let automation_hint = automation_value(["example", "leptos"]);

    let uncontrolled_html = blueprint.render_markup(
        &blueprint.uncontrolled_state(),
        UNCONTROLLED_ANALYTICS_ID,
        UNCONTROLLED_STATUS_ID,
    );
    let controlled_html = blueprint.render_markup(
        &blueprint.controlled_state(),
        CONTROLLED_ANALYTICS_ID,
        CONTROLLED_STATUS_ID,
    );

    let on_controlled_change = {
        let controlled_state = controlled_state.clone();
        let controlled_value = controlled_value.clone();
        let analytics_log = analytics_log.clone();
        Rc::new(move |event: InputChangeEvent| {
            push_log(
                &analytics_log,
                format!(
                    "change → value='{}' dirty={} selection={:?} analytics={:?}",
                    event.value, event.dirty, event.selection, event.analytics
                ),
            );
            controlled_value.set(event.value.clone());
            controlled_state.with(|handle| {
                let mut guard = handle.borrow_mut();
                guard.sync_controlled_value(event.value.clone());
            });
        })
    };

    let on_controlled_commit = {
        let analytics_log = analytics_log.clone();
        Rc::new(move |event: InputCommitEvent| {
            push_log(
                &analytics_log,
                format!(
                    "commit → value='{}' has_errors={} previously_visited={} analytics={:?}",
                    event.value, event.has_errors, event.previously_visited, event.analytics
                ),
            );
        })
    };

    let on_controlled_reset = {
        let analytics_log = analytics_log.clone();
        let controlled_state = controlled_state.clone();
        Rc::new(move |event: InputResetEvent| {
            push_log(
                &analytics_log,
                format!(
                    "reset → value='{}' cleared_errors={} analytics={:?}",
                    event.value, event.cleared_errors, event.analytics
                ),
            );
            controlled_state.with(|handle| {
                let mut guard = handle.borrow_mut();
                guard.sync_controlled_value(event.value.clone());
            });
        })
    };

    let on_uncontrolled_commit = {
        let analytics_log = analytics_log.clone();
        Rc::new(move |event: InputCommitEvent| {
            push_log(
                &analytics_log,
                format!(
                    "uncontrolled commit → value='{}' has_errors={} analytics={:?}",
                    event.value, event.has_errors, event.analytics
                ),
            );
        })
    };

    view! {
        <div
            style=container_style
            data-rustic-input-base-example=automation_hint
        >
            <article style=panel_style>
                <header>
                    <h1 style="margin:0;font-size:2rem;">"RusticUI InputBase – Leptos"</h1>
                    <p style="margin:8px 0 0;max-width:72ch;">
                        {"Signals propagate controlled edits while the shared InputState keeps automation selectors consistent."}
                    </p>
                    <p style="margin:12px 0 0;font-size:0.95rem;color:#cbd5f5;">{HYDRATION_NOTE}</p>
                </header>

                <section style="display:grid;grid-template-columns:repeat(auto-fit, minmax(280px, 1fr));gap:24px;">
                    <div style="display:flex;flex-direction:column;gap:12px;">
                        <h2 style="margin:0;">"Controlled input"</h2>
                        <InputBase
                            state=controlled_state.get()
                            placeholder=PLACEHOLDER
                            aria_label="Primary contact email"
                            input_type="email"
                            analytics_id=CONTROLLED_ANALYTICS_ID
                            status_id=CONTROLLED_STATUS_ID
                            color=InputBaseColor::Primary
                            variant=InputBaseVariant::Outlined
                            size=InputBaseSize::Medium
                            on_change=Some(on_controlled_change)
                            on_commit=Some(on_controlled_commit)
                            on_reset=Some(on_controlled_reset)
                        />
                        <p style="margin:0;font-size:0.9rem;color:#cbd5f5;">{move || format!("Current value: {}", controlled_value.get())}</p>
                    </div>
                    <div style="display:flex;flex-direction:column;gap:12px;">
                        <h2 style="margin:0;">"Uncontrolled input"</h2>
                        <InputBase
                            state=uncontrolled_state.get()
                            placeholder=PLACEHOLDER
                            aria_label="Primary contact email"
                            input_type="email"
                            analytics_id=UNCONTROLLED_ANALYTICS_ID
                            status_id=UNCONTROLLED_STATUS_ID
                            color=InputBaseColor::Secondary
                            variant=InputBaseVariant::Outlined
                            size=InputBaseSize::Medium
                            on_commit=Some(on_uncontrolled_commit)
                        />
                        <p style="margin:0;font-size:0.9rem;color:#cbd5f5;">{"Validation errors are embedded to surface data-status-message hooks."}</p>
                    </div>
                </section>

                <section style="display:grid;grid-template-columns:repeat(auto-fit, minmax(260px, 1fr));gap:24px;">
                    <div>
                        <h3 style="margin-top:0;">"Automation attributes"</h3>
                        <ul>
                            {automation_attributes.into_iter().map(|attr| view! { <li><code>{attr}</code></li> }).collect_view()}
                        </ul>
                    </div>
                    <div>
                        <h3 style="margin-top:0;">"SSR markup (uncontrolled)"</h3>
                        <pre style="background:#020617;padding:12px;border-radius:8px;white-space:pre-wrap;">{uncontrolled_html.clone()}</pre>
                    </div>
                    <div>
                        <h3 style="margin-top:0;">"SSR markup (controlled)"</h3>
                        <pre style="background:#020617;padding:12px;border-radius:8px;white-space:pre-wrap;">{controlled_html.clone()}</pre>
                    </div>
                </section>

                <section>
                    <h3 style="margin-top:0;">"Analytics log"</h3>
                    <pre style="background:#020617;padding:12px;border-radius:8px;min-height:120px;white-space:pre-wrap;">{move || normalise_log(&analytics_log.get())}</pre>
                </section>
            </article>
        </div>
    }
}

#[cfg(feature = "csr")]
pub fn main() {
    leptos::mount_to_body(|| view! { <App/> });
}

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    let blueprint = InputBaseBlueprint::new();
    println!("{}", blueprint.ssr_document());
}
