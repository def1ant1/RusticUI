use mui_styled_engine::{StyleRegistry, Theme};
use stylist::Style;

/// Helper function simulating a nested component hierarchy where inner
/// components also create styles.  All styles should end up in the same
/// registry.
fn inner_layer(registry: &StyleRegistry) {
    Style::new_with_manager("background: blue;", registry.style_manager()).unwrap();
}

#[test]
fn styles_are_scoped_per_registry() {
    // First render with red + blue styles
    let reg1 = StyleRegistry::new(Theme::default());
    Style::new_with_manager("color: red;", reg1.style_manager()).unwrap();
    inner_layer(&reg1);
    let styles1 = reg1.flush_styles();
    assert!(styles1.contains("color: red"));
    assert!(styles1.contains("background: blue"));
    assert!(
        reg1.flush_styles().trim().is_empty(),
        "styles leak after flush"
    );

    // Second render should not see previous styles
    let reg2 = StyleRegistry::new(Theme::default());
    Style::new_with_manager("color: green;", reg2.style_manager()).unwrap();
    let styles2 = reg2.flush_styles();
    assert!(styles2.contains("color: green"));
    assert!(!styles2.contains("color: red"));
}
