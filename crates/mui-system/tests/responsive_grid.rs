use mui_system::{grid::build_grid_style, responsive::Responsive, Theme};
use serde_json::json;

#[test]
fn grid_breakpoints_resolve_width_and_alignment() {
    let theme = Theme::default();
    let columns = Responsive {
        xs: 4,
        sm: Some(8),
        md: Some(12),
        lg: None,
        xl: Some(16),
    };
    let span = Responsive {
        xs: 4,
        sm: Some(4),
        md: Some(6),
        lg: Some(8),
        xl: Some(12),
    };

    let base = build_grid_style(
        500,
        &theme.breakpoints,
        Some(&columns),
        Some(&span),
        Some("center"),
        None,
        Some(&json!({
            "border": "1px solid red",
        })),
    );
    assert!(base.contains("border:1px solid red;"));
    assert!(base.contains("width:100%;"));
    assert!(base.contains("justify-content:center;"));

    let medium = build_grid_style(
        950,
        &theme.breakpoints,
        Some(&columns),
        Some(&span),
        None,
        Some("flex-end"),
        None,
    );
    assert!(medium.contains("width:50%;"));
    assert!(medium.contains("align-items:flex-end;"));

    let extra_large = build_grid_style(
        1600,
        &theme.breakpoints,
        Some(&columns),
        Some(&span),
        None,
        None,
        None,
    );
    assert!(extra_large.contains("width:75%;"));
}
