use mui_system::{
    responsive::Responsive,
    stack::{build_stack_style, StackDirection},
    Theme,
};

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
        None,
        Some(&spacing),
        Some("center"),
        None,
        "",
    );
    assert!(column.contains("display:flex;"));
    assert!(column.contains("flex-direction:column;"));
    assert!(column.contains("gap:4px;"));
    assert!(column.contains("align-items:center;"));

    let row = build_stack_style(
        1000,
        &theme.breakpoints,
        Some(StackDirection::Row),
        Some(&spacing),
        None,
        Some("space-between"),
        "background:blue;",
    );
    assert!(row.contains("flex-direction:row;"));
    assert!(row.contains("gap:16px;"));
    assert!(row.contains("justify-content:space-between;"));
    assert!(row.ends_with("background:blue;"));
}
