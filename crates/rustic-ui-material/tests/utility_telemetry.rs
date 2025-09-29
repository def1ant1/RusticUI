#![cfg(feature = "dioxus")]

use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc, Mutex,
};

use rustic_ui_headless::{click_away::ClickAwayState, collapsible_region::CollapsibleRegionState};
use rustic_ui_material::{
    click_away, collapsible, dialog::DialogSurfaceOptions, TelemetryContext, TelemetryHooks,
};

fn record_contexts(
    target: Arc<Mutex<Vec<TelemetryContext>>>,
) -> Arc<dyn Fn(TelemetryContext) + Send + Sync> {
    Arc::new(move |ctx: TelemetryContext| {
        target.lock().unwrap().push(ctx);
    })
}

#[test]
fn click_away_dioxus_adapter_emits_dialog_telemetry() {
    let mut surface = DialogSurfaceOptions::default();
    surface.analytics_id = Some("checkout-flow".into());
    let fallback = click_away::dialog_click_away_automation(&surface);

    let contexts = Arc::new(Mutex::new(Vec::new()));
    let errors = Arc::new(AtomicUsize::new(0));
    let mut telemetry = TelemetryHooks::default();
    telemetry.analytics_id = Some("override-analytics".into());
    telemetry.on_render = Some(record_contexts(Arc::clone(&contexts)));
    telemetry.on_error = Some(Arc::new({
        let errors = Arc::clone(&errors);
        move |_, _| {
            errors.fetch_add(1, Ordering::Relaxed);
        }
    }));

    let html = click_away::dioxus::render(&click_away::dioxus::ClickAwayBoundaryProps {
        state: ClickAwayState::new(),
        options: click_away::ClickAwayBoundaryOptions::default(),
        automation_fallback: fallback.clone(),
        children: "<section></section>".into(),
        telemetry,
    });

    assert!(html.contains("data-rustic-click-away=\"root\""));
    assert!(html.contains("data-automation-id"));
    assert!(html.contains(&fallback));
    let stored = contexts.lock().unwrap();
    assert_eq!(stored.len(), 1);
    let ctx = &stored[0];
    assert_eq!(
        ctx.component,
        "rustic_ui_material::click_away::dioxus::ClickAwayBoundary"
    );
    assert_eq!(ctx.analytics_id.as_deref(), Some("override-analytics"));
    assert_eq!(ctx.automation_id.as_deref(), Some(fallback.as_str()));
    assert_eq!(errors.load(Ordering::Relaxed), 0);
}

#[test]
fn collapsible_dioxus_adapters_emit_menu_telemetry() {
    let trigger_contexts = Arc::new(Mutex::new(Vec::new()));
    let region_contexts = Arc::new(Mutex::new(Vec::new()));

    let mut trigger_hooks = TelemetryHooks::default();
    trigger_hooks.analytics_id = Some("menu-analytics".into());
    trigger_hooks.on_render = Some(record_contexts(Arc::clone(&trigger_contexts)));

    let mut region_hooks = TelemetryHooks::default();
    region_hooks.analytics_id = Some("menu-analytics".into());
    region_hooks.on_render = Some(record_contexts(Arc::clone(&region_contexts)));

    let trigger_html =
        collapsible::dioxus::render_trigger(&collapsible::dioxus::CollapsibleTriggerProps {
            state: CollapsibleRegionState::uncontrolled(false),
            options: collapsible::CollapsibleTriggerOptions::default(),
            automation_fallback: "menu::trigger::primary".into(),
            children: "<span>Trigger</span>".into(),
            telemetry: trigger_hooks,
        });
    assert!(trigger_html.contains("data-automation-id=\"menu::trigger::primary\""));
    assert!(trigger_html.contains("data-rustic-analytics-id=\"menu-analytics\""));

    let region_html =
        collapsible::dioxus::render_region(&collapsible::dioxus::CollapsibleRegionProps {
            state: CollapsibleRegionState::uncontrolled(false),
            options: collapsible::CollapsibleRegionOptions::default(),
            automation_fallback: "menu::region::primary".into(),
            children: "<div>Region</div>".into(),
            telemetry: region_hooks,
        });
    assert!(region_html.contains("data-automation-id=\"menu::region::primary\""));
    assert!(region_html.contains("data-rustic-analytics-id=\"menu-analytics\""));

    let trigger_events = trigger_contexts.lock().unwrap();
    assert_eq!(trigger_events.len(), 1);
    assert_eq!(
        trigger_events[0].component,
        "rustic_ui_material::collapsible::dioxus::CollapsibleTrigger"
    );
    assert_eq!(
        trigger_events[0].automation_id.as_deref(),
        Some("menu::trigger::primary")
    );

    let region_events = region_contexts.lock().unwrap();
    assert_eq!(region_events.len(), 1);
    assert_eq!(
        region_events[0].component,
        "rustic_ui_material::collapsible::dioxus::CollapsibleRegion"
    );
    assert_eq!(
        region_events[0].automation_id.as_deref(),
        Some("menu::region::primary")
    );
}
