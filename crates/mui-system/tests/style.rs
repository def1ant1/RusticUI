use mui_system::{
    background_color, border_radius, box_shadow, font_size, font_weight, height, left, line_height,
    margin, min_width, padding, position, top, width,
};

/// Ensure macro generated helpers emit valid CSS strings.
#[test]
fn spacing_helpers_work() {
    assert_eq!(margin("4px"), "margin:4px;");
    assert_eq!(padding("2px"), "padding:2px;");
    assert_eq!(font_size("16px"), "font-size:16px;");
    assert_eq!(font_weight("600"), "font-weight:600;");
    assert_eq!(line_height("24px"), "line-height:24px;");
    assert_eq!(width("100%"), "width:100%;");
    assert_eq!(height("auto"), "height:auto;");
    assert_eq!(min_width("120px"), "min-width:120px;");
    assert_eq!(background_color("#fff"), "background-color:#fff;");
    assert_eq!(border_radius("8px"), "border-radius:8px;");
    assert_eq!(
        box_shadow("0 1px 2px rgba(0,0,0,0.2)"),
        "box-shadow:0 1px 2px rgba(0,0,0,0.2);"
    );
    assert_eq!(position("absolute"), "position:absolute;");
    assert_eq!(top("8px"), "top:8px;");
    assert_eq!(left("4px"), "left:4px;");
}
