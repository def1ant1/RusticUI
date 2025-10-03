//! Strongly typed builders for Material selection control rendering.
//!
//! These builders replace ad-hoc HTML helpers with enterprise-friendly
//! descriptors that separate visual classes, ARIA metadata, automation signals
//! and analytics hints.  Builders can be converted into attribute maps for
//! hydration or serialized into deterministic HTML for SSR.

use std::collections::BTreeMap;
use std::fmt;
use std::sync::{Arc, OnceLock};

use rustic_ui_styled_engine::Style;
use rustic_ui_utils::attributes_to_html;

use crate::style_helpers;

/// Global hook signature used to customize selection control builders.
pub type SelectionControlHook =
    dyn Fn(&mut SelectionControlAttributesBuilder) + Send + Sync + 'static;

/// Global hook signature used to customize radio option builders.
pub type RadioOptionHook = dyn Fn(&mut RadioOptionAttributesBuilder) + Send + Sync + 'static;

/// Global hook signature used to customize radio group builders.
pub type RadioGroupHook = dyn Fn(&mut RadioGroupAttributesBuilder) + Send + Sync + 'static;

static SELECTION_CONTROL_HOOK: OnceLock<Arc<SelectionControlHook>> = OnceLock::new();
static RADIO_OPTION_HOOK: OnceLock<Arc<RadioOptionHook>> = OnceLock::new();
static RADIO_GROUP_HOOK: OnceLock<Arc<RadioGroupHook>> = OnceLock::new();

/// Registers a global selection control hook that runs before finalizing any
/// [`SelectionControlAttributes`].
///
/// This enables centralized analytics/theming providers to inject classes or
/// automation IDs without every adapter repeating that wiring.  The hook is
/// stored in a [`OnceLock`], so the first registration wins and subsequent
/// attempts will return an error to avoid unpredictable overrides in multi-tenant
/// environments.
#[allow(clippy::missing_panics_doc)]
pub fn register_selection_control_hook<F>(hook: F) -> Result<(), &'static str>
where
    F: Fn(&mut SelectionControlAttributesBuilder) + Send + Sync + 'static,
{
    SELECTION_CONTROL_HOOK
        .set(Arc::new(hook))
        .map_err(|_| "selection control hook already registered")
}

/// Registers a global radio option hook.  See [`register_selection_control_hook`]
/// for details on lifecycle guarantees.
pub fn register_radio_option_hook<F>(hook: F) -> Result<(), &'static str>
where
    F: Fn(&mut RadioOptionAttributesBuilder) + Send + Sync + 'static,
{
    RADIO_OPTION_HOOK
        .set(Arc::new(hook))
        .map_err(|_| "radio option hook already registered")
}

/// Registers a global radio group hook.  See [`register_selection_control_hook`]
/// for details on lifecycle guarantees.
pub fn register_radio_group_hook<F>(hook: F) -> Result<(), &'static str>
where
    F: Fn(&mut RadioGroupAttributesBuilder) + Send + Sync + 'static,
{
    RADIO_GROUP_HOOK
        .set(Arc::new(hook))
        .map_err(|_| "radio group hook already registered")
}

/// Data object describing a Material selection control (checkbox/switch).
///
/// The struct is intentionally verbose to provide predictable extension points
/// for enterprise telemetry teams.  Values are stored in deterministic
/// [`BTreeMap`]s so that SSR output is stable across releases.
#[derive(Debug, Clone)]
pub struct SelectionControlAttributes {
    label: String,
    style: Style,
    classes: Vec<String>,
    aria: BTreeMap<String, String>,
    data: BTreeMap<String, String>,
    automation_ids: BTreeMap<String, String>,
    attributes: BTreeMap<String, String>,
}

impl SelectionControlAttributes {
    /// Starts a new builder for a selection control descriptor.
    pub fn builder(label: impl Into<String>, style: Style) -> SelectionControlAttributesBuilder {
        SelectionControlAttributesBuilder::new(label, style)
    }

    /// Returns the label intended for end users and analytics payloads.
    pub fn label(&self) -> &str {
        &self.label
    }

    /// Returns a clone of the Material style handle so adapters can preload CSS.
    pub fn style(&self) -> Style {
        self.style.clone()
    }

    /// Returns all CSS classes that should be applied to the control root.
    pub fn classes(&self) -> &[String] {
        &self.classes
    }

    /// Returns ARIA attributes for accessibility testing and hydration as an
    /// iterator compatible with the legacy descriptor API.
    pub fn aria_attributes(&self) -> impl Iterator<Item = (&str, &str)> {
        self.aria
            .iter()
            .map(|(key, value)| (key.as_str(), value.as_str()))
    }

    /// Direct access to the internal ARIA map for adapters that require owned
    /// metadata.
    pub fn aria_map(&self) -> &BTreeMap<String, String> {
        &self.aria
    }

    /// Returns `data-*` attributes used for analytics and focus management as an
    /// iterator. Automation identifiers are surfaced as `data-automation-*`
    /// entries so existing instrumentation keeps working.
    pub fn data_state_attributes(&self) -> Vec<(String, String)> {
        let mut attrs: Vec<(String, String)> = self
            .data
            .iter()
            .map(|(key, value)| (key.clone(), value.clone()))
            .collect();
        attrs.extend(
            self.automation_ids
                .iter()
                .map(|(key, value)| (format!("data-automation-{key}"), value.clone())),
        );
        attrs
    }

    /// Direct access to the structured data attribute map.
    pub fn data_map(&self) -> &BTreeMap<String, String> {
        &self.data
    }

    /// Returns automation identifiers (exposed as `data-automation-*`).
    pub fn automation_ids(&self) -> &BTreeMap<String, String> {
        &self.automation_ids
    }

    /// Returns additional non-ARIA attributes (e.g. `role`, `tabindex`) as an
    /// iterator for backwards compatibility.
    pub fn standard_attributes(&self) -> impl Iterator<Item = (&str, &str)> {
        self.attributes
            .iter()
            .map(|(key, value)| (key.as_str(), value.as_str()))
    }

    /// Direct access to the passthrough attribute map.
    pub fn extra_attributes(&self) -> &BTreeMap<String, String> {
        &self.attributes
    }

    fn base_attribute_pairs(&self) -> Vec<(String, String)> {
        let mut pairs: Vec<(String, String)> = Vec::new();
        if !self.classes.is_empty() {
            pairs.push(("class".into(), self.classes.join(" ")));
        }
        pairs.extend(self.attributes.iter().map(|(k, v)| (k.clone(), v.clone())));
        pairs.extend(self.aria.iter().map(|(k, v)| (k.clone(), v.clone())));
        pairs.extend(self.data.iter().map(|(k, v)| (k.clone(), v.clone())));
        pairs.extend(
            self.automation_ids
                .iter()
                .map(|(k, v)| (format!("data-automation-{k}"), v.clone())),
        );
        pairs
    }

    /// Returns the themed attribute pairs ready for framework hydration.
    pub fn themed_attributes(&self) -> Vec<(String, String)> {
        style_helpers::themed_attributes(self.style.clone(), self.base_attribute_pairs())
    }

    /// Serializes the control into a `<span>` suitable for SSR pipelines.
    pub fn to_ssr_html(&self) -> String {
        let attrs = self.themed_attributes();
        format!(
            "<span {attrs}>{label}</span>",
            attrs = attributes_to_html(&attrs),
            label = self.label()
        )
    }
}

/// Builder backing [`SelectionControlAttributes`].
#[derive(Debug, Clone)]
pub struct SelectionControlAttributesBuilder {
    label: String,
    style: Style,
    classes: Vec<String>,
    aria: BTreeMap<String, String>,
    data: BTreeMap<String, String>,
    automation_ids: BTreeMap<String, String>,
    attributes: BTreeMap<String, String>,
}

impl SelectionControlAttributesBuilder {
    fn new(label: impl Into<String>, style: Style) -> Self {
        Self {
            label: label.into(),
            style,
            classes: Vec::new(),
            aria: BTreeMap::new(),
            data: BTreeMap::new(),
            automation_ids: BTreeMap::new(),
            attributes: BTreeMap::new(),
        }
    }

    /// Applies a CSS class to the control.  Classes are deduplicated during build.
    pub fn class(mut self, class: impl Into<String>) -> Self {
        self.classes.push(class.into());
        self
    }

    /// Applies multiple classes at once.
    pub fn classes<I, S>(mut self, classes: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.classes.extend(classes.into_iter().map(Into::into));
        self
    }

    /// Records an ARIA attribute.
    pub fn aria(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.aria.insert(key.into(), value.into());
        self
    }

    /// Records a `data-*` attribute.  The caller may omit the `data-` prefix to
    /// reduce boilerplate.
    pub fn data(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        let key = key.into();
        let normalized = if key.starts_with("data-") {
            key
        } else {
            format!("data-{key}")
        };
        self.data.insert(normalized, value.into());
        self
    }

    /// Records an automation identifier surfaced as `data-automation-{key}`.
    pub fn automation_id(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.automation_ids.insert(key.into(), value.into());
        self
    }

    /// Adds any additional attribute that does not fit the categories above.
    pub fn attribute(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.attributes.insert(key.into(), value.into());
        self
    }

    fn apply_global_hook(&mut self) {
        if let Some(hook) = SELECTION_CONTROL_HOOK.get() {
            hook(self);
        }
    }

    /// Finalizes the builder and returns an immutable descriptor.
    pub fn build(mut self) -> SelectionControlAttributes {
        self.apply_global_hook();
        self.classes.sort();
        self.classes.dedup();
        SelectionControlAttributes {
            label: self.label,
            style: self.style,
            classes: self.classes,
            aria: self.aria,
            data: self.data,
            automation_ids: self.automation_ids,
            attributes: self.attributes,
        }
    }
}

/// Data object describing a single radio option.
#[derive(Debug, Clone)]
pub struct RadioOptionAttributes {
    label: String,
    style: Style,
    classes: Vec<String>,
    aria: BTreeMap<String, String>,
    data: BTreeMap<String, String>,
    automation_ids: BTreeMap<String, String>,
    attributes: BTreeMap<String, String>,
}

impl RadioOptionAttributes {
    /// Starts a new builder for a radio option descriptor.
    pub fn builder(label: impl Into<String>, style: Style) -> RadioOptionAttributesBuilder {
        RadioOptionAttributesBuilder::new(label, style)
    }

    /// The label rendered next to the radio option.
    pub fn label(&self) -> &str {
        &self.label
    }

    /// Style handle used for preloading.
    pub fn style(&self) -> Style {
        self.style.clone()
    }

    /// List of classes applied to the option container.
    pub fn classes(&self) -> &[String] {
        &self.classes
    }

    /// Iterator over ARIA attributes retained for the option.
    pub fn aria_attributes(&self) -> impl Iterator<Item = (&str, &str)> {
        self.aria
            .iter()
            .map(|(key, value)| (key.as_str(), value.as_str()))
    }

    /// Owned view of data attributes, including automation identifiers.
    pub fn data_state_attributes(&self) -> Vec<(String, String)> {
        let mut attrs: Vec<(String, String)> = self
            .data
            .iter()
            .map(|(key, value)| (key.clone(), value.clone()))
            .collect();
        attrs.extend(
            self.automation_ids
                .iter()
                .map(|(key, value)| (format!("data-automation-{key}"), value.clone())),
        );
        attrs
    }

    /// Iterator over non-ARIA passthrough attributes.
    pub fn standard_attributes(&self) -> impl Iterator<Item = (&str, &str)> {
        self.attributes
            .iter()
            .map(|(key, value)| (key.as_str(), value.as_str()))
    }

    /// Automation identifiers as a map for enterprise QA harnesses.
    pub fn automation_ids(&self) -> &BTreeMap<String, String> {
        &self.automation_ids
    }

    fn base_attribute_pairs(&self) -> Vec<(String, String)> {
        let mut pairs: Vec<(String, String)> = Vec::new();
        if !self.classes.is_empty() {
            pairs.push(("class".into(), self.classes.join(" ")));
        }
        pairs.extend(self.attributes.iter().map(|(k, v)| (k.clone(), v.clone())));
        pairs.extend(self.aria.iter().map(|(k, v)| (k.clone(), v.clone())));
        pairs.extend(self.data.iter().map(|(k, v)| (k.clone(), v.clone())));
        pairs.extend(
            self.automation_ids
                .iter()
                .map(|(k, v)| (format!("data-automation-{k}"), v.clone())),
        );
        pairs
    }

    /// Returns the themed attribute pairs for the option container.
    pub fn themed_attributes(&self) -> Vec<(String, String)> {
        style_helpers::themed_attributes(self.style.clone(), self.base_attribute_pairs())
    }

    /// Serializes the option to `<span>` markup for SSR.
    pub fn to_ssr_html(&self) -> String {
        let attrs = self.themed_attributes();
        format!(
            "<span {attrs}>{label}</span>",
            attrs = attributes_to_html(&attrs),
            label = self.label()
        )
    }
}

/// Builder for [`RadioOptionAttributes`].
#[derive(Debug, Clone)]
pub struct RadioOptionAttributesBuilder {
    label: String,
    style: Style,
    classes: Vec<String>,
    aria: BTreeMap<String, String>,
    data: BTreeMap<String, String>,
    automation_ids: BTreeMap<String, String>,
    attributes: BTreeMap<String, String>,
}

impl RadioOptionAttributesBuilder {
    fn new(label: impl Into<String>, style: Style) -> Self {
        Self {
            label: label.into(),
            style,
            classes: Vec::new(),
            aria: BTreeMap::new(),
            data: BTreeMap::new(),
            automation_ids: BTreeMap::new(),
            attributes: BTreeMap::new(),
        }
    }

    /// Applies a CSS class.
    pub fn class(mut self, class: impl Into<String>) -> Self {
        self.classes.push(class.into());
        self
    }

    /// Applies multiple classes.
    pub fn classes<I, S>(mut self, classes: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.classes.extend(classes.into_iter().map(Into::into));
        self
    }

    /// Adds an ARIA attribute.
    pub fn aria(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.aria.insert(key.into(), value.into());
        self
    }

    /// Adds a `data-*` attribute, prefixing if necessary.
    pub fn data(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        let key = key.into();
        let normalized = if key.starts_with("data-") {
            key
        } else {
            format!("data-{key}")
        };
        self.data.insert(normalized, value.into());
        self
    }

    /// Adds an automation identifier.
    pub fn automation_id(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.automation_ids.insert(key.into(), value.into());
        self
    }

    /// Adds a passthrough attribute.
    pub fn attribute(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.attributes.insert(key.into(), value.into());
        self
    }

    fn apply_global_hook(&mut self) {
        if let Some(hook) = RADIO_OPTION_HOOK.get() {
            hook(self);
        }
    }

    /// Finalizes the descriptor.
    pub fn build(mut self) -> RadioOptionAttributes {
        self.apply_global_hook();
        self.classes.sort();
        self.classes.dedup();
        RadioOptionAttributes {
            label: self.label,
            style: self.style,
            classes: self.classes,
            aria: self.aria,
            data: self.data,
            automation_ids: self.automation_ids,
            attributes: self.attributes,
        }
    }
}

/// Data object describing a radio group container and its options.
#[derive(Debug, Clone)]
pub struct RadioGroupAttributes {
    style: Style,
    classes: Vec<String>,
    aria: BTreeMap<String, String>,
    data: BTreeMap<String, String>,
    automation_ids: BTreeMap<String, String>,
    attributes: BTreeMap<String, String>,
    options: Vec<RadioOptionAttributes>,
}

impl RadioGroupAttributes {
    /// Starts a new builder for a radio group descriptor.
    pub fn builder(style: Style) -> RadioGroupAttributesBuilder {
        RadioGroupAttributesBuilder::new(style)
    }

    /// Returns all option descriptors, preserving the insertion order.
    pub fn options(&self) -> &[RadioOptionAttributes] {
        &self.options
    }

    /// Returns the style handle for the group container.
    pub fn style(&self) -> Style {
        self.style.clone()
    }

    /// Iterator over ARIA attributes retained for the group container.
    pub fn aria_attributes(&self) -> impl Iterator<Item = (&str, &str)> {
        self.aria
            .iter()
            .map(|(key, value)| (key.as_str(), value.as_str()))
    }

    /// Owned data attributes, including automation identifiers.
    pub fn data_state_attributes(&self) -> Vec<(String, String)> {
        let mut attrs: Vec<(String, String)> = self
            .data
            .iter()
            .map(|(key, value)| (key.clone(), value.clone()))
            .collect();
        attrs.extend(
            self.automation_ids
                .iter()
                .map(|(key, value)| (format!("data-automation-{key}"), value.clone())),
        );
        attrs
    }

    /// Iterator over passthrough attributes such as `role` or `tabindex`.
    pub fn standard_attributes(&self) -> impl Iterator<Item = (&str, &str)> {
        self.attributes
            .iter()
            .map(|(key, value)| (key.as_str(), value.as_str()))
    }

    /// Automation identifiers stored for QA harnesses.
    pub fn automation_ids(&self) -> &BTreeMap<String, String> {
        &self.automation_ids
    }

    fn base_attribute_pairs(&self) -> Vec<(String, String)> {
        let mut pairs: Vec<(String, String)> = Vec::new();
        if !self.classes.is_empty() {
            pairs.push(("class".into(), self.classes.join(" ")));
        }
        pairs.extend(self.attributes.iter().map(|(k, v)| (k.clone(), v.clone())));
        pairs.extend(self.aria.iter().map(|(k, v)| (k.clone(), v.clone())));
        pairs.extend(self.data.iter().map(|(k, v)| (k.clone(), v.clone())));
        pairs.extend(
            self.automation_ids
                .iter()
                .map(|(k, v)| (format!("data-automation-{k}"), v.clone())),
        );
        pairs
    }

    /// Returns themed attributes for the radio group container.
    pub fn themed_attributes(&self) -> Vec<(String, String)> {
        style_helpers::themed_attributes(self.style.clone(), self.base_attribute_pairs())
    }

    /// Serializes the entire group to `<div>` markup for SSR.
    pub fn to_ssr_html(&self) -> String {
        let group_attrs = self.themed_attributes();
        let mut options_html = String::new();
        for option in &self.options {
            options_html.push_str(&option.to_ssr_html());
        }
        format!(
            "<div {attrs}>{options}</div>",
            attrs = attributes_to_html(&group_attrs),
            options = options_html
        )
    }
}

/// Builder for [`RadioGroupAttributes`].
#[derive(Debug, Clone)]
pub struct RadioGroupAttributesBuilder {
    style: Style,
    classes: Vec<String>,
    aria: BTreeMap<String, String>,
    data: BTreeMap<String, String>,
    automation_ids: BTreeMap<String, String>,
    attributes: BTreeMap<String, String>,
    options: Vec<RadioOptionAttributes>,
}

impl RadioGroupAttributesBuilder {
    fn new(style: Style) -> Self {
        Self {
            style,
            classes: Vec::new(),
            aria: BTreeMap::new(),
            data: BTreeMap::new(),
            automation_ids: BTreeMap::new(),
            attributes: BTreeMap::new(),
            options: Vec::new(),
        }
    }

    /// Applies a CSS class to the group container.
    pub fn class(mut self, class: impl Into<String>) -> Self {
        self.classes.push(class.into());
        self
    }

    /// Applies multiple CSS classes to the group container.
    pub fn classes<I, S>(mut self, classes: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.classes.extend(classes.into_iter().map(Into::into));
        self
    }

    /// Adds an ARIA attribute to the group container.
    pub fn aria(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.aria.insert(key.into(), value.into());
        self
    }

    /// Adds a `data-*` attribute to the group container.
    pub fn data(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        let key = key.into();
        let normalized = if key.starts_with("data-") {
            key
        } else {
            format!("data-{key}")
        };
        self.data.insert(normalized, value.into());
        self
    }

    /// Adds an automation identifier that will be surfaced as `data-automation-*`.
    pub fn automation_id(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.automation_ids.insert(key.into(), value.into());
        self
    }

    /// Adds a passthrough attribute such as `role` or `tabindex`.
    pub fn attribute(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.attributes.insert(key.into(), value.into());
        self
    }

    /// Appends an option descriptor that was previously built.
    pub fn option(mut self, option: RadioOptionAttributes) -> Self {
        self.options.push(option);
        self
    }

    fn apply_global_hook(&mut self) {
        if let Some(hook) = RADIO_GROUP_HOOK.get() {
            hook(self);
        }
    }

    /// Finalizes the descriptor.
    pub fn build(mut self) -> RadioGroupAttributes {
        self.apply_global_hook();
        self.classes.sort();
        self.classes.dedup();
        RadioGroupAttributes {
            style: self.style,
            classes: self.classes,
            aria: self.aria,
            data: self.data,
            automation_ids: self.automation_ids,
            attributes: self.attributes,
            options: self.options,
        }
    }
}

impl fmt::Display for SelectionControlAttributes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.to_ssr_html())
    }
}

impl fmt::Display for RadioOptionAttributes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.to_ssr_html())
    }
}

impl fmt::Display for RadioGroupAttributes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.to_ssr_html())
    }
}

#[cfg(test)]
mod internal_tests {
    use super::*;

    fn style() -> Style {
        Style::new(rustic_ui_styled_engine::css!("color: inherit;")).expect("valid style")
    }

    #[test]
    fn selection_control_ssr_contains_expected_attributes() {
        let descriptor = SelectionControlAttributes::builder("Toggle", style())
            .class("root")
            .aria("role", "switch")
            .data("state", "on")
            .automation_id("qa", "toggle-1")
            .build();

        let html = descriptor.to_ssr_html();
        assert!(html.contains("role=\"switch\""));
        assert!(html.contains("data-state=\"on\""));
        assert!(html.contains("data-automation-qa=\"toggle-1\""));
        assert!(html.contains("Toggle"));
    }

    #[test]
    fn radio_group_ssr_contains_option_markup() {
        let option_style = style();
        let option = RadioOptionAttributes::builder("A", option_style.clone())
            .aria("role", "radio")
            .automation_id("qa", "radio-a")
            .build();
        let group = RadioGroupAttributes::builder(style())
            .aria("role", "radiogroup")
            .option(option)
            .build();

        let html = group.to_ssr_html();
        assert!(html.contains("role=\"radiogroup\""));
        assert!(html.contains("radio-a"));
    }
}
