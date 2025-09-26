//! Data display avatar blueprint combining chips and tooltips.
//!
//! The helper orchestrates a read-only chip with an interactive tooltip so teams
//! can surface presence state and secondary context (like on-call status) without
//! duplicating markup across frameworks.

use std::collections::BTreeMap;

use rustic_ui_headless::chip::{ChipConfig, ChipState};
use rustic_ui_headless::tooltip::{TooltipConfig, TooltipState};
use rustic_ui_material::chip::{
    dioxus as chip_dioxus, leptos as chip_leptos, sycamore as chip_sycamore, yew as chip_yew,
    ChipProps,
};
use rustic_ui_material::tooltip::{
    dioxus as tooltip_dioxus, leptos as tooltip_leptos, sycamore as tooltip_sycamore,
    yew as tooltip_yew, TooltipProps,
};
use rustic_ui_styled_engine::Theme;

/// Combined avatar story output.
#[derive(Debug, Clone)]
pub struct AvatarStory {
    /// Automation id applied to the wrapper for analytics and QA hooks.
    pub automation_id: String,
    /// Framework-specific markup combining the chip and tooltip renderers.
    pub markup: BTreeMap<&'static str, String>,
    /// Theme overrides to apply during hydration.
    pub theme: Theme,
}

/// Render the avatar blueprint for every supported framework.
pub fn enterprise_story() -> AvatarStory {
    let automation_id = "avatar-alex".to_string();

    let mut chip_state = ChipState::new(ChipConfig::enterprise_defaults());
    chip_state.set_disabled(false);

    let chip_props = ChipProps::new("Alex Rivers")
        .with_automation_id(&automation_id)
        .with_dismissible(false);

    let mut tooltip_state = TooltipState::new(TooltipConfig::enterprise_defaults());
    tooltip_state.focus_anchor();
    tooltip_state.poll();

    let tooltip_props = TooltipProps::new("Availability", "Primary on-call • responds < 5 min")
        .with_automation_id(format!("{automation_id}-tooltip"))
        .with_surface_labelled_by("avatar-availability")
        .with_trigger_haspopup("dialog");

    let mut markup = BTreeMap::new();
    markup.insert(
        "yew",
        wrap_markup(
            &automation_id,
            &chip_yew::render(&chip_props, &chip_state),
            &tooltip_yew::render(&tooltip_props, &tooltip_state),
        ),
    );
    markup.insert(
        "leptos",
        wrap_markup(
            &automation_id,
            &chip_leptos::render(&chip_props, &chip_state),
            &tooltip_leptos::render(&tooltip_props, &tooltip_state),
        ),
    );
    markup.insert(
        "dioxus",
        wrap_markup(
            &automation_id,
            &chip_dioxus::render(&chip_props, &chip_state),
            &tooltip_dioxus::render(&tooltip_props, &tooltip_state),
        ),
    );
    markup.insert(
        "sycamore",
        wrap_markup(
            &automation_id,
            &chip_sycamore::render(&chip_props, &chip_state),
            &tooltip_sycamore::render(&tooltip_props, &tooltip_state),
        ),
    );

    AvatarStory {
        automation_id,
        markup,
        theme: enterprise_theme(),
    }
}

fn wrap_markup(automation_id: &str, chip_html: &str, tooltip_html: &str) -> String {
    format!(
        "<article class=\"avatar-card\" data-automation-avatar=\"{automation_id}\">\n  {chip}\n  <div class=\"avatar-presence\">{tooltip}</div>\n</article>",
        automation_id = automation_id,
        chip = chip_html,
        tooltip = tooltip_html
    )
}

fn enterprise_theme() -> Theme {
    let mut theme = Theme::default();
    for scheme in [
        rustic_ui_system::theme::ColorScheme::Light,
        rustic_ui_system::theme::ColorScheme::Dark,
    ] {
        let palette = theme.palette.scheme_mut(scheme);
        palette.background_paper = "#0F172A".into();
        palette.text_primary = "#E2E8F0".into();
    }
    theme.typography.body1 = 0.9375;
    theme
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn avatar_markup_exposes_wrapper_automation() {
        let story = enterprise_story();
        assert_eq!(story.markup.len(), 4);
        for (framework, html) in &story.markup {
            assert!(
                html.contains("data-automation-avatar=\"avatar-alex\""),
                "wrapper automation hook missing for {framework}: {html}"
            );
            assert!(
                html.contains("data-automation-id=\"avatar-alex\""),
                "chip automation id missing for {framework}: {html}"
            );
            assert!(
                html.contains("data-automation-id=\"avatar-alex-tooltip\""),
                "tooltip automation id missing for {framework}: {html}"
            );
        }
    }
}
