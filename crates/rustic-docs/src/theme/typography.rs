//! Typography scaffolding reused by the docs portal.
//!
//! RusticUI exposes a complete typography ramp via `rustic-ui-system`.  The
//! helpers below snapshot the values into simple structs so Leptos and Yew
//! components can reference the same data without re-computing it on every
//! render.  This keeps automation deterministic and documents the contract for
//! enterprise teams overriding fonts at build time.

use once_cell::sync::Lazy;
use rustic_ui_system::theme_provider::material_theme;

/// Subset of typography tokens surfaced to docs components.
#[derive(Clone, Debug)]
pub struct TypographySnapshot {
    /// Primary sans-serif stack used for headings and body copy.
    pub font_family: String,
    /// Monospace stack rendered by code snippets.
    pub font_family_monospace: String,
    /// Base body font size expressed in pixels.
    pub body_font_size_px: f32,
    /// Base html font size expressed in pixels.
    pub html_font_size_px: f32,
    /// Representative heading sizes.
    pub heading_scale: [f32; 3],
    /// Default line height multiplier.
    pub line_height: f32,
    /// Letter spacing used for uppercase buttons.
    pub button_letter_spacing: f32,
}

impl TypographySnapshot {
    fn from_theme() -> Self {
        let theme = material_theme();
        let typography = theme.typography;
        Self {
            font_family: typography.font_family.clone(),
            font_family_monospace: typography.font_family_monospace.clone(),
            body_font_size_px: typography.body1,
            html_font_size_px: typography.html_font_size,
            heading_scale: [typography.h2, typography.h4, typography.h6],
            line_height: typography.line_height,
            button_letter_spacing: typography.button_letter_spacing,
        }
    }
}

static TYPOGRAPHY: Lazy<TypographySnapshot> = Lazy::new(TypographySnapshot::from_theme);

/// Returns a cached snapshot of the typography ramp used across docs widgets.
#[must_use]
pub fn typography_scale() -> &'static TypographySnapshot {
    &*TYPOGRAPHY
}
