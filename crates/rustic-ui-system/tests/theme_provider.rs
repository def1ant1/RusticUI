use rustic_ui_styled_engine_macros::Theme as ThemeOverride;
extern crate rustic_ui_system as rustic_ui_styled_engine;

use rustic_ui_system::theme::{ColorScheme, Palette};
use rustic_ui_system::theme_provider::{
    material_css_baseline, material_css_baseline_from_theme, material_theme, material_theme_dark,
    material_theme_for_scheme, material_theme_light, material_theme_with_optional_overrides,
    material_theme_with_overrides,
};

#[derive(Clone)]
struct PaletteOverrides {
    primary: String,
    secondary: String,
}

impl From<PaletteOverrides> for Palette {
    fn from(value: PaletteOverrides) -> Self {
        let mut palette = Palette::default();
        palette.light.primary = value.primary.clone();
        palette.light.secondary = value.secondary.clone();
        palette.dark.primary = value.primary;
        palette.dark.secondary = value.secondary;
        palette
    }
}

#[derive(ThemeOverride)]
struct PartialTheme {
    palette: PaletteOverrides,
}

#[derive(ThemeOverride)]
struct OptionalTheme {
    palette: Option<PaletteOverrides>,
}

#[test]
fn derived_overrides_merge_with_material_defaults() {
    let theme = material_theme_with_overrides(PartialTheme {
        palette: PaletteOverrides {
            primary: "#102030".into(),
            secondary: "#405060".into(),
        },
    });

    assert_eq!(theme.palette.light.primary, "#102030");
    assert_eq!(theme.palette.light.secondary, "#405060");
    assert_eq!(theme.palette.dark.primary, "#102030");
    assert_eq!(theme.palette.dark.secondary, "#405060");
    // Fields not supplied by the overrides fall back to Material defaults.
    assert_eq!(
        theme.palette.scheme(ColorScheme::Light).background_default,
        material_theme()
            .palette
            .scheme(ColorScheme::Light)
            .background_default
    );
    assert_eq!(
        theme.typography.font_family,
        material_theme().typography.font_family
    );
}

#[test]
fn optional_overrides_are_respected_when_present() {
    let overrides = Some(OptionalTheme {
        palette: Some(PaletteOverrides {
            primary: "#abcdef".into(),
            secondary: "#123456".into(),
        }),
    });

    let theme = material_theme_with_optional_overrides(overrides);
    assert_eq!(theme.palette.light.primary, "#abcdef");
    assert_eq!(theme.palette.light.secondary, "#123456");
}

#[test]
fn baseline_injection_uses_theme_tokens() {
    let css = material_css_baseline();
    assert!(css.contains("box-sizing: border-box"));
    assert!(css.contains(&material_theme().typography.font_family));
    assert!(css.contains(
        &material_theme()
            .palette
            .scheme(ColorScheme::Light)
            .background_default
    ));
    assert!(css.contains("color-scheme"));
}

#[test]
fn css_baseline_includes_both_schemes() {
    let theme = material_theme();
    let css = material_css_baseline_from_theme(&theme);
    assert!(css.contains(&theme.palette.light.background_default));
    assert!(css.contains(&theme.palette.dark.background_default));
    assert!(css.contains("@media (prefers-color-scheme: dark)"));
    assert!(css.contains("[data-mui-color-scheme='dark']"));
}

#[test]
fn scheme_specific_helpers_adjust_initial_mode() {
    assert_eq!(
        material_theme_light().palette.initial_color_scheme,
        ColorScheme::Light
    );
    assert_eq!(
        material_theme_dark().palette.initial_color_scheme,
        ColorScheme::Dark
    );

    let forced = material_theme_for_scheme(ColorScheme::Dark);
    assert_eq!(forced.palette.initial_color_scheme, ColorScheme::Dark);
}

#[test]
fn css_differs_between_light_and_dark_templates() {
    let light_theme = material_theme_light();
    let dark_theme = material_theme_dark();

    let css_light = material_css_baseline_from_theme(&light_theme);
    let css_dark = material_css_baseline_from_theme(&dark_theme);

    assert_ne!(css_light, css_dark);
    assert!(css_light.contains(&light_theme.palette.light.background_default));
    assert!(css_dark.contains(&dark_theme.palette.dark.background_default));
    assert!(css_dark.contains("color-scheme: dark"));
}
