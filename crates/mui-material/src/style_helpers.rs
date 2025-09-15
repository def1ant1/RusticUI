//! Internal helpers for converting theme driven styles into scoped class names.
//!
//! Components frequently invoke [`css_with_theme!`](mui_styled_engine::css_with_theme)
//! and only require the generated CSS class. Centralizing the class name
//! extraction avoids repetitive `.get_class_name().to_string()` calls while
//! documenting the intended lifecycle of stylist [`Style`] handles.

use mui_styled_engine::Style;

/// Consumes a [`Style`] and returns the scoped class name produced by the
/// styled engine.
///
/// Dropping the [`Style`] after retrieving the class is sufficient because the
/// engine registers the CSS with the global manager during creation. The helper
/// makes this pattern explicit so component modules follow the same lifecycle.
#[must_use]
pub(crate) fn themed_class(style: Style) -> String {
    style.get_class_name().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use mui_styled_engine::css;

    #[test]
    fn extracts_class_name() {
        let style =
            Style::new(css!("color: red;")).expect("css! macro should produce a valid style");
        let class = themed_class(style);
        assert!(!class.is_empty());
    }
}
