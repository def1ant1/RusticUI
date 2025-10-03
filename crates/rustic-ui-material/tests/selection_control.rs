use std::sync::Once;

use rustic_ui_material::selection_control::{
    register_radio_group_hook, register_radio_option_hook, register_selection_control_hook,
    RadioGroupAttributes, RadioOptionAttributes, SelectionControlAttributes,
};
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
