use mui_system::Theme;

/// Validate spacing helper and default palette values.
#[test]
fn theme_defaults() {
    let theme = Theme::default();
    assert_eq!(theme.spacing(3), 24);
    assert_eq!(theme.palette.light.primary, "#1976d2");
    assert_eq!(
        theme.palette.initial_color_scheme,
        mui_system::theme::ColorScheme::Light
    );
    assert_eq!(theme.palette.dark.background_default, "#121212");
}
