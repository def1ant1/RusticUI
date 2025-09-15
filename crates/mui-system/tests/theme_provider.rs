use mui_styled_engine_macros::Theme as ThemeOverride;
extern crate mui_system as mui_styled_engine;

use mui_system::theme::Palette;
use mui_system::theme_provider::{
    material_css_baseline, material_theme, material_theme_with_optional_overrides,
    material_theme_with_overrides,
};

#[derive(Clone)]
struct PaletteOverrides {
    primary: String,
    secondary: String,
}

impl From<PaletteOverrides> for Palette {
    fn from(value: PaletteOverrides) -> Self {
        Palette {
            primary: value.primary,
            secondary: value.secondary,
            ..Palette::default()
        }
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

    assert_eq!(theme.palette.primary, "#102030");
    assert_eq!(theme.palette.secondary, "#405060");
    // Fields not supplied by the overrides fall back to Material defaults.
    assert_eq!(
        theme.palette.background_default,
        material_theme().palette.background_default
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
    assert_eq!(theme.palette.primary, "#abcdef");
    assert_eq!(theme.palette.secondary, "#123456");
}

#[test]
fn baseline_injection_uses_theme_tokens() {
    let css = material_css_baseline();
    assert!(css.contains("box-sizing: border-box"));
    assert!(css.contains(&material_theme().typography.font_family));
    assert!(css.contains(&material_theme().palette.background_default));
}
