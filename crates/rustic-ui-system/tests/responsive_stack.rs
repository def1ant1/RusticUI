use rustic_ui_system::{
    responsive::Responsive,
    stack::{build_stack_style, StackDirection, StackStyleInputs},
    Theme,
};
use serde_json::json;

#[test]
fn stack_spacing_scales_with_breakpoints() {
    let theme = Theme::default();
    let spacing = Responsive {
        xs: "4px".into(),
        sm: Some("8px".into()),
        md: Some("16px".into()),
        lg: None,
        xl: Some("32px".into()),
    };

    let column = build_stack_style(
        480,
        &theme.breakpoints,
        StackStyleInputs {
            direction: None,
            spacing: Some(&spacing),
            align_items: Some("center"),
            justify_content: None,
            sx: None,
        },
    );
    assert!(column.contains("display:flex;"));
    assert!(column.contains("flex-direction:column;"));
    assert!(column.contains("gap:4px;"));
    assert!(column.contains("align-items:center;"));

    let row = build_stack_style(
        1000,
        &theme.breakpoints,
        StackStyleInputs {
            direction: Some(StackDirection::Row),
            spacing: Some(&spacing),
            align_items: None,
            justify_content: Some("space-between"),
            sx: Some(&json!({
                "background": "blue",
            })),
        },
    );
    assert!(row.contains("flex-direction:row;"));
    assert!(row.contains("gap:16px;"));
    assert!(row.contains("justify-content:space-between;"));
    assert!(row.contains("background:blue;"));
}
