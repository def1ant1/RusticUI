use mui_system::{
    container::build_container_style,
    r#box::{build_box_style, BoxStyleInputs},
    responsive::Responsive,
    Theme,
};
use serde_json::json;

#[test]
fn container_json_sx_overrides_width() {
    let theme = Theme::default();
    let style = build_container_style(
        1024,
        &theme.breakpoints,
        None,
        Some(&json!({
            "width": "80%",
            "background-color": "#fafafa",
        })),
    );

    assert!(style.contains("width:80%;"));
    assert!(!style.contains("width:100%;"));
    assert!(style.contains("background-color:#fafafa;"));
}

#[test]
fn box_json_sx_merges_and_overrides_padding() {
    let theme = Theme::default();
    let padding = Responsive::constant("8px".to_string());
    let style = build_box_style(
        800,
        &theme.breakpoints,
        BoxStyleInputs {
            margin: None,
            padding: Some(&padding),
            font_size: None,
            font_weight: None,
            line_height: None,
            letter_spacing: None,
            color: None,
            background_color: None,
            width: None,
            height: None,
            min_width: None,
            max_width: None,
            min_height: None,
            max_height: None,
            position: None,
            top: None,
            right: None,
            bottom: None,
            left: None,
            display: None,
            align_items: None,
            justify_content: None,
            sx: Some(&json!({
                "padding": "24px",
                "box-shadow": "0 1px 2px rgba(0,0,0,0.2)",
            })),
        },
    );

    assert!(style.contains("box-shadow:0 1px 2px rgba(0,0,0,0.2);"));
    assert!(style.contains("padding:24px;"));
    assert!(!style.contains("padding:8px;"));
}
