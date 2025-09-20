//! Shared tooltip example used by the feedback blueprints.
//!
//! The helpers in this crate generate deterministic HTML for every supported
//! framework so SSR pipelines, hydration bootstraps, and QA automation can share
//! the exact same markup. Consumers typically call [`enterprise_story`] inside
//! their build scripts or bootstrap binaries and then pipe the returned markup
//! into templating engines or framework specific renderers.

use std::collections::BTreeMap;

use mui_headless::tooltip::{TooltipConfig, TooltipState};
use mui_material::tooltip::{dioxus, leptos, sycamore, yew, TooltipProps};
use mui_styled_engine::Theme;

/// Aggregated tooltip story including markup for each framework and the
/// automation identifier driving portal ids.
#[derive(Debug, Clone)]
pub struct TooltipStory {
    /// Automation identifier propagated into `data-*` hooks and DOM ids.
    pub automation_id: String,
    /// Server-rendered markup for each supported framework adapter.
    pub markup: BTreeMap<&'static str, String>,
    /// Themed palette/typography overrides that should wrap hydration roots.
    pub theme: Theme,
}

/// Render a tooltip configured with enterprise automation hooks.
///
/// The returned story provides SSR-ready markup for Yew, Leptos, Dioxus, and
/// Sycamore adapters while sharing the same [`TooltipState`]. Automation ids are
/// embedded into the markup so QA suites can hydrate the document without
/// rebuilding selectors per framework.
pub fn enterprise_story() -> TooltipStory {
    let mut state = TooltipState::new(TooltipConfig::enterprise_defaults());
    state.focus_anchor();
    state.poll();

    let automation_id = "feedback-tooltip".to_string();
    let props = TooltipProps::new("?", "Escalation SLA guidance")
        .with_automation_id(&automation_id)
        .with_surface_labelled_by("sla-tooltip-heading")
        .with_trigger_haspopup("dialog");

    let mut markup = BTreeMap::new();
    markup.insert("yew", yew::render(&props, &state));
    markup.insert("leptos", leptos::render(&props, &state));
    markup.insert("dioxus", dioxus::render(&props, &state));
    markup.insert("sycamore", sycamore::render(&props, &state));

    TooltipStory {
        automation_id,
        markup,
        theme: enterprise_theme(),
    }
}

fn enterprise_theme() -> Theme {
    let mut theme = Theme::default();
    theme.palette.primary = "#0057B7".into();
    theme.palette.secondary = "#F59E0B".into();
    theme.typography.caption = 0.8125;
    theme
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn markup_contains_automation_and_portal_hooks() {
        let story = enterprise_story();
        assert_eq!(story.markup.len(), 4);
        for (framework, html) in &story.markup {
            assert!(
                html.contains("data-automation-id=\"feedback-tooltip\""),
                "missing automation id for {framework}: {html}"
            );
            assert!(
                html.contains("data-portal-root"),
                "missing portal metadata for {framework}: {html}"
            );
        }
    }
}
