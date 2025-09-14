use mui_system::{margin, padding};

/// Ensure macro generated helpers emit valid CSS strings.
#[test]
fn spacing_helpers_work() {
    assert_eq!(margin("4px"), "margin:4px;");
    assert_eq!(padding("2px"), "padding:2px;");
}
