use mui_system::Theme;

/// Validate spacing helper and default palette values.
#[test]
fn theme_defaults() {
    let theme = Theme::default();
    assert_eq!(theme.spacing(3), 24);
    assert_eq!(theme.palette.primary, "#1976d2");
}
