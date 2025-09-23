use std::{cell::RefCell, rc::Rc};

use joy_workflows_core::{
    JoyWorkflowEvent, JoyWorkflowMachine, JoyWorkflowSnapshot, SnackbarSeverity,
};
use leptos::ev::{Event, MouseEvent};
use leptos::*;
use leptos::{event_target_value, IntoView};
use mui_headless::stepper::StepStatus;
use mui_joy::helpers::resolve_surface_tokens;
use mui_joy::{Color, Variant};

/// Map the shared snackbar severity into Joy color/variant pairs for styling.
fn snackbar_tokens(severity: &SnackbarSeverity) -> (Color, Variant) {
    match severity {
        SnackbarSeverity::Info => (Color::Neutral, Variant::Soft),
        SnackbarSeverity::Success => (Color::Primary, Variant::Solid),
        SnackbarSeverity::Warning => (Color::Danger, Variant::Soft),
    }
}

/// Render the shared Joy workflow using Leptos signals.
#[component]
pub fn App() -> impl IntoView {
    let machine = Rc::new(RefCell::new(JoyWorkflowMachine::new()));
    let snapshot = create_rw_signal(machine.borrow().snapshot());
    let capacity_profile = create_rw_signal(machine.borrow().capacity_profile().to_string());

    let blueprint = machine.borrow().blueprint().clone();
    let theme = Rc::new(blueprint.theme.clone());
    let snackbar_theme = store_value(theme.clone());

    // Callbacks centralise mutations inside the shared machine so every adapter
    // reuses the exact same logic.
    let on_advance = {
        let machine = machine.clone();
        let snapshot = snapshot.clone();
        let capacity_profile = capacity_profile.clone();
        move |_| {
            let next = {
                let mut machine = machine.borrow_mut();
                let next = machine.apply(JoyWorkflowEvent::Advance);
                capacity_profile.set(machine.capacity_profile().to_string());
                next
            };
            snapshot.set(next);
        }
    };

    let on_rollback = {
        let machine = machine.clone();
        let snapshot = snapshot.clone();
        let capacity_profile = capacity_profile.clone();
        move |_| {
            let next = {
                let mut machine = machine.borrow_mut();
                let next = machine.apply(JoyWorkflowEvent::Rollback);
                capacity_profile.set(machine.capacity_profile().to_string());
                next
            };
            snapshot.set(next);
        }
    };

    let on_capacity_change = {
        let machine = machine.clone();
        let snapshot = snapshot.clone();
        let capacity_profile = capacity_profile.clone();
        move |ev: Event| {
            if let Ok(value) = event_target_value(&ev).parse::<f64>() {
                let next = {
                    let mut machine = machine.borrow_mut();
                    let next = machine.apply(JoyWorkflowEvent::SetCapacity(value));
                    capacity_profile.set(machine.capacity_profile().to_string());
                    next
                };
                snapshot.set(next);
            }
        }
    };

    let dismiss_snackbar = {
        let machine = machine.clone();
        let snapshot = snapshot.clone();
        store_value(move |_: MouseEvent| {
            let next = {
                let mut machine = machine.borrow_mut();
                machine.apply(JoyWorkflowEvent::DismissSnackbar)
            };
            snapshot.set(next);
        })
    };

    let environment_style = resolve_surface_tokens(
        &*theme,
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

    let approve_style = resolve_surface_tokens(
        &*theme,
        blueprint.approve_action.color.clone(),
        blueprint.approve_action.variant.clone(),
    )
    .compose([
        ("padding", "10px 18px".to_string()),
        ("border", "none".to_string()),
        ("cursor", "pointer".to_string()),
        ("font-weight", "600".to_string()),
        ("border-radius", "8px".to_string()),
    ]);

    let rollback_style = resolve_surface_tokens(
        &*theme,
        blueprint.rollback_action.color.clone(),
        blueprint.rollback_action.variant.clone(),
    )
    .compose([
        ("padding", "10px 18px".to_string()),
        ("border", "none".to_string()),
        ("cursor", "pointer".to_string()),
        ("font-weight", "600".to_string()),
        ("border-radius", "8px".to_string()),
    ]);

    let card_shell = "max-width:960px;margin:48px auto;box-shadow:0 30px 70px rgba(15,23,42,0.35);border-radius:12px;padding:24px;display:flex;flex-direction:column;gap:16px;background:#0b1120;color:#e2e8f0;";

    view! {
        <main style="min-height:100vh;background:#0f172a;padding:32px;box-sizing:border-box;font-family:'Inter',sans-serif;color:#e2e8f0;">
            <section style=card_shell data-analytics-id={blueprint.automation.card_id}>
                <header style="display:flex;flex-direction:column;gap:12px;">
                    <span style={environment_style} data-analytics-id={blueprint.automation.environment_chip_id}>
                        <span>{blueprint.environment.label}</span>
                        <span style="font-weight:400;font-size:0.875rem;">{blueprint.environment.description}</span>
                    </span>
                    <div>
                        <h1 style="margin:0;font-size:2rem;">{blueprint.release_title}</h1>
                        <p style="margin:4px 0 0;max-width:60ch;">{blueprint.release_summary}</p>
                    </div>
                </header>

                <Show when=move || snapshot.get().snackbar.is_some() fallback=|| view! { <></> }>
                    {move || {
                        let snapshot: JoyWorkflowSnapshot = snapshot.get();
                        match snapshot.snackbar {
                            Some(payload) => {
                                let (color, variant) = snackbar_tokens(&payload.severity);
                                let style = snackbar_theme.with_value(|theme| {
                                    resolve_surface_tokens(
                                        theme.as_ref(),
                                        color.clone(),
                                        variant.clone(),
                                    )
                                    .compose([
                                        ("padding", "12px 16px".to_string()),
                                        ("border-radius", "8px".to_string()),
                                        ("display", "flex".to_string()),
                                        ("align-items", "center".to_string()),
                                        ("justify-content", "space-between".to_string()),
                                        ("gap", "16px".to_string()),
                                    ])
                                });
                                view! {
                                    <div
                                        style=style
                                        data-analytics-id={blueprint.automation.snackbar_id}
                                        role="status"
                                        aria-live="polite"
                                    >
                                        <span>{format!("{} — {}", blueprint.snackbar.success_label, payload.message)}</span>
                                        <button
                                            style="background:rgba(15,23,42,0.35);border:none;color:inherit;padding:6px 12px;border-radius:6px;cursor:pointer;"
                                            on:click=move |event| dismiss_snackbar.with_value(|handler| handler(event))
                                        >{"Dismiss"}</button>
                                    </div>
                                }
                            }
                            None => view! { <div style="display:none;"></div> },
                        }
                    }}
                </Show>

                <section style="display:grid;grid-template-columns:repeat(auto-fit,minmax(180px,1fr));gap:12px;">
                    {blueprint
                        .metrics
                        .iter()
                        .map(|metric| {
                            view! {
                                <article style="display:flex;flex-direction:column;gap:4px;">
                                    <span style="font-size:0.75rem;letter-spacing:0.08em;text-transform:uppercase;color:#94a3b8;">{metric.label}</span>
                                    <strong style="font-size:1.5rem;">{metric.value}</strong>
                                    <span style="font-size:0.9rem;color:#cbd5f5;opacity:0.9;">{metric.detail}</span>
                                </article>
                            }
                        })
                        .collect::<Vec<_>>()}
                </section>

                <section style="display:flex;flex-direction:column;gap:12px;">
                    <label for="joy-capacity" style="font-weight:600;">{"Deployment capacity"}</label>
                    <input
                        id="joy-capacity"
                        type="range"
                        min={blueprint.capacity.min}
                        max={blueprint.capacity.max}
                        step={blueprint.capacity.step}
                        prop:value=move || format!("{:.0}", snapshot.get().capacity_value)
                        style=move || format!(
                            "width:100%;appearance:none;background:linear-gradient(90deg,#38bdf8 0%,#3b82f6 {:.2}%,rgba(148,163,184,0.3) {:.2}%);height:8px;border-radius:6px;outline:none;",
                            snapshot.get().capacity_percent,
                            snapshot.get().capacity_percent
                        )
                        data-analytics-id={blueprint.automation.capacity_slider_id}
                        on:input=on_capacity_change.clone()
                    />
                    <div style="display:flex;justify-content:space-between;font-size:0.85rem;color:#94a3b8;">
                        {blueprint
                            .capacity
                            .marks
                            .iter()
                            .map(|mark| view! { <span>{format!("{}% — {}", mark.value, mark.label)}</span> })
                            .collect::<Vec<_>>()}
                    </div>
                    <p style="margin:0;font-weight:500;">{move || format!("Current allocation: {:.0}% ({})", snapshot.get().capacity_value, capacity_profile.get())}</p>
                </section>

                <section style="display:flex;flex-direction:column;gap:12px;">
                    <h2 style="margin:0;font-size:1.25rem;">{"Release checklist"}</h2>
                    <ol style="margin:0;padding-left:18px;display:flex;flex-direction:column;gap:8px;">
                        {blueprint
                            .steps
                            .iter()
                            .enumerate()
                            .map(|(index, step)| {
                                view! {
                                    <li>
                                        <div style="display:flex;flex-direction:column;gap:4px;">
                                            <div style="display:flex;align-items:center;gap:8px;">
                                                <span style="font-weight:600;">{step.title}</span>
                                            <span style="font-size:0.8rem;color:#94a3b8;">{move || {
                                                snapshot
                                                    .get()
                                                    .step_status
                                                    .get(index)
                                                    .map(|status| match status {
                                                        StepStatus::Completed => "Completed",
                                                        StepStatus::Active => "In progress",
                                                        _ => "Pending",
                                                    })
                                                    .unwrap_or("Pending")
                                                    .to_string()
                                            }}</span>
                                            </div>
                                            <p style="margin:0;color:#cbd5f5;">{step.detail}</p>
                                        </div>
                                    </li>
                                }
                            })
                            .collect::<Vec<_>>()}
                    </ol>
                </section>

                <section style="display:flex;flex-wrap:wrap;gap:12px;align-items:flex-start;">
                    <div data-analytics-id={blueprint.approve_action.analytics_id} style="display:flex;flex-direction:column;gap:6px;max-width:28ch;">
                        <button style={approve_style.clone()} on:click=on_advance.clone() disabled=move || snapshot.get().completed>{blueprint.approve_action.label}</button>
                        <span style="font-size:0.85rem;color:#94a3b8;">{blueprint.approve_action.description}</span>
                    </div>
                    <div data-analytics-id={blueprint.rollback_action.analytics_id} style="display:flex;flex-direction:column;gap:6px;max-width:28ch;">
                        <button style={rollback_style.clone()} on:click=on_rollback.clone()>{blueprint.rollback_action.label}</button>
                        <span style="font-size:0.85rem;color:#94a3b8;">{blueprint.rollback_action.description}</span>
                    </div>
                </section>

                <section style="display:flex;flex-direction:column;gap:8px;">
                    <h3 style="margin:0;font-size:1rem;">{"Lifecycle journal"}</h3>
                    <ul style="margin:0;padding-left:18px;display:flex;flex-direction:column;gap:4px;">
                        <For
                            each=move || snapshot.get().lifecycle_log.clone()
                            key=|entry| entry.clone()
                            children=|entry: String| view! { <li>{entry}</li> }
                        />
                    </ul>
                </section>
            </section>
        </main>
    }
}

#[cfg(feature = "csr")]
#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
    leptos::mount_to_body(App);
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
