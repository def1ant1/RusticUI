//! Palette helpers shared across the documentation surfaces.
//!
//! The documentation portal renders light and dark layouts side-by-side to
//! illustrate how RusticUI primitives react to theme overrides.  This module
//! exposes curated snapshots so Leptos and Yew components can surface those
//! tokens without duplicating the low level `rustic-ui-system` plumbing.

use once_cell::sync::Lazy;
use rustic_ui_system::theme::ColorScheme;
use rustic_ui_system::theme_provider::material_theme_for_scheme;

/// Immutable snapshot of the Material palette for a specific colour scheme.
#[derive(Clone, Debug, PartialEq)]
pub struct PaletteSnapshot {
    /// Scheme the snapshot was derived from.
    pub scheme: ColorScheme,
    /// Primary brand colour.
    pub primary: String,
    /// Secondary accent used by call-to-action components.
    pub secondary: String,
    /// Neutral tone leveraged by Joy surfaces.
    pub neutral: String,
    /// Paper colour rendered behind elevated surfaces such as cards.
    pub surface: String,
    /// Default page background colour.
    pub background: String,
    /// Primary text colour with the strongest contrast.
    pub text_primary: String,
    /// Secondary/disabled text tone.
    pub text_secondary: String,
}

impl PaletteSnapshot {
    /// Builds a snapshot by cloning the palette generated for `scheme`.
    fn from_scheme(scheme: ColorScheme) -> Self {
        let theme = material_theme_for_scheme(scheme);
        let palette = theme.palette.scheme(scheme);
        Self {
            scheme,
            primary: palette.primary.clone(),
            secondary: palette.secondary.clone(),
            neutral: palette.neutral.clone(),
            surface: palette.background_paper.clone(),
            background: palette.background_default.clone(),
            text_primary: palette.text_primary.clone(),
            text_secondary: palette.text_secondary.clone(),
        }
    }
}

static BASELINE: Lazy<[PaletteSnapshot; 2]> = Lazy::new(|| {
    [
        PaletteSnapshot::from_scheme(ColorScheme::Light),
        PaletteSnapshot::from_scheme(ColorScheme::Dark),
    ]
});

/// Returns immutable palette snapshots for both light and dark schemes.
#[must_use]
pub fn baseline_palettes() -> &'static [PaletteSnapshot; 2] {
    &*BASELINE
}

/// Convenience helper returning a cloned snapshot for the requested scheme.
#[must_use]
pub fn palette_for_scheme(scheme: ColorScheme) -> PaletteSnapshot {
    baseline_palettes()
        .iter()
        .find(|snapshot| snapshot.scheme == scheme)
        .cloned()
        .unwrap_or_else(|| PaletteSnapshot::from_scheme(scheme))
}
