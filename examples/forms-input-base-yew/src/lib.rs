//! Yew implementation of the shared InputBase blueprint.
//!
//! The component keeps business logic extremely small by delegating analytics
//! metadata, SSR markup, and automation namespaces to the
//! `forms-input-base-shared` crate.  This mirrors how enterprise teams should
//! structure production repositories: centralise deterministic infrastructure
//! pieces and let framework shells focus on wiring events.

use forms_input_base_shared::{
    automation_value, InputBaseBlueprint, CONTROLLED_ANALYTICS_ID, CONTROLLED_STATUS_ID,
    HYDRATION_NOTE, PLACEHOLDER, UNCONTROLLED_ANALYTICS_ID, UNCONTROLLED_STATUS_ID,
};
use rustic_ui_headless::input_base::{InputChangeEvent, InputCommitEvent, InputResetEvent};
use rustic_ui_material::input_base::yew::InputBase;
use rustic_ui_material::input_base::{
    InputBaseColor, InputBaseSize, InputBaseStateHandle, InputBaseVariant,
};
use rustic_ui_styled_engine::{Theme, ThemeProvider};
use yew::prelude::*;

/// Convenience helper that clones and appends to a `UseStateHandle<Vec<String>>`.
fn push_log(target: &UseStateHandle<Vec<String>>, line: String) {
    target.set({
        let mut next = (*target).clone();
        next.push(line);
        next
    });
}

/// Escape newlines for display inside `<pre>` blocks.
fn normalise_log(entries: &[String]) -> String {
    entries.join("\n")
}

/// Root Yew component rendered by the example binaries/tests.
#[function_component(App)]
pub fn app() -> Html {
    let blueprint = InputBaseBlueprint::new();
    let theme: Theme = blueprint.enterprise_theme();
    let automation_attributes = blueprint.automation_attributes();

    let uncontrolled_state =
        use_state(|| InputBaseStateHandle::from(blueprint.uncontrolled_state()));
    let controlled_state = use_state(|| InputBaseStateHandle::from(blueprint.controlled_state()));
    let controlled_value = use_state(|| "ops@rusticui.dev".to_string());
    let analytics_log = use_state(Vec::<String>::new);

    let automation_hint = automation_value(["example", "yew"]);

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
        let controlled_value = controlled_value.clone();
        let analytics_log = analytics_log.clone();
        let state_handle = (*controlled_state).clone();
        Callback::from(move |event: InputChangeEvent| {
            push_log(
                &analytics_log,
                format!(
                    "change → value='{}' dirty={} selection={:?} analytics={:?}",
                    event.value, event.dirty, event.selection, event.analytics
                ),
            );
            controlled_value.set(event.value.clone());
            {
                let mut guard = state_handle.borrow_mut();
                guard.sync_controlled_value(event.value);
            }
        })
    };

    let on_controlled_commit = {
        let analytics_log = analytics_log.clone();
        Callback::from(move |event: InputCommitEvent| {
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
        let state_handle = (*controlled_state).clone();
        Callback::from(move |event: InputResetEvent| {
            push_log(
                &analytics_log,
                format!(
                    "reset → value='{}' cleared_errors={} analytics={:?}",
                    event.value, event.cleared_errors, event.analytics
                ),
            );
            {
                let mut guard = state_handle.borrow_mut();
                guard.sync_controlled_value(event.value);
            }
        })
    };

    let on_uncontrolled_commit = {
        let analytics_log = analytics_log.clone();
        Callback::from(move |event: InputCommitEvent| {
            push_log(
                &analytics_log,
                format!(
                    "uncontrolled commit → value='{}' has_errors={} analytics={:?}",
                    event.value, event.has_errors, event.analytics
                ),
            );
        })
    };

    html! {
        <ThemeProvider theme={theme}>
            <section
                style="min-height:100vh;padding:48px;box-sizing:border-box;background:#020617;display:flex;justify-content:center;align-items:flex-start;"
                data-rustic-input-base-example={automation_hint}
            >
                <article
                    style="width:100%;max-width:920px;background:#0f172a;color:#e2e8f0;padding:40px;border-radius:16px;box-shadow:0 24px 64px rgba(15,23,42,0.45);display:flex;flex-direction:column;gap:24px;"
                >
                    <header>
                        <h1 style="margin:0;font-size:2rem;">{"RusticUI InputBase – Yew"}</h1>
                        <p style="margin:8px 0 0;max-width:72ch;">
                            {"Controlled and uncontrolled flows reuse the same InputState instrumentation so SSR snapshots and hydration analytics stay perfectly aligned."}
                        </p>
                        <p style="margin:12px 0 0;font-size:0.95rem;color:#cbd5f5;">{HYDRATION_NOTE}</p>
                    </header>

                    <section style="display:grid;grid-template-columns:repeat(auto-fit, minmax(280px, 1fr));gap:24px;">
                        <div style="display:flex;flex-direction:column;gap:12px;">
                            <h2 style="margin:0;">{"Controlled input"}</h2>
                            <InputBase
                                state={(*controlled_state).clone()}
                                placeholder={PLACEHOLDER.into()}
                                aria_label={"Primary contact email".into()}
                                input_type={"email".into()}
                                analytics_id={Some(CONTROLLED_ANALYTICS_ID.into())}
                                status_id={Some(CONTROLLED_STATUS_ID.into())}
                                color={InputBaseColor::Primary}
                                variant={InputBaseVariant::Outlined}
                                size={InputBaseSize::Medium}
                                on_change={Some(on_controlled_change)}
                                on_commit={Some(on_controlled_commit)}
                                on_reset={Some(on_controlled_reset)}
                            />
                            <p style="margin:0;font-size:0.9rem;color:#cbd5f5;">{"Current value (controlled parent state): "}{(*controlled_value).clone()}</p>
                        </div>
                        <div style="display:flex;flex-direction:column;gap:12px;">
                            <h2 style="margin:0;">{"Uncontrolled input"}</h2>
                            <InputBase
                                state={(*uncontrolled_state).clone()}
                                placeholder={PLACEHOLDER.into()}
                                aria_label={"Primary contact email".into()}
                                input_type={"email".into()}
                                analytics_id={Some(UNCONTROLLED_ANALYTICS_ID.into())}
                                status_id={Some(UNCONTROLLED_STATUS_ID.into())}
                                color={InputBaseColor::Secondary}
                                variant={InputBaseVariant::Outlined}
                                size={InputBaseSize::Medium}
                                on_commit={Some(on_uncontrolled_commit)}
                            />
                            <p style="margin:0;font-size:0.9rem;color:#cbd5f5;">{"Validation errors are preloaded so SSR snapshots expose data-status-message hooks."}</p>
                        </div>
                    </section>

                    <section style="display:grid;grid-template-columns:repeat(auto-fit, minmax(260px, 1fr));gap:24px;">
                        <div>
                            <h3 style="margin-top:0;">{"Automation attributes"}</h3>
                            <ul>
                                { for automation_attributes.iter().map(|attr| html!{ <li><code>{*attr}</code></li> }) }
                            </ul>
                        </div>
                        <div>
                            <h3 style="margin-top:0;">{"SSR markup (uncontrolled)"}</h3>
                            <pre style="background:#020617;padding:12px;border-radius:8px;white-space:pre-wrap;">{uncontrolled_html.clone()}</pre>
                        </div>
                        <div>
                            <h3 style="margin-top:0;">{"SSR markup (controlled)"}</h3>
                            <pre style="background:#020617;padding:12px;border-radius:8px;white-space:pre-wrap;">{controlled_html.clone()}</pre>
                        </div>
                    </section>

                    <section>
                        <h3 style="margin-top:0;">{"Analytics log"}</h3>
                        <pre style="background:#020617;padding:12px;border-radius:8px;min-height:120px;white-space:pre-wrap;">
                            {normalise_log(&*analytics_log)}
                        </pre>
                    </section>
                </article>
            </section>
        </ThemeProvider>
    }
}

/// Headless entry used by `cargo run`.
#[cfg(feature = "csr")]
pub fn main() {
    yew::Renderer::<App>::new().render();
}
