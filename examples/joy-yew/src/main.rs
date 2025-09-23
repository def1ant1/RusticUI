use joy_workflows_core::{
    JoyWorkflowEvent, JoyWorkflowMachine, JoyWorkflowSnapshot, SnackbarSeverity,
};
use mui_joy::helpers::resolve_surface_tokens;
use mui_joy::{Button, ButtonProps, Card, Color, Variant};
use mui_system::theme_provider::{CssBaseline, ThemeProvider};
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

/// Translate the Joy stepper status into a short label for the demo UI.
fn status_label(status: &mui_joy::stepper::StepStatus) -> &'static str {
    use mui_joy::stepper::StepStatus::{Active, Completed, Pending};
    match status {
        Completed => "Completed",
        Active => "In progress",
        Pending => "Pending",
        _ => "Disabled",
    }
}

/// Determine the Joy color/variant pairing to use for snackbar payloads.
fn snackbar_tokens(severity: &SnackbarSeverity) -> (Color, Variant) {
    match severity {
        SnackbarSeverity::Info => (Color::Neutral, Variant::Soft),
        SnackbarSeverity::Success => (Color::Primary, Variant::Solid),
        SnackbarSeverity::Warning => (Color::Danger, Variant::Soft),
    }
}

/// Yew component rendering the shared Joy workflow.
#[function_component(App)]
fn app() -> Html {
    // Centralise the workflow machine in a mutable ref so callbacks can mutate it
    // without forcing full re-renders until the snapshot state is replaced.
    let machine = use_mut_ref(JoyWorkflowMachine::new);
    let snapshot_state = use_state(|| machine.borrow().snapshot());

    // Clone the blueprint once per render. The struct only stores `'static`
    // strings plus a theme clone so this is cheap and keeps the templates tidy.
    let blueprint = { machine.borrow().blueprint().clone() };
    let snapshot: JoyWorkflowSnapshot = (*snapshot_state).clone();
    let capacity_profile = { machine.borrow().capacity_profile() };

    // Primary action: advance the workflow stepper.
    let on_advance = {
        let machine = machine.clone();
        let snapshot_state = snapshot_state.clone();
        Callback::from(move |_| {
            let snapshot = machine.borrow_mut().apply(JoyWorkflowEvent::Advance);
            snapshot_state.set(snapshot);
        })
    };

    // Secondary action: roll the workflow back.
    let on_rollback = {
        let machine = machine.clone();
        let snapshot_state = snapshot_state.clone();
        Callback::from(move |_| {
            let snapshot = machine.borrow_mut().apply(JoyWorkflowEvent::Rollback);
            snapshot_state.set(snapshot);
        })
    };

    // Capacity slider handler keeps the logic inside the shared machine so SSR
    // and hydration reuse the same validation + logging.
    let on_capacity_change = {
        let machine = machine.clone();
        let snapshot_state = snapshot_state.clone();
        Callback::from(move |event: InputEvent| {
            if let Some(input) = event
                .target()
                .and_then(|target| target.dyn_into::<HtmlInputElement>().ok())
            {
                if let Ok(value) = input.value().parse::<f64>() {
                    let snapshot = machine
                        .borrow_mut()
                        .apply(JoyWorkflowEvent::SetCapacity(value));
                    snapshot_state.set(snapshot);
                }
            }
        })
    };

    // Expose a dismiss button for snackbar payloads so automation can verify the
    // analytics hooks even when the message is cleared.
    let dismiss_snackbar = {
        let machine = machine.clone();
        let snapshot_state = snapshot_state.clone();
        Callback::from(move |_| {
            let snapshot = machine
                .borrow_mut()
                .apply(JoyWorkflowEvent::DismissSnackbar);
            snapshot_state.set(snapshot);
        })
    };

    // Style helpers derived from the Joy tokens so the example feels native.
    let card_container_style = "max-width:960px;margin:48px auto;box-shadow:0 30px 70px rgba(15,23,42,0.35);border-radius:12px;";

    let environment_style = resolve_surface_tokens(
        &blueprint.theme,
        blueprint.environment.color.clone(),
        blueprint.environment.variant.clone(),
    )
    .compose([
        ("display", "inline-flex".to_string()),
        ("align-items", "center".to_string()),
        ("gap", "8px".to_string()),
        ("padding", "6px 12px".to_string()),
        ("font-weight", "600".to_string()),
    ]);

    let snackbar_view = snapshot.snackbar.as_ref().map(|payload| {
        let (color, variant) = snackbar_tokens(&payload.severity);
        let style = resolve_surface_tokens(&blueprint.theme, color, variant).compose([
            ("padding", "12px 16px".to_string()),
            ("border-radius", "8px".to_string()),
            ("display", "flex".to_string()),
            ("align-items", "center".to_string()),
            ("justify-content", "space-between".to_string()),
            ("gap", "16px".to_string()),
        ]);
        html! {
            <div
                style={style}
                data-analytics-id={blueprint.automation.snackbar_id}
                role="status"
                aria-live="polite"
            >
                <span>{format!("{} — {}", blueprint.snackbar.success_label, payload.message)}</span>
                <button type="button" onclick={dismiss_snackbar.clone()}>{"Dismiss"}</button>
            </div>
        }
    });

    let approve_button = ButtonProps {
        color: blueprint.approve_action.color.clone(),
        variant: blueprint.approve_action.variant.clone(),
        label: blueprint.approve_action.label.to_string(),
        onclick: on_advance.clone(),
        throttle_ms: blueprint.approve_action.throttle_ms,
        disabled: snapshot.completed,
    };

    let rollback_button = ButtonProps {
        color: blueprint.rollback_action.color.clone(),
        variant: blueprint.rollback_action.variant.clone(),
        label: blueprint.rollback_action.label.to_string(),
        onclick: on_rollback.clone(),
        throttle_ms: blueprint.rollback_action.throttle_ms,
        disabled: snapshot.completed && snapshot.active_step.is_none(),
    };

    html! {
        <ThemeProvider theme={blueprint.theme.clone()}>
            <CssBaseline />
            <main style="min-height:100vh;background:#0f172a;padding:32px;box-sizing:border-box;">
                <div style={card_container_style}>
                    <Card color={Color::Neutral} variant={Variant::Soft}>
                        <div
                            style="display:flex;flex-direction:column;gap:16px;"
                            data-analytics-id={blueprint.automation.card_id}
                        >
                        <header style="display:flex;flex-direction:column;gap:12px;">
                            <div style={environment_style.clone()} data-analytics-id={blueprint.automation.environment_chip_id}>
                                <span>{blueprint.environment.label}</span>
                                <span style="font-weight:400;font-size:0.875rem;">{blueprint.environment.description}</span>
                            </div>
                            <div>
                                <h1 style="margin:0;font-size:2rem;">{blueprint.release_title}</h1>
                                <p style="margin:4px 0 0;max-width:60ch;">{blueprint.release_summary}</p>
                            </div>
                        </header>

                        if let Some(snackbar) = snackbar_view {
                            <section>{snackbar}</section>
                        }

                        <section style="display:grid;grid-template-columns:repeat(auto-fit,minmax(180px,1fr));gap:12px;">
                            { for blueprint.metrics.iter().map(|metric| html! {
                                <article style="display:flex;flex-direction:column;gap:4px;">
                                    <span style="font-size:0.75rem;letter-spacing:0.08em;text-transform:uppercase;color:#94a3b8;">{metric.label}</span>
                                    <strong style="font-size:1.5rem;">{metric.value}</strong>
                                    <span style="font-size:0.9rem;color:#cbd5f5;opacity:0.9;">{metric.detail}</span>
                                </article>
                            }) }
                        </section>

                        <section style="display:flex;flex-direction:column;gap:12px;">
                            <label for="joy-capacity" style="font-weight:600;">{"Deployment capacity"}</label>
                            <input
                                id="joy-capacity"
                                type="range"
                                min={blueprint.capacity.min.to_string()}
                                max={blueprint.capacity.max.to_string()}
                                step={blueprint.capacity.step.to_string()}
                                value={format!("{:.0}", snapshot.capacity_value)}
                                style={format!("width:100%;appearance:none;background:linear-gradient(90deg,#38bdf8 0%,#3b82f6 {:.2}%,rgba(148,163,184,0.3) {:.2}%);
                                    height:8px;border-radius:6px;outline:none;", snapshot.capacity_percent, snapshot.capacity_percent)}
                                oninput={on_capacity_change.clone()}
                                data-analytics-id={blueprint.automation.capacity_slider_id}
                            />
                            <div style="display:flex;justify-content:space-between;font-size:0.85rem;color:#94a3b8;">
                                { for blueprint.capacity.marks.iter().map(|mark| html! {
                                    <span>{format!("{}% — {}", mark.value, mark.label)}</span>
                                }) }
                            </div>
                            <p style="margin:0;font-weight:500;">{format!("Current allocation: {:.0}% ({})", snapshot.capacity_value, capacity_profile)}</p>
                        </section>

                        <section style="display:flex;flex-direction:column;gap:12px;">
                            <h2 style="margin:0;font-size:1.25rem;">{"Release checklist"}</h2>
                            <ol style="margin:0;padding-left:18px;display:flex;flex-direction:column;gap:8px;">
                                { for blueprint.steps.iter().enumerate().map(|(index, step)| {
                                    let status = snapshot.step_status.get(index).cloned().unwrap_or(mui_joy::stepper::StepStatus::Pending);
                                    html! {
                                        <li>
                                            <div style="display:flex;flex-direction:column;gap:4px;">
                                                <div style="display:flex;align-items:center;gap:8px;">
                                                    <span style="font-weight:600;">{step.title}</span>
                                                    <span style="font-size:0.8rem;color:#94a3b8;">{status_label(&status)}</span>
                                                </div>
                                                <p style="margin:0;color:#cbd5f5;">{step.detail}</p>
                                            </div>
                                        </li>
                                    }
                                }) }
                            </ol>
                        </section>

                        <section style="display:flex;flex-wrap:wrap;gap:12px;align-items:flex-start;">
                            <div data-analytics-id={blueprint.approve_action.analytics_id}>
                                <Button ..approve_button.clone() />
                                <p style="margin:4px 0 0;font-size:0.85rem;color:#94a3b8;max-width:28ch;">{blueprint.approve_action.description}</p>
                            </div>
                            <div data-analytics-id={blueprint.rollback_action.analytics_id}>
                                <Button ..rollback_button.clone() />
                                <p style="margin:4px 0 0;font-size:0.85rem;color:#94a3b8;max-width:28ch;">{blueprint.rollback_action.description}</p>
                            </div>
                        </section>

                        <section style="display:flex;flex-direction:column;gap:8px;">
                            <h3 style="margin:0;font-size:1rem;">{"Lifecycle journal"}</h3>
                            <ul style="margin:0;padding-left:18px;display:flex;flex-direction:column;gap:4px;">
                                { for snapshot.lifecycle_log.iter().map(|entry| html! { <li>{entry}</li> }) }
                            </ul>
                        </section>
                        </div>
                    </Card>
                </div>
            </main>
        </ThemeProvider>
    }
}

#[cfg(feature = "csr")]
fn main() {
    yew::Renderer::<App>::new().render();
}

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    let snapshot = JoyWorkflowMachine::new().snapshot();
    println!(
        "Joy workflow SSR snapshot: step {:?}, capacity {:.0}%",
        snapshot.active_step_label, snapshot.capacity_value
    );
}
