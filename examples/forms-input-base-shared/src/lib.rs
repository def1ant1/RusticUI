//! Shared automation, SSR, and analytics helpers for the InputBase form blueprints.
//!
//! The goal of this crate is to minimise repetitive glue across the
//! `forms-input-base-*` examples.  Every framework specific crate can focus on
//! wiring signals/events while this module keeps the analytics namespace,
//! automation attributes, and SSR snapshots aligned.  When a new attribute is
//! introduced in the Material renderer we update the helpers here and every
//! example inherits the change automatically.

use std::fmt::Write as _;

use rustic_ui_headless::input_base::{InputSelection, InputState};
use rustic_ui_material::input_base::{
    render_input_base_html, InputBaseColor, InputBaseRenderConfig, InputBaseSize, InputBaseVariant,
};
use rustic_ui_system::theme::{ColorScheme, Theme};

/// Stable automation namespace applied to every DOM node.
pub const AUTOMATION_NAMESPACE: &str = "rustic-input-base";
/// Analytics identifier used by the controlled example.
pub const CONTROLLED_ANALYTICS_ID: &str = "rusticui.forms.input-base.controlled";
/// Analytics identifier used by the uncontrolled example.
pub const UNCONTROLLED_ANALYTICS_ID: &str = "rusticui.forms.input-base.uncontrolled";
/// Accessibility identifier for the controlled status message.
pub const CONTROLLED_STATUS_ID: &str = "ssr-status-input-base-controlled";
/// Accessibility identifier for the uncontrolled status message.
pub const UNCONTROLLED_STATUS_ID: &str = "ssr-status-input-base-uncontrolled";
/// Placeholder shared across renderers.
pub const PLACEHOLDER: &str = "you@example.com";
/// ARIA label shared across renderers.
pub const ARIA_LABEL: &str = "Primary contact email";
/// Narrative shown inside bootstrap metadata to highlight hydration behaviour.
pub const HYDRATION_NOTE: &str = "Hydrates the shared InputState and replays analytics so data-rustic-input-base-* markers stay in sync.";

/// Blueprint configuration describing the shared story.
#[derive(Clone, Debug, Default)]
pub struct InputBaseBlueprint;

impl InputBaseBlueprint {
    /// Construct the canonical story used by every framework specific example.
    pub fn new() -> Self {
        Self
    }

    /// Enterprise flavoured theme used across the demos.
    pub fn enterprise_theme(&self) -> Theme {
        let mut theme = Theme::default();
        for scheme in [ColorScheme::Light, ColorScheme::Dark] {
            let palette = theme.palette.scheme_mut(scheme);
            palette.primary = "#1d4ed8".into();
            palette.secondary = "#db2777".into();
            palette.background_default = "#0f172a".into();
            palette.background_paper = "#111c3a".into();
            palette.text_primary = "#e2e8f0".into();
            palette.text_secondary = "#cbd5f5".into();
        }
        theme.palette.initial_color_scheme = ColorScheme::Dark;
        theme.typography.font_family = "'IBM Plex Sans', 'Segoe UI', sans-serif".into();
        theme.joy.radius = 8;
        theme
    }

    /// Attributes rendered by the Material InputBase component that QA suites rely on.
    pub fn automation_attributes(&self) -> Vec<&'static str> {
        vec![
            "data-component",
            "data-rustic-input-base-dirty",
            "data-rustic-input-base-visited",
            "data-rustic-input-base-focused",
            "data-rustic-input-base-error-count",
            "data-rustic-input-base-selection-start",
            "data-rustic-input-base-selection-end",
            "data-rustic-input-base-status-message",
            "data-status-message",
            "data-rustic-input-base-analytics-id",
            "data-analytics-id",
        ]
    }

    /// Construct an uncontrolled state with validation errors to showcase analytics hooks.
    pub fn uncontrolled_state(&self) -> InputState {
        let mut state = InputState::uncontrolled("", Some(InputSelection::new(0, 0)));
        state.set_errors([
            "Email address is required".to_string(),
            "Use the corporate domain".to_string(),
        ]);
        state
    }

    /// Construct a controlled state seeded with a placeholder e-mail.
    pub fn controlled_state(&self) -> InputState {
        InputState::controlled("ops@rusticui.dev", Some(InputSelection::new(0, 3)))
    }

    /// Render deterministic HTML for the provided state snapshot.
    pub fn render_markup(&self, state: &InputState, analytics_id: &str, status_id: &str) -> String {
        let mut config = InputBaseRenderConfig::new(state);
        config.placeholder = PLACEHOLDER;
        config.aria_label = ARIA_LABEL;
        config.input_type = "email";
        config.analytics_id = Some(analytics_id);
        config.status_id = Some(status_id);
        config.color = InputBaseColor::Primary;
        config.variant = InputBaseVariant::Outlined;
        config.size = InputBaseSize::Medium;
        config.style_overrides = Some("max-width:420px;margin:0;width:100%;");
        render_input_base_html(&config)
    }

    /// Render a full SSR document embedding both the controlled and uncontrolled variants.
    pub fn ssr_document(&self) -> String {
        let theme = self.enterprise_theme();
        let palette = theme.palette.active();
        let uncontrolled = self.render_markup(
            &self.uncontrolled_state(),
            UNCONTROLLED_ANALYTICS_ID,
            UNCONTROLLED_STATUS_ID,
        );
        let controlled = self.render_markup(
            &self.controlled_state(),
            CONTROLLED_ANALYTICS_ID,
            CONTROLLED_STATUS_ID,
        );

        let mut body = String::new();
        writeln!(
            &mut body,
            "<main data-rustic-input-base-shell=\"{}\" style=\"max-width:760px;margin:64px auto;padding:32px;background:{};color:{};font-family:{};border-radius:12px;box-shadow:0 24px 64px rgba(15,23,42,0.45);\">",
            automation_value(["shell"]),
            palette.background_paper,
            palette.text_primary,
            theme.typography.font_family
        )
        .unwrap();
        writeln!(
            &mut body,
            "  <h1 style=\"margin-top:0;font-size:1.75rem;\">RusticUI InputBase SSR snapshot</h1>"
        )
        .unwrap();
        writeln!(
            &mut body,
            "  <p style=\"margin-top:4px;max-width:60ch;\">{}",
            HYDRATION_NOTE
        )
        .unwrap();
        writeln!(
            &mut body,
            "  <section data-rustic-input-base-mode=\"uncontrolled\">\n    <h2 style=\"margin-bottom:8px;\">Uncontrolled state</h2>\n    <p id=\"{status}\" style=\"margin:4px 0 12px;color:{color};\">Server rendered errors mirror automation attributes for QA snapshots.</p>\n    {input}\n  </section>",
            status = UNCONTROLLED_STATUS_ID,
            color = palette.secondary,
            input = uncontrolled
        )
        .unwrap();
        writeln!(
            &mut body,
            "  <section data-rustic-input-base-mode=\"controlled\" style=\"margin-top:24px;\">\n    <h2 style=\"margin-bottom:8px;\">Controlled state</h2>\n    <p id=\"{status}\" style=\"margin:4px 0 12px;color:{color};\">Hydration consumes the same InputState so data-* flags survive client hand-off.</p>\n    {input}\n  </section>",
            status = CONTROLLED_STATUS_ID,
            color = palette.secondary,
            input = controlled
        )
        .unwrap();
        body.push_str("</main>");

        format!(
            "<!DOCTYPE html><html><head><meta charset=\"utf-8\"/><title>InputBase SSR</title></head><body style=\"margin:0;background:{};color:{};\">{body}</body></html>",
            palette.background_default,
            palette.text_primary,
        )
    }

    /// Build the hydration stub that each framework specific bootstrap binary writes to disk.
    pub fn hydration_stub(&self, framework: &str, hydrate_invocation: &str) -> String {
        format!(
            "// Hydration entry generated by forms-input-base-shared.\n// Framework: {framework}\n// Automation namespace: {namespace}\n// Analytics IDs: {controlled} (controlled), {uncontrolled} (uncontrolled)\n{hydrate_invocation}\n",
            framework = framework,
            namespace = AUTOMATION_NAMESPACE,
            controlled = CONTROLLED_ANALYTICS_ID,
            uncontrolled = UNCONTROLLED_ANALYTICS_ID,
            hydrate_invocation = hydrate_invocation
        )
    }

    /// Produce a README snippet documenting the automation hooks for the generated snapshot.
    pub fn bootstrap_readme(&self, framework: &str) -> String {
        let mut content = String::new();
        writeln!(
            &mut content,
            "# {framework} InputBase bootstrap\n",
            framework = framework.replace('-', " ")
        )
        .unwrap();
        writeln!(
            &mut content,
            "- Automation namespace: `{namespace}`",
            namespace = AUTOMATION_NAMESPACE
        )
        .unwrap();
        writeln!(
            &mut content,
            "- Controlled analytics ID: `{}`",
            CONTROLLED_ANALYTICS_ID
        )
        .unwrap();
        writeln!(
            &mut content,
            "- Uncontrolled analytics ID: `{}`",
            UNCONTROLLED_ANALYTICS_ID
        )
        .unwrap();
        writeln!(
            &mut content,
            "- Hydration note: {HYDRATION_NOTE}\n",
            HYDRATION_NOTE = HYDRATION_NOTE
        )
        .unwrap();
        writeln!(&mut content, "## Automation attributes\n").unwrap();
        for attr in self.automation_attributes() {
            writeln!(&mut content, "- `{attr}`").unwrap();
        }
        content
    }

    /// Helper used by bootstrap binaries to materialise artifacts on disk.
    pub fn bootstrap_artifacts(
        &self,
        framework: &str,
        hydrate_invocation: &str,
    ) -> BootstrapArtifacts {
        BootstrapArtifacts {
            ssr_html: self.ssr_document(),
            hydration_stub: self.hydration_stub(framework, hydrate_invocation),
            readme: self.bootstrap_readme(framework),
        }
    }
}

/// Bundle of files that bootstrap binaries persist.
#[derive(Debug, Clone)]
pub struct BootstrapArtifacts {
    pub ssr_html: String,
    pub hydration_stub: String,
    pub readme: String,
}

/// Join automation segments with the canonical namespace.
pub fn automation_value<I, S>(segments: I) -> String
where
    I: IntoIterator<Item = S>,
    S: Into<String>,
{
    let mut value = String::from(AUTOMATION_NAMESPACE);
    for segment in segments {
        value.push_str("::");
        value.push_str(&segment.into());
    }
    value
}
