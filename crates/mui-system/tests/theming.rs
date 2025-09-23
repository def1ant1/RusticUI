use mui_system::theme::JoyTheme;
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

/// Ensure Joy overrides serialize/deserialize cleanly across the JSON boundary.
#[test]
fn joy_overrides_round_trip() {
    let theme = Theme::with_joy_overrides(
        JoyTheme::builder()
            .focus_thickness(6)
            .focus_palette_reference("success")
            .shadow_surface("0 2px 20px rgba(0,0,0,0.25)")
            .build(),
    );
    let json = serde_json::to_string(&theme).expect("serialize joy theme");
    let restored: Theme = serde_json::from_str(&json).expect("deserialize joy theme");
    assert_eq!(restored.joy.focus.thickness, 6);
    assert_eq!(restored.joy.focus.palette_reference, "success");
    assert_eq!(restored.joy.shadow.surface, "0 2px 20px rgba(0,0,0,0.25)");
}
