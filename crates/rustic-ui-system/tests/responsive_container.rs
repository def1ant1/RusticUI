use rustic_ui_system::{
    container::{build_container_style, ContainerStyleInputs},
    responsive::Responsive,
    Theme,
};
use serde_json::json;

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

    let mobile = build_container_style(
        400,
        &theme.breakpoints,
        ContainerStyleInputs {
            max_width: Some(&max_width),
            sx: None,
        },
    );
    assert!(mobile.contains("width:100%;"));
    assert!(mobile.contains("max-width:100%;"));

    let desktop = build_container_style(
        1280,
        &theme.breakpoints,
        ContainerStyleInputs {
            max_width: Some(&max_width),
            sx: Some(&json!({
                "padding": "24px",
            })),
        },
    );
    assert!(desktop.contains("max-width:1200px;"));
    assert!(desktop.contains("margin-left:auto;"));
    assert!(desktop.contains("margin-right:auto;"));
    assert!(desktop.contains("width:100%;"));
    assert!(desktop.contains("padding:24px;"));
}
