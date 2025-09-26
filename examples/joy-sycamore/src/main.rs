use std::{cell::RefCell, rc::Rc};

use joy_workflows_core::{JoyWorkflowEvent, JoyWorkflowMachine, SnackbarSeverity};
use rustic_ui_headless::stepper::StepStatus;
use rustic_ui_joy::helpers::resolve_surface_tokens;
use rustic_ui_joy::{Color, Variant};
use rustic_ui_system::theme::Theme;
use sycamore::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;

/// Translate the workflow snackbar severity into Joy color + variant tokens.
fn snackbar_tokens(severity: &SnackbarSeverity) -> (Color, Variant) {
    match severity {
        SnackbarSeverity::Info => (Color::Neutral, Variant::Soft),
        SnackbarSeverity::Success => (Color::Primary, Variant::Solid),
        SnackbarSeverity::Warning => (Color::Danger, Variant::Soft),
    }
}

/// Root component rendered by the Sycamore example.  The implementation keeps
/// the business logic and design tokens centralised so we only focus on binding
/// signals + events to the Joy workflow machine.  Comments are intentionally
/// verbose to make it obvious how each section contributes to the enterprise
/// workflow demo and to minimise future manual boilerplate.
#[component]
fn App() -> View {
    // The state machine drives the Joy workflow without any framework specific
    // dependencies.  We wrap it in `Rc<RefCell<_>>` so event handlers can mutate
    // it while remaining clonable for memoised closures and signals.
    let machine = Rc::new(RefCell::new(JoyWorkflowMachine::new()));
    let snapshot = create_signal(machine.borrow().snapshot());
    let capacity_profile = create_signal(machine.borrow().capacity_profile().to_string());

    // Clone blueprint + theme once so every closure can reference the shared
    // descriptors without recomputing derived styles.  `Rc` lets Sycamore capture
    // the data inside `'static` closures without juggling lifetimes.
    let blueprint = Rc::new(machine.borrow().blueprint().clone());
    let theme: Rc<Theme> = Rc::new(blueprint.theme.clone());

    // Pre-compute the Joy surface styles that stay constant for the lifetime of
    // the component.  Doing this upfront avoids repetitive recomposition inside
    // the reactive closures.
    let environment_descriptor = Rc::as_ref(&blueprint).environment.clone();
    let approve_descriptor = Rc::as_ref(&blueprint).approve_action.clone();
    let rollback_descriptor = Rc::as_ref(&blueprint).rollback_action.clone();
    let automation_ids = Rc::as_ref(&blueprint).automation.clone();
    let snackbar_descriptor = Rc::as_ref(&blueprint).snackbar.clone();
    let release_title = Rc::as_ref(&blueprint).release_title;
    let release_summary = Rc::as_ref(&blueprint).release_summary;
    let card_id = automation_ids.card_id;
    let environment_chip_id = automation_ids.environment_chip_id;
    let capacity_slider_id = automation_ids.capacity_slider_id;
    let snackbar_id = automation_ids.snackbar_id;

    let environment_style = resolve_surface_tokens(
        theme.as_ref(),
        environment_descriptor.color.clone(),
        environment_descriptor.variant.clone(),
    )
    .compose([
        ("display", "inline-flex".to_string()),
        ("align-items", "center".to_string()),
        ("gap", "8px".to_string()),
        ("padding", "6px 12px".to_string()),
        ("font-weight", "600".to_string()),
    ]);
    let approve_style = resolve_surface_tokens(
        theme.as_ref(),
        approve_descriptor.color.clone(),
        approve_descriptor.variant.clone(),
    )
    .compose([
        ("padding", "10px 18px".to_string()),
        ("border", "none".to_string()),
        ("cursor", "pointer".to_string()),
        ("font-weight", "600".to_string()),
        ("border-radius", "8px".to_string()),
    ]);
    let rollback_style = resolve_surface_tokens(
        theme.as_ref(),
        rollback_descriptor.color.clone(),
        rollback_descriptor.variant.clone(),
    )
    .compose([
        ("padding", "10px 18px".to_string()),
        ("border", "none".to_string()),
        ("cursor", "pointer".to_string()),
        ("font-weight", "600".to_string()),
        ("border-radius", "8px".to_string()),
    ]);

    // Shared strings reused across the view tree.
    let card_shell = "max-width:960px;margin:48px auto;box-shadow:0 30px 70px rgba(15,23,42,0.35);border-radius:12px;padding:24px;display:flex;flex-direction:column;gap:16px;background:#0b1120;color:#e2e8f0;";
    let slider_min = Rc::as_ref(&blueprint).capacity.min.to_string();
    let slider_max = Rc::as_ref(&blueprint).capacity.max.to_string();
    let slider_step = Rc::as_ref(&blueprint).capacity.step.to_string();

    // List data copied locally to avoid re-borrowing inside reactive closures.
    let metrics = Rc::as_ref(&blueprint).metrics.clone();
    let steps = Rc::as_ref(&blueprint).steps.clone();
    let marks = Rc::as_ref(&blueprint).capacity.marks.clone();

    // Helper that advances the workflow and refreshes all reactive signals.
    let on_advance = {
        let machine = Rc::clone(&machine);
        let snapshot = snapshot.clone();
        let capacity_profile = capacity_profile.clone();
        Rc::new(move || {
            let next = {
                let mut machine = machine.borrow_mut();
                let next = machine.apply(JoyWorkflowEvent::Advance);
                capacity_profile.set(machine.capacity_profile().to_string());
                next
            };
            snapshot.set(next);
        })
    };

    // Helper that rolls back the workflow.
    let on_rollback = {
        let machine = Rc::clone(&machine);
        let snapshot = snapshot.clone();
        Rc::new(move || {
            let next = machine.borrow_mut().apply(JoyWorkflowEvent::Rollback);
            snapshot.set(next);
        })
    };

    // Capacity slider handler wired to the shared machine so the Joy surface
    // stays in sync with headless state transitions.
    let on_capacity = {
        let machine = Rc::clone(&machine);
        let snapshot = snapshot.clone();
        let capacity_profile = capacity_profile.clone();
        Rc::new(move |event: web_sys::Event| {
            if let Some(target) = event.target() {
                if let Ok(input) = target.dyn_into::<HtmlInputElement>() {
                    if let Ok(value) = input.value().parse::<f64>() {
                        let next = {
                            let mut machine = machine.borrow_mut();
                            let next = machine.apply(JoyWorkflowEvent::SetCapacity(value));
                            capacity_profile.set(machine.capacity_profile().to_string());
                            next
                        };
                        snapshot.set(next);
                    }
                }
            }
        })
    };

    // Snackbar dismiss handler reused by the dynamic snackbar view.
    let dismiss_snackbar = {
        let machine = Rc::clone(&machine);
        let snapshot = snapshot.clone();
        Rc::new(move || {
            let next = machine
                .borrow_mut()
                .apply(JoyWorkflowEvent::DismissSnackbar);
            snapshot.set(next);
        })
    };

    view! {
        main(style="min-height:100vh;background:#0f172a;padding:32px;box-sizing:border-box;font-family:'Inter',sans-serif;color:#e2e8f0;") {
            section(style=card_shell, data-analytics-id=card_id) {
                header(style="display:flex;flex-direction:column;gap:12px;") {
                    span(style=environment_style.clone(), data-analytics-id=environment_chip_id) {
                        span { (environment_descriptor.label) }
                        span(style="font-weight:400;font-size:0.875rem;") { (environment_descriptor.description) }
                    }
                    div {
                        h1(style="margin:0;font-size:2rem;") { (release_title) }
                        p(style="margin:4px 0 0;max-width:60ch;") { (release_summary) }
                    }
                }

                (View::from({
                    let snapshot = snapshot.clone();
                    let theme = Rc::clone(&theme);
                    let snackbar_descriptor = snackbar_descriptor.clone();
                    let snackbar_id = snackbar_id;
                    let dismiss_snackbar = Rc::clone(&dismiss_snackbar);
                    move || {
                        snapshot.with(|snap| snap.snackbar.clone()).map(|payload| {
                            let (color, variant) = snackbar_tokens(&payload.severity);
                            let surface = resolve_surface_tokens(theme.as_ref(), color, variant).compose([
                                ("padding", "12px 16px".to_string()),
                                ("border-radius", "8px".to_string()),
                                ("display", "flex".to_string()),
                                ("align-items", "center".to_string()),
                                ("justify-content", "space-between".to_string()),
                            ("gap", "16px".to_string()),
                            ]);
                            let dismiss = Rc::clone(&dismiss_snackbar);
                            let success_label = snackbar_descriptor.success_label;
                            view! {
                                div(
                                    style=surface,
                                    data-analytics-id=snackbar_id,
                                    role="status",
                                    aria-live="polite"
                                ) {
                                    span {
                                        (format!(
                                            "{} — {}",
                                            success_label,
                                            payload.message
                                        ))
                                    }
                                    button(
                                        style="background:rgba(15,23,42,0.35);border:none;color:inherit;padding:6px 12px;border-radius:6px;cursor:pointer;",
                                        on:click=move |_| dismiss()
                                    ) { "Dismiss" }
                                }
                            }
                        })
                    }
                }))

                section(style="display:grid;grid-template-columns:repeat(auto-fit,minmax(180px,1fr));gap:12px;") {
                    (View::from(metrics.iter().cloned().map(|metric| {
                        view! {
                            article(style="display:flex;flex-direction:column;gap:4px;") {
                                span(style="font-size:0.75rem;letter-spacing:0.08em;text-transform:uppercase;color:#94a3b8;") { (metric.label) }
                                strong(style="font-size:1.5rem;") { (metric.value) }
                                span(style="font-size:0.9rem;color:#cbd5f5;opacity:0.9;") { (metric.detail) }
                            }
                        }
                    }).collect::<Vec<_>>()))
                }

                section(style="display:flex;flex-direction:column;gap:12px;") {
                    label(r#for="joy-capacity", style="font-weight:600;") { "Deployment capacity" }
                    input(
                        id="joy-capacity",
                        r#type="range",
                        min=slider_min.clone(),
                        max=slider_max.clone(),
                        step=slider_step.clone(),
                        value=move || snapshot.with(|snap| format!("{:.0}", snap.capacity_value)),
                        style=move || snapshot.with(|snap| format!("width:100%;appearance:none;background:linear-gradient(90deg,#38bdf80%,#3b82f6 {:.2}%,rgba(148,163,184,0.3) {:.2}%);height:8px;border-radius:6px;outline:none;", snap.capacity_percent, snap.capacity_percent)),
                        data-analytics-id=capacity_slider_id,
                        on:input={
                            let handler = Rc::clone(&on_capacity);
                            move |event: web_sys::Event| handler(event)
                        }
                    )
                    div(style="display:flex;justify-content:space-between;font-size:0.85rem;color:#94a3b8;") {
                        (View::from(marks.iter().cloned().map(|mark| {
                            view! { span { (format!("{}% — {}", mark.value, mark.label)) } }
                        }).collect::<Vec<_>>()))
                    }
                    p(style="margin:0;font-weight:500;") {
                        (move || {
                            let profile = capacity_profile.get_clone();
                            snapshot.with(|snap| {
                                format!(
                                    "Current allocation: {:.0}% ({})",
                                    snap.capacity_value,
                                    profile
                                )
                            })
                        })
                    }
                }

                section(style="display:flex;flex-direction:column;gap:12px;") {
                    h2(style="margin:0;font-size:1.25rem;") { "Release checklist" }
                    ol(style="margin:0;padding-left:18px;display:flex;flex-direction:column;gap:8px;") {
                        (View::from({
                            let snapshot = snapshot.clone();
                            let steps = steps.clone();
                            move || {
                                let snapshot = snapshot.get_clone();
                                steps
                                    .iter()
                                    .enumerate()
                                    .map(|(index, step)| {
                                        let status_label = snapshot
                                            .step_status
                                            .get(index)
                                            .cloned()
                                            .map(|status| match status {
                                                StepStatus::Completed => "Completed",
                                                StepStatus::Active => "In progress",
                                                StepStatus::Pending => "Pending",
                                                StepStatus::Disabled => "Pending",
                                            })
                                            .unwrap_or("Pending");
                                        view! {
                                            li {
                                                div(style="display:flex;flex-direction:column;gap:4px;") {
                                                    div(style="display:flex;align-items:center;gap:8px;") {
                                                        span(style="font-weight:600;") { (step.title) }
                                                        span(style="font-size:0.8rem;color:#94a3b8;") { (status_label) }
                                                    }
                                                    p(style="margin:0;color:#cbd5f5;") { (step.detail) }
                                                }
                                            }
                                        }
                                    })
                                    .collect::<Vec<_>>()
                            }
                        }))
                    }
                }

                section(style="display:flex;flex-wrap:wrap;gap:12px;align-items:flex-start;") {
                    div(data-analytics-id=approve_descriptor.analytics_id, style="display:flex;flex-direction:column;gap:6px;max-width:28ch;") {
                        button(
                            style=approve_style.clone(),
                            disabled=move || snapshot.with(|snap| snap.completed),
                            on:click={
                                let handler = Rc::clone(&on_advance);
                                move |_| handler()
                            }
                        ) { (approve_descriptor.label) }
                        span(style="font-size:0.85rem;color:#94a3b8;") { (approve_descriptor.description) }
                    }
                    div(data-analytics-id=rollback_descriptor.analytics_id, style="display:flex;flex-direction:column;gap:6px;max-width:28ch;") {
                        button(
                            style=rollback_style.clone(),
                            on:click={
                                let handler = Rc::clone(&on_rollback);
                                move |_| handler()
                            }
                        ) { (rollback_descriptor.label) }
                        span(style="font-size:0.85rem;color:#94a3b8;") { (rollback_descriptor.description) }
                    }
                }

                section(style="display:flex;flex-direction:column;gap:8px;") {
                    h3(style="margin:0;font-size:1rem;") { "Lifecycle journal" }
                    ul(style="margin:0;padding-left:18px;display:flex;flex-direction:column;gap:4px;") {
                        (View::from({
                            let snapshot = snapshot.clone();
                            move || {
                                snapshot
                                    .get_clone()
                                    .lifecycle_log
                                    .iter()
                                    .cloned()
                                    .map(|entry| view! { li { (entry) } })
                                    .collect::<Vec<_>>()
                            }
                        }))
                    }
                }
            }
        }
    }
}

#[cfg(feature = "csr")]
#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn main() {
    sycamore::render(|| view! { App {} });
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
