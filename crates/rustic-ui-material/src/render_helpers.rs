//! Shared rendering helpers that convert theme-aware [`Style`](rustic_ui_styled_engine::Style)
//! handles into serialized HTML fragments.
//!
//! The module complements [`style_helpers`](crate::style_helpers) by layering in
//! HTML assembly logic.  Component crates often need to output pre-rendered
//! markup for server-driven frameworks (Leptos SSR, Dioxus, Sycamore) while also
//! exposing the same attribute maps to client side adapters like Yew. Keeping
//! the HTML formatting routines centralized eliminates subtle drift between
//! adapters and enables downstream automation to stitch together UX flows
//! without rewriting presentation logic for every target runtime.

use rustic_ui_styled_engine::Style;

/// Render an element with the provided tag, [`Style`] and attribute pairs.
///
/// * [`Style`] is converted to a scoped class via
///   [`style_helpers::themed_attributes_html`](crate::style_helpers::themed_attributes_html)
///   so the CSS emitted by [`css_with_theme!`](rustic_ui_styled_engine::css_with_theme)
///   automatically attaches to the element.
/// * `attrs` accepts any iterator of `(key, value)` pairs making it ergonomic to
///   feed attribute builders from `rustic_ui_headless` without additional
///   transformations.
/// * `children` is injected verbatim allowing adapters to pre-render complex
///   layouts upstream.
///
/// The helper intentionally returns a `String` so automated pipelines can stash
/// the serialized markup in caches, golden files or transport layers without
/// forcing each team to reinvent the formatting dance.
#[must_use]
pub(crate) fn render_element_html<I, K, V>(
    tag: &str,
    style: Style,
    attrs: I,
    children: &str,
) -> String
where
    I: IntoIterator<Item = (K, V)>,
    K: Into<String>,
    V: Into<String>,
{
    let attr_string = crate::style_helpers::themed_attributes_html(style, attrs);
    format!(
        "<{tag} {attrs}>{children}</{tag}>",
        tag = tag,
        attrs = attr_string,
        children = children
    )
}

/// Render a self-closing `<div>` element for backdrop surfaces.
///
/// Backdrops often have no inner content but still need a stable element in the
/// DOM to drive transitions. This helper mirrors [`render_element_html`] yet
/// emits the closing tag immediately, keeping SSR output terse while reusing the
/// scoped class machinery.
#[must_use]
pub(crate) fn render_backdrop_html<I, K, V>(style: Style, attrs: I) -> String
where
    I: IntoIterator<Item = (K, V)>,
    K: Into<String>,
    V: Into<String>,
{
    let attr_string = crate::style_helpers::themed_attributes_html(style, attrs);
    format!("<div {attrs}></div>", attrs = attr_string)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rustic_ui_styled_engine::css;

    #[test]
    fn render_element_generates_wrapped_markup() {
        let style = Style::new(css!("color: red;")).expect("valid style");
        let html = render_element_html("span", style, [("role", "note")], "Hello");
        assert!(html.starts_with("<span class=\""));
        assert!(html.contains("role=\"note\""));
        assert!(html.ends_with("Hello</span>"));
    }

    #[test]
    fn render_backdrop_produces_div_wrapper() {
        let style = Style::new(css!("opacity: 0.5;")).expect("valid style");
        let html = render_backdrop_html(style, [("data-open", "true")]);
        assert!(html.starts_with("<div class=\""));
        assert!(html.contains("data-open=\"true\""));
        assert!(html.ends_with("></div>"));
    }
}
