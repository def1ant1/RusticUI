use rustic_ui_styled_engine::{StyleRegistry, Theme};
use proptest::prelude::*;
use stylist::Style;

proptest! {
    /// Property-based test ensuring that arbitrary colors are flushed from the
    /// registry and embedded into the resulting style blocks.  Randomized inputs
    /// help catch any CSS escaping issues or stale cache references.
    #[test]
    fn collected_styles_contain_input(color in proptest::string::string_regex("#[0-9a-fA-F]{6}").unwrap()) {
        let registry = StyleRegistry::new(Theme::default());
        let css = format!("color: {color};");
        Style::new_with_manager(css.clone(), registry.style_manager()).unwrap();
        let out = registry.flush_styles();
        prop_assert!(out.contains(&css));
        // ensure flush drains accumulated styles
        prop_assert!(registry.flush_styles().trim().is_empty());
    }
}
