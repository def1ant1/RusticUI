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

use std::collections::BTreeMap;

use rustic_ui_headless::layout::{Breakpoint, BreakpointConfig};
use rustic_ui_styled_engine::Style;

/// Ordered map of CSS custom properties emitted by layout renderers.
///
/// [`BTreeMap`] keeps the serialization deterministic which is important when
/// SSR snapshots are compared against client renders in CI pipelines.  All
/// layout renderers populate the map using the helpers in this module so
/// adapters can trivially feed the result into attribute builders or inline
/// `style="..."` strings without manually juggling ordering rules.
pub(crate) type CssVariableMap = BTreeMap<String, String>;

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

/// Returns the canonical suffix attached to breakpoint scoped CSS variables.
///
/// `Breakpoint::Base` intentionally maps to an empty suffix so base styles read
/// as `--rustic_ui_container_padding`.  Derived breakpoints follow the
/// `--rustic_ui_container_padding-sm` convention which mirrors the naming used
/// by the JavaScript adapters.  Centralising this logic eliminates the risk of
/// typos whenever new layout primitives are introduced.
#[must_use]
pub(crate) const fn breakpoint_suffix(breakpoint: Breakpoint) -> &'static str {
    match breakpoint {
        Breakpoint::Base => "",
        Breakpoint::Sm => "-sm",
        Breakpoint::Md => "-md",
        Breakpoint::Lg => "-lg",
        Breakpoint::Xl => "-xl",
        Breakpoint::Xxl => "-xxl",
    }
}

/// Compose a custom property name for a given component/token/breakpoint
/// combination.
#[must_use]
pub(crate) fn css_var(component: &str, token: &str, breakpoint: Breakpoint) -> String {
    format!(
        "--rustic_ui_{component}_{token}{suffix}",
        component = component,
        token = token,
        suffix = breakpoint_suffix(breakpoint)
    )
}

/// Insert CSS variables for every breakpoint in the configuration using the
/// supplied callback.
///
/// Renderers frequently evaluate headless state per breakpoint.  Rather than
/// repeat the iteration boilerplate for each component we provide a helper that
/// feeds each [`Breakpoint`] into the caller supplied closure.  The closure is
/// expected to return `(token, value)` pairs which are automatically prefixed
/// following the [`css_var`] naming rules.
pub(crate) fn collect_responsive_variables<F>(
    map: &mut CssVariableMap,
    component: &str,
    breakpoints: &BreakpointConfig,
    mut f: F,
) where
    F: FnMut(Breakpoint) -> Vec<(&'static str, String)>,
{
    for (breakpoint, _) in breakpoints.iter() {
        for (token, value) in f(breakpoint) {
            map.insert(css_var(component, token, breakpoint), value);
        }
    }
}

/// Normalise spacing tokens ensuring SSR always emits a concrete CSS value.
///
/// Some state machines allow empty strings to represent "no spacing".  CSS
/// variables cannot contain the empty string, therefore this helper coerces
/// blank inputs into `0px`.  Downstream adapters consistently rely on this
/// helper which keeps server rendered markup aligned with the client side
/// hydration pass.
#[must_use]
pub(crate) fn normalise_spacing_token(token: &str) -> String {
    if token.trim().is_empty() {
        String::from("0px")
    } else {
        token.to_string()
    }
}

/// Normalise arbitrary CSS tokens while providing a semantic fallback.
#[must_use]
pub(crate) fn normalise_css_token(token: &str, fallback: &str) -> String {
    if token.trim().is_empty() {
        fallback.to_string()
    } else {
        token.to_string()
    }
}

/// Convert a boolean into the canonical CSS custom property string value.
///
/// Multiple layout renderers expose boolean state (e.g. the Hidden primitive's
/// `aria-hidden` toggle).  Encoding this logic in one helper keeps every
/// renderer serialising booleans with the exact same casing which simplifies
/// snapshot assertions and cross-language parity checks.
#[must_use]
pub(crate) fn bool_to_css_flag(value: bool) -> String {
    if value {
        String::from("true")
    } else {
        String::from("false")
    }
}

/// Translate a boolean visibility flag into a display token appropriate for CSS.
///
/// Material components frequently model visibility as `true/false` yet need to
/// emit explicit CSS strings for SSR.  Using `revert-layer` lets author defined
/// display styles bubble back in when the element is visible while still
/// allowing server rendered snapshots to capture the intent precisely.
#[must_use]
pub(crate) fn visibility_to_display(hidden: bool) -> String {
    if hidden {
        String::from("none")
    } else {
        String::from("revert-layer")
    }
}

/// Convert the same visibility flag into the CSS `visibility` property value.
#[must_use]
pub(crate) fn visibility_to_visibility(hidden: bool) -> String {
    if hidden {
        String::from("hidden")
    } else {
        String::from("visible")
    }
}

/// Convert a [`CssVariableMap`] into a SSR friendly inline style string.
#[must_use]
pub(crate) fn css_variables_to_style(map: &CssVariableMap) -> String {
    map.iter()
        .map(|(name, value)| format!("{name}: {value};"))
        .collect::<Vec<_>>()
        .join(" ")
}

/// Build a CSS grid template string for the supplied column count.
#[must_use]
pub(crate) fn grid_template(columns: u16) -> String {
    format!("repeat({columns}, minmax(0, 1fr))")
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
