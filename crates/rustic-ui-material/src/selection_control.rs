//! Shared rendering helpers for Material selection controls.
//!
//! The helpers now surface structured descriptors so adapters can decide whether
//! to emit HTML strings (SSR) or hydrate individual frameworks with attribute
//! maps.  Centralizing the metadata keeps the individual component modules
//! focused on data flow rather than DOM string assembly while making it trivial
//! to serialize the same descriptor in multiple environments.

use rustic_ui_styled_engine::Style;
use rustic_ui_utils::attributes_to_html;

use crate::style_helpers;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AttributeKind {
    Aria,
    DataState,
    Standard,
}

#[derive(Debug, Clone)]
struct AttributeEntry {
    key: String,
    value: String,
    kind: AttributeKind,
}

impl AttributeEntry {
    fn new(key: impl Into<String>, value: impl Into<String>) -> Self {
        let key = key.into();
        let kind = if key.starts_with("aria-") || key == "role" || key == "tabindex" {
            AttributeKind::Aria
        } else if key.starts_with("data-") {
            AttributeKind::DataState
        } else {
            AttributeKind::Standard
        };
        Self {
            key,
            value: value.into(),
            kind,
        }
    }
}

/// Describes a labelled toggle-style control such as a checkbox or switch.
///
/// The descriptor acts as a fluent builder that records ARIA attributes,
/// automation-focused `data-*` flags, and additional DOM metadata without
/// stringifying. Framework adapters can request the themed attribute pairs when
/// hydrating a client render, while SSR pipelines can convert the same
/// descriptor to HTML using [`render_toggle_html`].
///
/// # SSR and hydration flow
///
/// 1. Headless state machines expose attribute tuples describing ARIA metadata
///    and stateful `data-*` flags.
/// 2. Callers feed those tuples into [`ToggleControlDescriptor::with_attributes`]
///    which records the metadata and allows inspection without conversion.
/// 3. Framework adapters call [`ToggleControlDescriptor::themed_attributes`] to
///    receive a merged `(String, String)` vector suitable for frameworks like
///    Yew, Leptos, Sycamore or React (through the WASM bridge).
/// 4. Server renderers invoke [`render_toggle_html`] which serializes the same
///    descriptor via [`style_helpers::themed_attributes`] and
///    [`attributes_to_html`].
///
/// # Examples
///
/// ```rust,ignore
/// use rustic_ui_material::selection_control::{
///     render_toggle_html, ToggleControlDescriptor,
/// };
/// use rustic_ui_styled_engine::Style;
///
/// let style = Style::new(rustic_ui_styled_engine::css!("color: red;"))
///     .expect("valid style");
/// let descriptor = ToggleControlDescriptor::new("Notifications", style.clone())
///     .with_attributes([
///         ("role", "switch"),
///         ("aria-checked", "true"),
///         ("data-on", "true"),
///     ]);
///
/// // Framework adapter hydration:
/// let themed_pairs = descriptor.themed_attributes();
/// assert!(themed_pairs.iter().any(|(k, _)| k == "class"));
///
/// // SSR pipeline:
/// let html = render_toggle_html(&descriptor);
/// assert!(html.contains("aria-checked"));
/// ```
#[derive(Debug, Clone)]
pub struct ToggleControlDescriptor {
    label: String,
    style: Style,
    attributes: Vec<AttributeEntry>,
}

#[allow(dead_code)]
impl ToggleControlDescriptor {
    /// Create a new descriptor for a toggle-like control.
    pub fn new(label: impl Into<String>, style: Style) -> Self {
        Self {
            label: label.into(),
            style,
            attributes: Vec::new(),
        }
    }

    /// Returns the visible label associated with the control.
    pub fn label(&self) -> &str {
        &self.label
    }

    /// Returns a clone of the themed style handle so adapters can pre-register
    /// CSS with their runtime.
    pub fn style(&self) -> Style {
        self.style.clone()
    }

    /// Iterate over ARIA attributes recorded on the descriptor.
    pub fn aria_attributes(&self) -> impl Iterator<Item = (&str, &str)> {
        self.attributes
            .iter()
            .filter(|attr| attr.kind == AttributeKind::Aria)
            .map(|attr| (attr.key.as_str(), attr.value.as_str()))
    }

    /// Iterate over `data-*` attributes tracked by the descriptor.
    pub fn data_state_attributes(&self) -> impl Iterator<Item = (&str, &str)> {
        self.attributes
            .iter()
            .filter(|attr| attr.kind == AttributeKind::DataState)
            .map(|attr| (attr.key.as_str(), attr.value.as_str()))
    }

    /// Iterate over non-ARIA, non-`data-*` attributes.
    pub fn standard_attributes(&self) -> impl Iterator<Item = (&str, &str)> {
        self.attributes
            .iter()
            .filter(|attr| attr.kind == AttributeKind::Standard)
            .map(|attr| (attr.key.as_str(), attr.value.as_str()))
    }

    /// Append raw attributes emitted by the headless state machine.
    pub fn with_attributes<I, K, V>(mut self, attributes: I) -> Self
    where
        I: IntoIterator<Item = (K, V)>,
        K: Into<String>,
        V: Into<String>,
    {
        for (key, value) in attributes {
            self.attributes.push(AttributeEntry::new(key, value));
        }
        self
    }

    /// Add a single attribute to the descriptor.
    pub fn attribute(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.attributes.push(AttributeEntry::new(key, value));
        self
    }

    /// Return the merged themed attributes ready for framework hydration.
    pub fn themed_attributes(&self) -> Vec<(String, String)> {
        style_helpers::themed_attributes(self.style.clone(), self.raw_attributes())
    }

    fn raw_attributes(&self) -> Vec<(String, String)> {
        let mut standard = Vec::new();
        let mut aria = Vec::new();
        let mut data = Vec::new();

        for attr in &self.attributes {
            let pair = (attr.key.clone(), attr.value.clone());
            match attr.kind {
                AttributeKind::Standard => standard.push(pair),
                AttributeKind::Aria => aria.push(pair),
                AttributeKind::DataState => data.push(pair),
            }
        }

        standard.extend(aria);
        standard.extend(data);
        standard
    }
}

/// Describes an individual option within a radio group.
///
/// The descriptor mirrors [`ToggleControlDescriptor`] but focuses on options
/// that share a group container.  Each option tracks its own theme handle,
/// attribute metadata and label so adapters can compose option-level DOM nodes
/// independently of the surrounding group.
#[derive(Debug, Clone)]
pub struct RadioOptionDescriptor {
    label: String,
    style: Style,
    attributes: Vec<AttributeEntry>,
}

#[allow(dead_code)]
impl RadioOptionDescriptor {
    /// Create a new option descriptor.
    pub fn new(label: impl Into<String>, style: Style) -> Self {
        Self {
            label: label.into(),
            style,
            attributes: Vec::new(),
        }
    }

    /// Returns the option label displayed to end users.
    pub fn label(&self) -> &str {
        &self.label
    }

    /// Returns a clone of the themed style handle for the option container.
    pub fn style(&self) -> Style {
        self.style.clone()
    }

    /// Iterate over ARIA metadata for the option.
    pub fn aria_attributes(&self) -> impl Iterator<Item = (&str, &str)> {
        self.attributes
            .iter()
            .filter(|attr| attr.kind == AttributeKind::Aria)
            .map(|attr| (attr.key.as_str(), attr.value.as_str()))
    }

    /// Iterate over `data-*` flags exposed for analytics and focus management.
    pub fn data_state_attributes(&self) -> impl Iterator<Item = (&str, &str)> {
        self.attributes
            .iter()
            .filter(|attr| attr.kind == AttributeKind::DataState)
            .map(|attr| (attr.key.as_str(), attr.value.as_str()))
    }

    /// Iterate over non-ARIA attributes.
    pub fn standard_attributes(&self) -> impl Iterator<Item = (&str, &str)> {
        self.attributes
            .iter()
            .filter(|attr| attr.kind == AttributeKind::Standard)
            .map(|attr| (attr.key.as_str(), attr.value.as_str()))
    }

    /// Append attributes retrieved from the radio state machine.
    pub fn with_attributes<I, K, V>(mut self, attributes: I) -> Self
    where
        I: IntoIterator<Item = (K, V)>,
        K: Into<String>,
        V: Into<String>,
    {
        for (key, value) in attributes {
            self.attributes.push(AttributeEntry::new(key, value));
        }
        self
    }

    /// Add a single attribute to the descriptor.
    pub fn attribute(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.attributes.push(AttributeEntry::new(key, value));
        self
    }

    /// Return the themed attribute pairs for hydration.
    pub fn themed_attributes(&self) -> Vec<(String, String)> {
        style_helpers::themed_attributes(self.style.clone(), self.raw_attributes())
    }

    fn raw_attributes(&self) -> Vec<(String, String)> {
        let mut standard = Vec::new();
        let mut aria = Vec::new();
        let mut data = Vec::new();

        for attr in &self.attributes {
            let pair = (attr.key.clone(), attr.value.clone());
            match attr.kind {
                AttributeKind::Standard => standard.push(pair),
                AttributeKind::Aria => aria.push(pair),
                AttributeKind::DataState => data.push(pair),
            }
        }

        standard.extend(aria);
        standard.extend(data);
        standard
    }
}

/// Describes the container and options for a radio group.
///
/// The descriptor is intentionally exhaustive, allowing SSR callers to generate
/// HTML via [`render_radio_group_html`] while giving client renderers direct
/// access to the themed attribute sets.  Options are stored as
/// [`RadioOptionDescriptor`] instances so adapters can lazily render only the
/// portions of the group that changed during hydration.
///
/// # Examples
///
/// ```rust,ignore
/// use rustic_ui_material::selection_control::{
///     render_radio_group_html, RadioGroupDescriptor, RadioOptionDescriptor,
/// };
/// use rustic_ui_styled_engine::Style;
///
/// let group_style = Style::new(rustic_ui_styled_engine::css!("display: flex;"))
///     .expect("valid style");
/// let option_style = group_style.clone();
/// let descriptor = RadioGroupDescriptor::new(group_style)
///     .with_group_attributes([("role", "radiogroup")])
///     .option(
///         RadioOptionDescriptor::new("A", option_style.clone()).with_attributes([
///             ("role", "radio"),
///             ("aria-checked", "true"),
///         ]),
///     )
///     .option(
///         RadioOptionDescriptor::new("B", option_style).with_attributes([
///             ("role", "radio"),
///             ("aria-checked", "false"),
///         ]),
///     );
///
/// // Hydration fetches the attribute pairs lazily.
/// let group_pairs = descriptor.group_thematic_attributes();
/// assert!(group_pairs.iter().any(|(k, _)| k == "class"));
///
/// // SSR renders deterministic markup.
/// let html = render_radio_group_html(&descriptor);
/// assert!(html.contains("role=\"radiogroup\""));
/// ```
#[derive(Debug, Clone)]
pub struct RadioGroupDescriptor {
    style: Style,
    attributes: Vec<AttributeEntry>,
    options: Vec<RadioOptionDescriptor>,
}

#[allow(dead_code)]
impl RadioGroupDescriptor {
    /// Create a new descriptor for a radio group container.
    pub fn new(style: Style) -> Self {
        Self {
            style,
            attributes: Vec::new(),
            options: Vec::new(),
        }
    }

    /// Append attributes emitted by the headless radio group state.
    pub fn with_group_attributes<I, K, V>(mut self, attributes: I) -> Self
    where
        I: IntoIterator<Item = (K, V)>,
        K: Into<String>,
        V: Into<String>,
    {
        for (key, value) in attributes {
            self.attributes.push(AttributeEntry::new(key, value));
        }
        self
    }

    /// Add an individual attribute to the group container.
    pub fn group_attribute(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.attributes.push(AttributeEntry::new(key, value));
        self
    }

    /// Push a radio option descriptor into the group.
    pub fn option(mut self, option: RadioOptionDescriptor) -> Self {
        self.options.push(option);
        self
    }

    /// Returns all option descriptors for further inspection.
    pub fn options(&self) -> &[RadioOptionDescriptor] {
        &self.options
    }

    /// Returns a clone of the group style for preloading CSS rules.
    pub fn style(&self) -> Style {
        self.style.clone()
    }

    /// Iterate over ARIA attributes attached to the group container.
    pub fn aria_attributes(&self) -> impl Iterator<Item = (&str, &str)> {
        self.attributes
            .iter()
            .filter(|attr| attr.kind == AttributeKind::Aria)
            .map(|attr| (attr.key.as_str(), attr.value.as_str()))
    }

    /// Iterate over container-level `data-*` flags.
    pub fn data_state_attributes(&self) -> impl Iterator<Item = (&str, &str)> {
        self.attributes
            .iter()
            .filter(|attr| attr.kind == AttributeKind::DataState)
            .map(|attr| (attr.key.as_str(), attr.value.as_str()))
    }

    /// Iterate over standard container attributes.
    pub fn standard_attributes(&self) -> impl Iterator<Item = (&str, &str)> {
        self.attributes
            .iter()
            .filter(|attr| attr.kind == AttributeKind::Standard)
            .map(|attr| (attr.key.as_str(), attr.value.as_str()))
    }

    /// Returns the themed attribute pairs for the group container.
    pub fn group_thematic_attributes(&self) -> Vec<(String, String)> {
        style_helpers::themed_attributes(self.style.clone(), self.raw_attributes())
    }

    fn raw_attributes(&self) -> Vec<(String, String)> {
        let mut standard = Vec::new();
        let mut aria = Vec::new();
        let mut data = Vec::new();

        for attr in &self.attributes {
            let pair = (attr.key.clone(), attr.value.clone());
            match attr.kind {
                AttributeKind::Standard => standard.push(pair),
                AttributeKind::Aria => aria.push(pair),
                AttributeKind::DataState => data.push(pair),
            }
        }

        standard.extend(aria);
        standard.extend(data);
        standard
    }
}

/// Serialize a toggle descriptor into HTML for SSR environments.
#[must_use]
pub(crate) fn render_toggle_html(descriptor: &ToggleControlDescriptor) -> String {
    let attrs = descriptor.themed_attributes();
    format!(
        "<span {attrs}>{label}</span>",
        attrs = attributes_to_html(&attrs),
        label = descriptor.label()
    )
}

/// Serialize a radio group descriptor into HTML for SSR environments.
#[must_use]
pub(crate) fn render_radio_group_html(descriptor: &RadioGroupDescriptor) -> String {
    let group_attrs = descriptor.group_thematic_attributes();
    let mut options_html = String::new();
    for option in descriptor.options() {
        let attrs = option.themed_attributes();
        options_html.push_str(&format!(
            "<span {attrs}>{label}</span>",
            attrs = attributes_to_html(&attrs),
            label = option.label()
        ));
    }
    format!(
        "<div {attrs}>{options}</div>",
        attrs = attributes_to_html(&group_attrs),
        options = options_html
    )
}
