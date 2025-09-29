use rustic_ui_system::{responsive::Responsive, theme::Theme};

/// Declarative representation of a marketing section that participates in the
/// responsive grid. Centralising this state keeps CSR and SSR renders aligned
/// and mirrors the architecture used in production apps where layout intent is
/// codified in configuration rather than spread across templates.
#[derive(Clone, Debug, PartialEq)]
pub struct SectionBlueprint {
    /// Stable automation identifier propagated into `data-rustic-*` attributes
    /// for Playwright/Cypress selectors.
    pub id: &'static str,
    /// Human facing headline rendered alongside the section content.
    pub title: &'static str,
    /// Supporting summary explaining the layout goal.
    pub summary: &'static str,
    /// Responsive grid span describing how wide the section should render at
    /// each breakpoint. Values cascade following CSS media query semantics.
    pub span: Responsive<u16>,
    /// Optional override for the number of columns the grid container exposes
    /// at each breakpoint. When `None`, the shared `grid_columns` helper is
    /// used so sections share the same 12-column baseline.
    pub columns: Option<Responsive<u16>>,
}

impl SectionBlueprint {
    /// Resolves the grid span for a given viewport width.
    pub fn resolved_span(&self, width: u32, theme: &Theme) -> u16 {
        self.span.resolve(width, &theme.breakpoints)
    }

    /// Resolves the number of columns governing the section at the specified
    /// viewport width. Falls back to [`grid_columns`] when no override is set.
    pub fn resolved_columns(&self, width: u32, theme: &Theme) -> u16 {
        self.columns
            .as_ref()
            .unwrap_or(&grid_columns())
            .resolve(width, &theme.breakpoints)
    }
}

/// Returns the canonical responsive column configuration used by the grid
/// container. Material UI defaults to a 12-column system but we keep the helper
/// flexible so design systems can experiment with alternative densities.
pub fn grid_columns() -> Responsive<u16> {
    Responsive {
        xs: 12,
        sm: Some(12),
        md: Some(12),
        lg: Some(12),
        xl: Some(12),
    }
}

/// Returns the marketing sections rendered by the demo. Each entry mirrors a
/// typical enterprise hero/feature/sidebar combination and intentionally uses
/// different spans so the responsive cascade can be observed during hydration.
pub fn layout_blueprint() -> Vec<SectionBlueprint> {
    vec![
        SectionBlueprint {
            id: "hero",
            title: "Hero spotlight",
            summary: "Large marketing hero that collapses to full width on narrow screens.",
            span: Responsive {
                xs: 12,
                sm: Some(12),
                md: Some(8),
                lg: Some(7),
                xl: Some(6),
            },
            columns: None,
        },
        SectionBlueprint {
            id: "features",
            title: "Feature grid",
            summary: "Secondary highlights balancing the hero copy with supporting detail.",
            span: Responsive {
                xs: 12,
                sm: Some(12),
                md: Some(4),
                lg: Some(3),
                xl: Some(3),
            },
            columns: None,
        },
        SectionBlueprint {
            id: "cta-panel",
            title: "Conversion CTA",
            summary: "Sticky call-to-action that remains visible even on dense dashboards.",
            span: Responsive {
                xs: 12,
                sm: Some(12),
                md: Some(6),
                lg: Some(5),
                xl: Some(3),
            },
            columns: Some(Responsive {
                xs: 12,
                sm: Some(12),
                md: Some(12),
                lg: Some(12),
                xl: Some(16),
            }),
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn responsive_spans_resolve() {
        let theme = Theme::default();
        let sections = layout_blueprint();
        let hero = &sections[0];
        assert_eq!(hero.resolved_span(320, &theme), 12);
        assert_eq!(hero.resolved_span(920, &theme), 8);
        assert_eq!(hero.resolved_span(1400, &theme), 7);
    }

    #[test]
    fn columns_default_to_shared_helper() {
        let theme = Theme::default();
        let sections = layout_blueprint();
        let features = &sections[1];
        assert_eq!(features.resolved_columns(1280, &theme), 12);
    }

    #[test]
    fn columns_override_is_honoured() {
        let theme = Theme::default();
        let sections = layout_blueprint();
        let cta = &sections[2];
        assert_eq!(cta.resolved_columns(1600, &theme), 16);
    }
}
