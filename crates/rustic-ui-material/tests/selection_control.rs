use std::sync::Once;

use rustic_ui_material::selection_control::{
    register_radio_group_hook, register_radio_option_hook, register_selection_control_hook,
    RadioGroupAttributes, RadioOptionAttributes, SelectionControlAttributes,
    SelectionControlDescriptor, SelectionControlStateAdapter, SelectionControlTelemetry,
    SelectionControlThemeTokens,
};
use rustic_ui_material::TelemetryHooks;
use rustic_ui_styled_engine::Style;

fn style() -> Style {
    Style::new(rustic_ui_styled_engine::css!("color: inherit;")).expect("valid style")
}

fn ensure_hooks_registered() {
    static INIT: Once = Once::new();
    INIT.call_once(|| {
        register_selection_control_hook(|builder| {
            let updated = builder.clone().class("enterprise-toggle");
            *builder = updated;
        })
        .expect("selection control hook");
        register_radio_option_hook(|builder| {
            let updated = builder.clone().automation_id("global", "radio-option");
            *builder = updated;
        })
        .expect("radio option hook");
        register_radio_group_hook(|builder| {
            let updated = builder.clone().class("enterprise-group");
            *builder = updated;
        })
        .expect("radio group hook");
    });
}

#[test]
fn selection_control_builder_separates_maps() {
    ensure_hooks_registered();
    let descriptor = SelectionControlAttributes::builder("Notifications", style())
        .aria("role", "switch")
        .data("tracking", "enabled")
        .automation_id("qa", "notifications-toggle")
        .attribute("tabindex", "0")
        .build();

    assert_eq!(descriptor.label(), "Notifications");
    assert!(descriptor
        .classes()
        .contains(&"enterprise-toggle".to_string()));
    assert_eq!(
        descriptor.aria_map().get("role"),
        Some(&"switch".to_string())
    );
    assert_eq!(
        descriptor.data_map().get("data-tracking"),
        Some(&"enabled".to_string())
    );
    assert_eq!(
        descriptor.automation_ids().get("qa").map(String::as_str),
        Some("notifications-toggle")
    );
    assert_eq!(
        descriptor.extra_attributes().get("tabindex"),
        Some(&"0".to_string())
    );
}

#[test]
fn selection_control_ssr_string_includes_themed_attributes() {
    ensure_hooks_registered();
    let descriptor = SelectionControlAttributes::builder("Airplane mode", style())
        .class("custom-toggle")
        .aria("aria-checked", "true")
        .build();

    let html = descriptor.to_ssr_html();
    assert!(html.contains("Airplane mode"));
    assert!(html.contains("aria-checked=\"true\""));
    assert!(html.contains("class=\"custom-toggle enterprise-toggle"));
}

#[test]
fn radio_group_hydration_and_ssr_paths_match() {
    ensure_hooks_registered();
    let option_style = style();
    let option = RadioOptionAttributes::builder("Scheduled", option_style.clone())
        .class("option")
        .aria("aria-checked", "false")
        .data("segment", "scheduled")
        .build();
    let group = RadioGroupAttributes::builder(style())
        .aria("role", "radiogroup")
        .automation_id("qa", "notifications-group")
        .option(option)
        .build();

    let hydrated = group.themed_attributes();
    let html = group.to_ssr_html();

    assert!(hydrated.iter().any(|(k, _)| k == "class"));
    assert!(html.contains("enterprise-group"));
    assert!(html.contains("data-automation-qa=\"notifications-group\""));
    assert!(html.contains("radio-option"));
}

#[test]
fn display_trait_round_trips_to_ssr_html() {
    ensure_hooks_registered();
    let descriptor = SelectionControlAttributes::builder("Wi-Fi", style()).build();
    assert_eq!(descriptor.to_ssr_html(), descriptor.to_string());
}

struct DeterministicState;

impl SelectionControlStateAdapter for DeterministicState {
    fn snapshot_attributes(&self) -> Vec<(&'static str, String)> {
        vec![
            ("aria-checked", "false".into()),
            ("data-rustic-analytics-id", "analytics-toggle".into()),
            ("data-rustic-extra", "telemetry".into()),
            ("role", "switch".into()),
        ]
    }
}

#[test]
fn descriptor_merges_telemetry_and_remains_deterministic() {
    ensure_hooks_registered();
    let mut hooks = TelemetryHooks::default();
    hooks.analytics_id = Some("analytics-toggle".into());
    hooks.automation_id = Some("automation-toggle".into());

    let telemetry = SelectionControlTelemetry::new(hooks)
        .with_data_keys("data-rustic-analytics-id", "data-rustic-automation-id")
        .enforce_defaults();

    let theme = SelectionControlThemeTokens::material_defaults()
        .with_class("ssr-token")
        .with_data("priority", "p1")
        .with_attribute("tabindex", "0");

    let descriptor = SelectionControlDescriptor::from_headless(
        "Deterministic toggle",
        style(),
        &DeterministicState,
        &theme,
        &telemetry,
    )
    .expect("descriptor construction succeeds");

    let (attributes, resolved) = descriptor.into_parts();
    assert_eq!(resolved.effective_analytics_id(), Some("analytics-toggle"));
    assert_eq!(
        resolved.effective_automation_id(),
        Some("automation-toggle")
    );

    assert_eq!(
        attributes
            .data_map()
            .get("data-rustic-analytics-id")
            .map(String::as_str),
        Some("analytics-toggle"),
    );
    assert_eq!(
        attributes
            .data_map()
            .get("data-rustic-extra")
            .map(String::as_str),
        Some("telemetry"),
    );
    assert_eq!(
        attributes
            .extra_attributes()
            .get("role")
            .map(String::as_str),
        Some("switch"),
    );

    let first = attributes.to_ssr_html();
    let second = attributes.to_ssr_html();
    assert_eq!(first, second);
    assert!(first.contains("data-rustic-analytics-id=\"analytics-toggle\""));
    assert!(first.contains("data-rustic-automation-id=\"automation-toggle\""));
    assert!(first.contains("data-rustic-extra=\"telemetry\""));

    let classes = attributes.classes();
    assert!(classes.contains(&"ssr-token".to_string()));
    assert!(classes.contains(&"rustic-selection-control".to_string()));
    assert!(classes.contains(&"enterprise-toggle".to_string()));
    assert_eq!(resolved.effective_analytics_id(), Some("analytics-toggle"));
    assert_eq!(
        resolved.effective_automation_id(),
        Some("automation-toggle")
    );
}
