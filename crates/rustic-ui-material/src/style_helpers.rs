//! Internal helpers for converting theme driven styles into scoped class names.
//!
//! Components frequently invoke [`css_with_theme!`](mui_styled_engine::css_with_theme)
//! and only require the generated CSS class. Centralizing the class name
//! extraction avoids repetitive `.get_class_name().to_string()` calls while
//! documenting the intended lifecycle of stylist [`Style`] handles.

use mui_styled_engine::Style;
use mui_utils::{attributes_to_html, collect_attributes};

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

/// Converts a [`Style`] into a full attribute map that includes the scoped
/// class followed by any additional key/value pairs.
///
/// Component adapters frequently need to attach both theme-driven styling and
/// ARIA metadata to the same element. Centralizing the merge logic guarantees
/// consistent ordering of the generated attributes across Yew, Leptos, Dioxus
/// and Sycamore integrations.
#[must_use]
pub(crate) fn themed_attributes<I, K, V>(style: Style, iter: I) -> Vec<(String, String)>
where
    I: IntoIterator<Item = (K, V)>,
    K: Into<String>,
    V: Into<String>,
{
    collect_attributes(Some(themed_class(style)), iter)
}

/// Collects themed attributes and renders them into a HTML-ready string.
///
/// The helper mirrors [`themed_attributes`] but directly returns the
/// serialized representation which is handy for SSR adapters. Documenting the
/// pattern encourages component authors to prefer this helper over hand-written
/// string concatenation, reducing the likelihood of missing accessibility data
/// or mis-ordering attributes between frameworks.
#[must_use]
pub(crate) fn themed_attributes_html<I, K, V>(style: Style, iter: I) -> String
where
    I: IntoIterator<Item = (K, V)>,
    K: Into<String>,
    V: Into<String>,
{
    let attrs = themed_attributes(style, iter);
    attributes_to_html(&attrs)
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

    #[test]
    fn themed_attributes_include_class_and_custom_pairs() {
        let style =
            Style::new(css!("color: red;")).expect("css! macro should produce a valid style");
        let attrs = themed_attributes(style, [("role", "button")]);
        assert_eq!(attrs[0].0, "class");
        assert!(attrs.iter().any(|(k, v)| k == "role" && v == "button"));
    }

    #[test]
    fn themed_attributes_html_serializes_pairs() {
        let style =
            Style::new(css!("color: red;")).expect("css! macro should produce a valid style");
        let html = themed_attributes_html(style, [("aria-label", "Save")]);
        assert!(html.contains("class=\""));
        assert!(html.contains("aria-label=\"Save\""));
    }
}
