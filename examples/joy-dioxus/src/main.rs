use dioxus::events::FormEvent;
use dioxus::prelude::*;
use joy_workflows_core::{JoyWorkflowEvent, JoyWorkflowMachine, SnackbarSeverity};
use mui_headless::stepper::StepStatus;
use mui_joy::helpers::resolve_surface_tokens;
use mui_joy::{Color, Variant};
use mui_system::theme::Theme;

fn snackbar_tokens(severity: &SnackbarSeverity) -> (Color, Variant) {
    match severity {
        SnackbarSeverity::Info => (Color::Neutral, Variant::Soft),
        SnackbarSeverity::Success => (Color::Primary, Variant::Solid),
        SnackbarSeverity::Warning => (Color::Danger, Variant::Soft),
    }
}

fn App(cx: Scope) -> Element {
    let machine = use_ref(cx, JoyWorkflowMachine::new);
    let snapshot = use_state(cx, || machine.read().snapshot());
    let capacity_profile = use_state(cx, || machine.read().capacity_profile().to_string());
    let blueprint = machine.read().blueprint().clone();
    let theme: Theme = blueprint.theme.clone();

    let environment_style = resolve_surface_tokens(
        &theme,
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
        &theme,
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
        &theme,
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

    let on_advance = {
        let machine = machine.clone();
        let snapshot = snapshot.clone();
        let capacity_profile = capacity_profile.clone();
        move |_| {
            let next = {
                let mut machine = machine.write_silent();
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
                let mut machine = machine.write_silent();
                let next = machine.apply(JoyWorkflowEvent::Rollback);
                capacity_profile.set(machine.capacity_profile().to_string());
                next
            };
            snapshot.set(next);
        }
    };

    let on_capacity = {
        let machine = machine.clone();
        let snapshot = snapshot.clone();
        let capacity_profile = capacity_profile.clone();
        move |ev: FormEvent| {
            if let Ok(value) = ev.value.parse::<f64>() {
                let next = {
                    let mut machine = machine.write_silent();
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
        move |_| {
            let next = {
                let mut machine = machine.write_silent();
                machine.apply(JoyWorkflowEvent::DismissSnackbar)
            };
            snapshot.set(next);
        }
    };

    let card_shell = "max-width:960px;margin:48px auto;box-shadow:0 30px 70px rgba(15,23,42,0.35);border-radius:12px;padding:24px;display:flex;flex-direction:column;gap:16px;background:#0b1120;color:#e2e8f0;";

    let snackbar_view = snapshot
        .get()
        .snackbar
        .as_ref()
        .map(|payload| {
            let (color, variant) = snackbar_tokens(&payload.severity);
            let style = resolve_surface_tokens(&theme, color, variant).compose([
                ("padding", "12px 16px".to_string()),
                ("border-radius", "8px".to_string()),
                ("display", "flex".to_string()),
                ("align-items", "center".to_string()),
                ("justify-content", "space-between".to_string()),
                ("gap", "16px".to_string()),
            ]);
            rsx! {
                div {
                    style: "{style}",
                    "data-analytics-id": "{blueprint.automation.snackbar_id}",
                    role: "status",
                    "aria-live": "polite",
                    span {{format!("{} — {}", blueprint.snackbar.success_label, payload.message)}}
                    button {
                        style: "background:rgba(15,23,42,0.35);border:none;color:inherit;padding:6px 12px;border-radius:6px;cursor:pointer;",
                        onclick: move |_| dismiss_snackbar(()),
                        "Dismiss"
                    }
                }
            }
        });

    let metrics = blueprint.metrics.clone();
    let steps = blueprint.steps.clone();
    let marks = blueprint.capacity.marks.clone();

    cx.render(rsx! {
        main {
            style: "min-height:100vh;background:#0f172a;padding:32px;box-sizing:border-box;font-family:'Inter',sans-serif;color:#e2e8f0;",
            section {
                style: "{card_shell}",
                "data-analytics-id": "{blueprint.automation.card_id}",
                header {
                    style: "display:flex;flex-direction:column;gap:12px;",
                    span {
                        style: "{environment_style}",
                        "data-analytics-id": "{blueprint.automation.environment_chip_id}",
                        span {{blueprint.environment.label}}
                        span { style: "font-weight:400;font-size:0.875rem;", {blueprint.environment.description} }
                    }
                    div {
                        h1 { style: "margin:0;font-size:2rem;", {blueprint.release_title} }
                        p { style: "margin:4px 0 0;max-width:60ch;", {blueprint.release_summary} }
                    }
                }

                {snackbar_view}

                section {
                    style: "display:grid;grid-template-columns:repeat(auto-fit,minmax(180px,1fr));gap:12px;",
                    {metrics.iter().map(|metric| rsx! {
                        article {
                            style: "display:flex;flex-direction:column;gap:4px;",
                            span { style: "font-size:0.75rem;letter-spacing:0.08em;text-transform:uppercase;color:#94a3b8;", {metric.label} }
                            strong { style: "font-size:1.5rem;", {metric.value} }
                            span { style: "font-size:0.9rem;color:#cbd5f5;opacity:0.9;", {metric.detail} }
                        }
                    })}
                }

                section {
                    style: "display:flex;flex-direction:column;gap:12px;",
                    label { r#for: "joy-capacity", style: "font-weight:600;", "Deployment capacity" }
                    input {
                        id: "joy-capacity",
                        r#type: "range",
                        min: "{blueprint.capacity.min}",
                        max: "{blueprint.capacity.max}",
                        step: "{blueprint.capacity.step}",
                        value: format_args!("{:.0}", snapshot.get().capacity_value),
                        style: format_args!(
                            "width:100%;appearance:none;background:linear-gradient(90deg,#38bdf8 0%,#3b82f6 {:.2}%,rgba(148,163,184,0.3) {:.2}%);height:8px;border-radius:6px;outline:none;",
                            snapshot.get().capacity_percent,
                            snapshot.get().capacity_percent
                        ),
                        "data-analytics-id": "{blueprint.automation.capacity_slider_id}",
                        oninput: on_capacity.clone()
                    }
                    div {
                        style: "display:flex;justify-content:space-between;font-size:0.85rem;color:#94a3b8;",
                        {marks.iter().map(|mark| rsx! { span {{format!("{}% — {}", mark.value, mark.label)}} })}
                    }
                    p {
                        style: "margin:0;font-weight:500;",
                        {format!("Current allocation: {:.0}% ({})", snapshot.get().capacity_value, capacity_profile.get())}
                    }
                }

                section {
                    style: "display:flex;flex-direction:column;gap:12px;",
                    h2 { style: "margin:0;font-size:1.25rem;", "Release checklist" }
                    ol {
                        style: "margin:0;padding-left:18px;display:flex;flex-direction:column;gap:8px;",
                        {steps.iter().enumerate().map(|(index, step)| {
                            let status = snapshot
                                .get()
                                .step_status
                                .get(index)
                                .cloned()
                                .unwrap_or(StepStatus::Pending);
                            let status_label = match status {
                                StepStatus::Completed => "Completed",
                                StepStatus::Active => "In progress",
                                _ => "Pending",
                            };
                            rsx! {
                                li {
                                    div { style: "display:flex;flex-direction:column;gap:4px;",
                                        div { style: "display:flex;align-items:center;gap:8px;",
                                            span { style: "font-weight:600;", {step.title} }
                                            span { style: "font-size:0.8rem;color:#94a3b8;", {status_label} }
                                        }
                                        p { style: "margin:0;color:#cbd5f5;", {step.detail} }
                                    }
                                }
                            }
                        })}
                    }
                }

                section {
                    style: "display:flex;flex-wrap:wrap;gap:12px;align-items:flex-start;",
                    div {
                        "data-analytics-id": "{blueprint.approve_action.analytics_id}",
                        style: "display:flex;flex-direction:column;gap:6px;max-width:28ch;",
                        button { style: "{approve_style}", onclick: on_advance.clone(), disabled: snapshot.get().completed, {blueprint.approve_action.label} }
                        span { style: "font-size:0.85rem;color:#94a3b8;", {blueprint.approve_action.description} }
                    }
                    div {
                        "data-analytics-id": "{blueprint.rollback_action.analytics_id}",
                        style: "display:flex;flex-direction:column;gap:6px;max-width:28ch;",
                        button { style: "{rollback_style}", onclick: on_rollback.clone(), {blueprint.rollback_action.label} }
                        span { style: "font-size:0.85rem;color:#94a3b8;", {blueprint.rollback_action.description} }
                    }
                }

                section {
                    style: "display:flex;flex-direction:column;gap:8px;",
                    h3 { style: "margin:0;font-size:1rem;", "Lifecycle journal" }
                    ul {
                        style: "margin:0;padding-left:18px;display:flex;flex-direction:column;gap:4px;",
                        {snapshot
                            .get()
                            .lifecycle_log
                            .iter()
                            .map(|entry| rsx! { li {{entry.clone()}} })}
                    }
                }
            }
        }
    })
}

#[cfg(feature = "csr")]
fn main() {
    dioxus_web::launch(App);
}

#[cfg(feature = "ssr")]
fn main() {
    let snapshot = JoyWorkflowMachine::new().snapshot();
    println!(
        "Joy workflow SSR snapshot: step {:?}, capacity {:.0}%",
        snapshot.active_step_label, snapshot.capacity_value
    );
}
