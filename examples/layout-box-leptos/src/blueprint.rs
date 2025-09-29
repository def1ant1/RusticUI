use rustic_ui_system::{responsive::Responsive, theme::Theme};

/// Declarative representation of a responsive panel rendered by the demo.
#[derive(Clone, Debug, PartialEq)]
pub struct PanelBlueprint {
    /// Stable automation identifier.
    pub id: &'static str,
    /// Human facing heading for the panel.
    pub title: &'static str,
    /// Supporting copy describing the layout decision.
    pub summary: &'static str,
    /// Responsive padding scale applied to the RusticUI `Box` component.
    pub padding: Responsive<String>,
    /// Responsive width constraint expressed as CSS lengths.
    pub max_width: Responsive<String>,
}

impl PanelBlueprint {
    /// Resolves the padding at the supplied viewport width.
    pub fn resolved_padding(&self, width: u32, theme: &Theme) -> String {
        self.padding.resolve(width, &theme.breakpoints)
    }

    /// Resolves the maximum width at the supplied viewport width.
    pub fn resolved_max_width(&self, width: u32, theme: &Theme) -> String {
        self.max_width.resolve(width, &theme.breakpoints)
    }
}

/// Panels rendered by the example. The data mirrors common enterprise sections:
/// introduction, feature summary, and compliance notes.
pub fn panel_blueprint() -> Vec<PanelBlueprint> {
    vec![
        PanelBlueprint {
            id: "intro",
            title: "Adaptive overview",
            summary: "Padding scales with the viewport so copy remains legible on narrow devices.",
            padding: Responsive {
                xs: "20px".into(),
                sm: Some("24px".into()),
                md: Some("28px".into()),
                lg: Some("32px".into()),
                xl: Some("36px".into()),
            },
            max_width: Responsive {
                xs: "100%".into(),
                sm: Some("640px".into()),
                md: Some("720px".into()),
                lg: Some("800px".into()),
                xl: Some("880px".into()),
            },
        },
        PanelBlueprint {
            id: "features",
            title: "Signal-driven layout",
            summary: "Box props are derived from runtime signals so hydration preserves intent.",
            padding: Responsive {
                xs: "18px".into(),
                sm: Some("22px".into()),
                md: Some("24px".into()),
                lg: Some("26px".into()),
                xl: Some("28px".into()),
            },
            max_width: Responsive {
                xs: "100%".into(),
                sm: Some("720px".into()),
                md: Some("760px".into()),
                lg: Some("820px".into()),
                xl: Some("900px".into()),
            },
        },
        PanelBlueprint {
            id: "compliance",
            title: "Compliance ready",
            summary: "Automation hooks and deterministic spans keep audits predictable.",
            padding: Responsive {
                xs: "16px".into(),
                sm: Some("18px".into()),
                md: Some("20px".into()),
                lg: Some("22px".into()),
                xl: Some("24px".into()),
            },
            max_width: Responsive {
                xs: "100%".into(),
                sm: Some("560px".into()),
                md: Some("640px".into()),
                lg: Some("720px".into()),
                xl: Some("760px".into()),
            },
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn padding_resolves_across_breakpoints() {
        let theme = Theme::default();
        let panels = panel_blueprint();
        let intro = &panels[0];
        assert_eq!(intro.resolved_padding(320, &theme), "20px");
        assert_eq!(intro.resolved_padding(960, &theme), "28px");
        assert_eq!(intro.resolved_padding(1400, &theme), "32px");
    }

    #[test]
    fn max_width_resolves_across_breakpoints() {
        let theme = Theme::default();
        let panels = panel_blueprint();
        let compliance = &panels[2];
        assert_eq!(compliance.resolved_max_width(1280, &theme), "720px");
    }
}
