use rustic_ui_styled_engine::css_with_theme;

fn main() {
    let style = css_with_theme!(r#"color: ${p};"#, p = theme.palette.primary.clone());
    let _ = style.get_class_name();
}
