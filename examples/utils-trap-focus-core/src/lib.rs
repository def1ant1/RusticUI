//! Shared focus trap snapshot used by the multi-framework demos.
//!
//! Enterprise overlays need deterministic focus trapping across SSR and
//! hydration so QA monitors can assert that keyboard navigation never escapes
//! modal dialogs.  This crate centralises the configuration used by the
//! framework-specific examples.  The helpers emit automation-friendly markup
//! and preconfigured [`FocusTrapState`] instances so every adapter can render
//! identical sentinels without duplicating business logic.

use rustic_ui_headless::focus_trap::FocusTrapState;
use rustic_ui_material::focus_trap::{
    render_focus_trap_sentinel_html, FocusTrapSentinelKind, FocusTrapSentinelOptions,
};
use rustic_ui_styled_engine::Theme;
use rustic_ui_system::theme::ColorScheme;
use rustic_ui_system::theme_provider::material_css_baseline_from_theme;

/// Describes the deterministic focus trap used by each framework.
#[derive(Clone, Debug)]
pub struct TrapFocusStory {
    /// Automation prefix mirrored across `data-automation-id` hooks.
    pub automation_prefix: String,
    /// Analytics identifier mirrored to the sentinel nodes for telemetry.
    pub analytics_tag: String,
    /// DOM id applied to the modal surface so focus tracking can reference it.
    pub container_id: String,
    /// Heading identifier referenced by `aria-labelledby`.
    pub title_id: String,
    /// Body copy identifier referenced by `aria-describedby`.
    pub description_id: String,
    /// Identifier of the dismiss button inside the focus trap.
    pub dismiss_button_id: String,
    /// Identifier of the primary action inside the focus trap.
    pub primary_button_id: String,
    /// Options applied to the sentinel renderers.
    pub sentinel_options: FocusTrapSentinelOptions,
    /// Fallback automation prefix for adapters that omit overrides.
    pub fallback_prefix: String,
    /// Configured focus trap state used by every framework harness.
    pub focus_state: FocusTrapState,
    /// SSR markup representing the sentinels plus modal surface.
    pub ssr_markup: String,
    /// Theme overrides mirrored into CSS baselines and hydration wrappers.
    pub theme: Theme,
}

impl TrapFocusStory {
    /// Re-render the start sentinel as HTML.
    pub fn start_sentinel_html(&self) -> String {
        render_focus_trap_sentinel_html(
            &self.focus_state,
            FocusTrapSentinelKind::Start,
            &self.sentinel_options,
            &self.fallback_prefix,
        )
    }

    /// Re-render the end sentinel as HTML.
    pub fn end_sentinel_html(&self) -> String {
        render_focus_trap_sentinel_html(
            &self.focus_state,
            FocusTrapSentinelKind::End,
            &self.sentinel_options,
            &self.fallback_prefix,
        )
    }

    /// Return a clone of the configured focus trap state for hydrators.
    pub fn cloned_state(&self) -> FocusTrapState {
        self.focus_state.clone()
    }

    /// Emit a standalone HTML document containing the SSR snapshot.
    pub fn ssr_document(&self) -> String {
        let baseline = material_css_baseline_from_theme(&self.theme);
        format!(
            "<!doctype html>\n<html lang=\"en\">\n  <head>\n    <meta charset=\"utf-8\" />\n    <title>Focus trap SSR snapshot</title>\n    <style>{baseline}</style>\n  </head>\n  <body data-automation-id=\"{prefix}::ssr-root\">\n    <div id=\"app\">{body}</div>\n    <!-- Hydration harnesses mount into #app and reuse the same focus trap state. -->\n  </body>\n</html>\n",
            baseline = baseline,
            prefix = self.automation_prefix,
            body = self.ssr_markup
        )
    }
}

/// Generate the canonical focus trap story used by the utilities examples.
pub fn enterprise_story() -> TrapFocusStory {
    let automation_prefix = "support-dialog".to_string();
    let analytics_tag = format!("{automation_prefix}::focus");
    let fallback_prefix = automation_prefix.clone();
    let container_id = format!("{automation_prefix}-surface");
    let title_id = format!("{automation_prefix}-title");
    let description_id = format!("{automation_prefix}-body");
    let dismiss_button_id = format!("{automation_prefix}-dismiss");
    let primary_button_id = format!("{automation_prefix}-escalate");

    let mut focus_state = FocusTrapState::new(true);
    focus_state.set_focusables([dismiss_button_id.clone(), primary_button_id.clone()]);
    focus_state.set_analytics_tag(Some(&analytics_tag));

    let sentinel_options = FocusTrapSentinelOptions {
        automation_prefix: Some(automation_prefix.clone()),
    };

    let start = render_focus_trap_sentinel_html(
        &focus_state,
        FocusTrapSentinelKind::Start,
        &sentinel_options,
        &fallback_prefix,
    );
    let end = render_focus_trap_sentinel_html(
        &focus_state,
        FocusTrapSentinelKind::End,
        &sentinel_options,
        &fallback_prefix,
    );

    let ssr_markup = format!(
        concat!(
            "{start}\n",
            "<section ",
            "data-automation-id=\"{prefix}::surface\" ",
            "data-focus-trap=\"active\" ",
            "role=\"dialog\" ",
            "aria-modal=\"true\" ",
            "aria-labelledby=\"{title_id}\" ",
            "aria-describedby=\"{description_id}\" ",
            "id=\"{container_id}\"\n>",
            "  <header data-automation-id=\"{prefix}::header\">\n",
            "    <h2 id=\"{title_id}\">Incident response</h2>\n",
            "  </header>\n",
            "  <p id=\"{description_id}\" data-automation-id=\"{prefix}::body-copy\">",
            "Keyboard focus remains inside this container until operators resolve or dismiss the incident.",
            "</p>\n",
            "  <div role=\"group\" aria-label=\"Incident actions\" data-automation-id=\"{prefix}::actions\">\n",
            "    <button id=\"{dismiss_id}\" data-automation-id=\"{prefix}::action-dismiss\" type=\"button\">",
            "Close incident",
            "</button>\n",
            "    <button id=\"{primary_id}\" data-automation-id=\"{prefix}::action-escalate\" type=\"button\">",
            "Escalate to secondary",
            "</button>\n",
            "  </div>\n",
            "</section>\n",
            "{end}\n"
        ),
        start = start,
        end = end,
        prefix = automation_prefix,
        container_id = container_id,
        title_id = title_id,
        description_id = description_id,
        dismiss_id = dismiss_button_id,
        primary_id = primary_button_id,
    );

    let mut theme = Theme::default();
    for scheme in [ColorScheme::Light, ColorScheme::Dark] {
        let palette = theme.palette.scheme_mut(scheme);
        palette.primary = "#0F766E".into();
        palette.secondary = "#F97316".into();
    }

    TrapFocusStory {
        automation_prefix,
        analytics_tag,
        container_id,
        title_id,
        description_id,
        dismiss_button_id,
        primary_button_id,
        sentinel_options,
        fallback_prefix,
        focus_state,
        ssr_markup,
        theme,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ssr_markup_contains_focus_trap_hooks() {
        let story = enterprise_story();
        assert!(
            story
                .ssr_markup
                .contains("data-rustic-focus-trap=\"sentinel-start\""),
            "start sentinel missing"
        );
        assert!(
            story
                .ssr_markup
                .contains("data-rustic-focus-trap=\"sentinel-end\""),
            "end sentinel missing"
        );
        assert!(
            story.ssr_markup.contains(&format!(
                "data-automation-id=\"{}::surface\"",
                story.automation_prefix
            )),
            "surface automation hook missing"
        );
    }
}
