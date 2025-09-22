//! Chip feedback blueprint shared by the SSR bootstrapper and hydration stubs.
//!
//! The helpers below keep the dismissible and read-only variants aligned across
//! Yew, Leptos, Dioxus, and Sycamore so analytics pipelines only need to learn a
//! single automation contract.

use std::collections::BTreeMap;

use mui_headless::chip::{ChipConfig, ChipState};
use mui_material::chip::{dioxus, leptos, sycamore, yew, ChipProps};
use mui_styled_engine::Theme;

/// Aggregates multi-framework chip markup for QA automation and SSR bootstraps.
#[derive(Debug, Clone)]
pub struct ChipStory {
    /// Automation identifier for the primary dismissible chip.
    pub automation_id: String,
    /// Dismissible chip markup keyed by framework.
    pub dismissible: BTreeMap<&'static str, String>,
    /// Read-only chip markup keyed by framework.
    pub read_only: BTreeMap<&'static str, String>,
    /// Theme overrides applied during hydration.
    pub theme: Theme,
}

/// Build chip markup for both dismissible and read-only variants.
pub fn enterprise_story() -> ChipStory {
    let automation_id = "feedback-chip".to_string();

    let mut dismissible_state = ChipState::new(ChipConfig::enterprise_defaults());
    dismissible_state.pointer_enter();
    dismissible_state.poll();

    let dismissible_props = ChipProps::new("Escalation")
        .with_automation_id(&automation_id)
        .with_delete_label("remove escalation")
        .with_delete_icon("✕");

    let mut read_only_state = ChipState::new(ChipConfig::enterprise_defaults());
    read_only_state.set_disabled(false);
    read_only_state.pointer_enter();
    read_only_state.poll();

    let read_only_props = ChipProps::new("At capacity")
        .with_automation_id(format!("{automation_id}-static"))
        .with_dismissible(false);

    let mut dismissible = BTreeMap::new();
    dismissible.insert("yew", yew::render(&dismissible_props, &dismissible_state));
    dismissible.insert(
        "leptos",
        leptos::render(&dismissible_props, &dismissible_state),
    );
    dismissible.insert(
        "dioxus",
        dioxus::render(&dismissible_props, &dismissible_state),
    );
    dismissible.insert(
        "sycamore",
        sycamore::render(&dismissible_props, &dismissible_state),
    );

    let mut read_only = BTreeMap::new();
    read_only.insert("yew", yew::render(&read_only_props, &read_only_state));
    read_only.insert("leptos", leptos::render(&read_only_props, &read_only_state));
    read_only.insert("dioxus", dioxus::render(&read_only_props, &read_only_state));
    read_only.insert(
        "sycamore",
        sycamore::render(&read_only_props, &read_only_state),
    );

    ChipStory {
        automation_id,
        dismissible,
        read_only,
        theme: enterprise_theme(),
    }
}

fn enterprise_theme() -> Theme {
    let mut theme = Theme::default();
    for scheme in [
        mui_system::theme::ColorScheme::Light,
        mui_system::theme::ColorScheme::Dark,
    ] {
        let palette = theme.palette.scheme_mut(scheme);
        palette.primary = "#334155".into();
        palette.neutral = "#1E293B".into();
    }
    theme.typography.body2 = 0.875;
    theme
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dismissible_markup_exposes_visibility_metadata() {
        let story = enterprise_story();
        for (framework, html) in &story.dismissible {
            assert!(
                html.contains("data-dismissible=\"true\""),
                "dismissible flag missing for {framework}: {html}"
            );
            assert!(
                html.contains("data-automation-id=\"feedback-chip\""),
                "automation id missing for {framework}: {html}"
            );
        }
        for (framework, html) in &story.read_only {
            assert!(
                html.contains("data-dismissible=\"false\""),
                "read-only flag missing for {framework}: {html}"
            );
        }
    }
}
