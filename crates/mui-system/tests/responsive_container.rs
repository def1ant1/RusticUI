use mui_system::{container::build_container_style, responsive::Responsive, Theme};

#[test]
fn container_applies_responsive_max_width() {
    let theme = Theme::default();
    let max_width = Responsive {
        xs: "100%".into(),
        sm: Some("640px".into()),
        md: Some("960px".into()),
        lg: Some("1200px".into()),
        xl: Some("1440px".into()),
    };

    let mobile = build_container_style(400, &theme.breakpoints, Some(&max_width), "");
    assert!(mobile.contains("width:100%;"));
    assert!(mobile.contains("max-width:100%;"));

    let desktop =
        build_container_style(1280, &theme.breakpoints, Some(&max_width), "padding:24px;");
    assert!(desktop.contains("max-width:1200px;"));
    assert!(desktop.starts_with("margin-left:auto;margin-right:auto;width:100%;"));
    assert!(desktop.ends_with("padding:24px;"));
}
