//! Internal helpers for converting theme driven styles into scoped class names.
//!
//! Components frequently invoke [`css_with_theme!`](rustic_ui_styled_engine::css_with_theme)
//! and only require the generated CSS class. Centralizing the class name
//! extraction avoids repetitive `.get_class_name().to_string()` calls while
//! documenting the intended lifecycle of stylist [`Style`] handles.

use rustic_ui_styled_engine::Style;
use rustic_ui_utils::{attributes_to_html, collect_attributes};

/// Global prefix applied to every automation selector emitted by Material components.
///
/// All DOM ids and `data-*` attributes intended for QA automation or accessibility
/// smoke tests should begin with this prefix. Centralising the prefix (and the
/// accompanying formatting helpers) keeps selectors stable across SSR and client
/// renders while dramatically reducing the risk of typos when new components are
/// introduced.
pub(crate) const COMPONENT_PREFIX: &str = "rustic";

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

/// Compose a deterministic automation DOM id that adheres to the workspace contract.
///
/// # Automation contract
///
/// * Every automation selector must start with [`COMPONENT_PREFIX`] so QA suites can
///   glob on `rustic-*` without worrying about historical aliases.
/// * Component adapters pass the logical `component` name (e.g. `"select"`,
///   `"table"`) along with the optional user provided identifier and any additional
///   `segments` describing the element (such as `"trigger"`, `"row-3"`).
/// * All inputs are sanitised to `kebab-case` so the same selector is produced
///   regardless of whether the caller provided `snake_case`, spaces or uppercase
///   values. Invalid characters collapse to hyphens which keeps CSS and testing
///   tooling happy.
///
/// The resulting id is safe to use both as a DOM `id` and as the value for
/// automation-focused `data-*` attributes. By funnelling every component through
/// this helper we minimise the amount of manual string formatting and guarantee
/// that SSR snapshots, client renders, and integration tests share the same
/// selectors.
#[must_use]
pub(crate) fn automation_id<I, S>(component: &str, user_id: Option<&str>, segments: I) -> String
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    let mut parts = Vec::new();
    parts.push(sanitise(component));

    if let Some(id) = user_id {
        let sanitised = sanitise(id);
        if !sanitised.is_empty() {
            parts.push(sanitised);
        }
    }

    for segment in segments {
        let sanitised = sanitise(segment.as_ref());
        if !sanitised.is_empty() {
            parts.push(sanitised);
        }
    }

    format!("{COMPONENT_PREFIX}-{}", parts.join("-"))
}

/// Compose the attribute name for automation-focused `data-*` hooks.
///
/// Unlike [`automation_id`], the attribute name never incorporates the caller's
/// `user_id` because QA tooling expects the attribute key to remain stable across
/// component instances. Only the logical `component` name and descriptive
/// `segments` participate in the key.
#[must_use]
pub(crate) fn automation_data_attr<I, S>(component: &str, segments: I) -> String
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    format!("data-{}", automation_id(component, None, segments))
}

fn sanitise(value: &str) -> String {
    let mut output = String::new();
    let mut prev_dash = false;

    for ch in value.chars() {
        let mapped = match ch {
            'A'..='Z' => Some(ch.to_ascii_lowercase()),
            'a'..='z' | '0'..='9' => Some(ch),
            '-' | '_' | ' ' | ':' | '.' | '/' => None,
            _ => None,
        };

        if let Some(valid) = mapped {
            output.push(valid);
            prev_dash = false;
        } else if !prev_dash {
            output.push('-');
            prev_dash = true;
        }
    }

    let trimmed = output.trim_matches('-').to_string();

    if trimmed.is_empty() {
        String::from("component")
    } else {
        trimmed
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rustic_ui_styled_engine::css;

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

    #[test]
    fn automation_id_sanitises_segments() {
        let id = automation_id("Select", Some("Team Menu"), ["Trigger", "Primary"]);
        assert_eq!(id, "rustic-select-team-menu-trigger-primary");
    }

    #[test]
    fn automation_data_attr_excludes_user_segment() {
        let attr = automation_data_attr("tooltip", ["surface"]);
        assert_eq!(attr, "data-rustic-tooltip-surface");
    }
}
