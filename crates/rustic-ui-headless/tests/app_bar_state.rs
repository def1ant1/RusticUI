//! Integration coverage for the headless [`AppBarState`].
//!
//! The assertions mirror how renderer adapters integrate with the state: HTML
//! helpers surface automation identifiers while SVG helpers expose
//! `aria-labelledby` relationships for inline branding.

use rustic_ui_headless::app_bar::{AppBarAnalytics, AppBarColor, AppBarSize, AppBarState};

#[test]
fn builder_sets_core_metadata() {
    let analytics = AppBarAnalytics::default()
        .with_view_id("nav.view")
        .with_interaction_id("nav.interaction");
    let state = AppBarState::new("Console")
        .with_aria_label("Primary navigation")
        .with_color(AppBarColor::Secondary)
        .with_size(AppBarSize::Large)
        .with_automation_id("console.app-bar")
        .with_analytics(analytics.clone())
        .with_svg_title_id("console.branding");

    assert_eq!(state.title(), "Console");
    assert_eq!(state.aria_label(), "Primary navigation");
    assert_eq!(state.color(), AppBarColor::Secondary);
    assert_eq!(state.size(), AppBarSize::Large);
    assert_eq!(state.automation_id(), Some("console.app-bar"));
    assert_eq!(state.analytics(), &analytics);
}

#[test]
fn html_attributes_surface_automation_hooks() {
    let attrs = AppBarState::new("Console")
        .with_aria_label("Primary navigation")
        .with_automation_id("console.app-bar")
        .with_analytics(AppBarAnalytics::default().with_view_id("nav.view"))
        .html_attributes();

    assert!(attrs.iter().any(|(k, v)| k == &"role" && v == "banner"));
    assert!(attrs
        .iter()
        .any(|(k, v)| k == &"data-automation-id" && v == "console.app-bar"));
    assert!(attrs
        .iter()
        .any(|(k, v)| k == &"data-analytics-view-id" && v == "nav.view"));
}

#[test]
fn svg_helpers_support_linked_titles() {
    let state = AppBarState::new("Console").with_svg_title_id("console.branding");
    let svg_attrs = state.svg_attributes();
    assert!(svg_attrs
        .iter()
        .any(|(k, v)| k == &"aria-labelledby" && v == "console.branding"));
    let title_attrs = state.svg_title_attributes().expect("title attrs");
    assert_eq!(title_attrs, vec![("id", "console.branding".to_string())]);
}
