//! Accessibility helpers for composing HTML attributes across frameworks.
//!
//! The utilities in this module make it easy to build up collections of
//! attributes that include ARIA metadata without repeating boilerplate. They
//! accept any iterator of key/value pairs so component adapters can merge
//! framework specific data with information emitted from headless state
//! machines. By collecting attributes in a structured way we can render
//! consistent markup for SSR focused adapters while still feeding individual
//! properties to WebAssembly oriented front-ends.

/// Collects attributes into a vector while optionally prepending a CSS class.
///
/// The function accepts any iterator that yields key/value pairs convertible to
/// `String`. This keeps the helper ergonomic for adapters that work with
/// `&'static str` keys (common for ARIA attributes) as well as dynamic `String`
/// values coming from user supplied properties.
#[must_use]
pub fn collect_attributes<C, I, K, V>(class: Option<C>, iter: I) -> Vec<(String, String)>
where
    C: Into<String>,
    I: IntoIterator<Item = (K, V)>,
    K: Into<String>,
    V: Into<String>,
{
    let mut attrs = Vec::new();
    if let Some(class) = class {
        attrs.push(("class".to_string(), class.into()));
    }
    extend_attributes(&mut attrs, iter);
    attrs
}

/// Extends an existing attribute collection with additional key/value pairs.
///
/// This is primarily used when components need to merge generated ARIA
/// metadata with caller supplied overrides. Chaining the helper keeps the
/// pattern consistent across adapters and reduces the likelihood of typos in
/// attribute names.
pub fn extend_attributes<I, K, V>(attrs: &mut Vec<(String, String)>, iter: I)
where
    I: IntoIterator<Item = (K, V)>,
    K: Into<String>,
    V: Into<String>,
{
    for (key, value) in iter {
        attrs.push((key.into(), value.into()));
    }
}

/// Renders the collected attributes into a HTML compatible string.
///
/// The output joins every `key="value"` pair with spaces making it trivial to
/// embed inside SSR adapters that output raw strings. Using a helper for this
/// ensures we escape values consistently (they are currently inserted as-is,
/// mirroring the repository's existing patterns) and provides a single spot to
/// enhance escaping in the future if necessary.
#[must_use]
pub fn attributes_to_html(attrs: &[(String, String)]) -> String {
    attrs
        .iter()
        .map(|(key, value)| format!(r#"{key}="{value}""#))
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn collects_and_renders_attributes() {
        let attrs =
            collect_attributes(Some("btn"), [("role", "button"), ("aria-pressed", "false")]);
        assert_eq!(attrs[0], ("class".into(), "btn".into()));
        assert_eq!(attrs.len(), 3);
        let html = attributes_to_html(&attrs);
        assert!(html.contains("class=\"btn\""));
        assert!(html.contains("role=\"button\""));
        assert!(html.contains("aria-pressed=\"false\""));
    }

    #[test]
    fn extends_existing_attributes() {
        let mut attrs = vec![("class".into(), "btn".into())];
        extend_attributes(&mut attrs, [("data-id", "42"), ("aria-live", "polite")]);
        assert_eq!(attrs.len(), 3);
        assert!(attrs.iter().any(|(k, _)| k == "aria-live"));
    }
}
