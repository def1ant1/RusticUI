//! # Selection control attribute builders
//!
//! Enterprise-grade Material selection controls in RusticUI centralise all of
//! their SSR, hydration, analytics, and automation state inside the strongly
//! typed builders defined in this module. The builders back
//! [`crate::checkbox`], [`crate::radio`], and [`crate::switch`], ensuring that
//! every adapter (React, Yew, Leptos, Dioxus, Sycamore, or pure SSR) consumes a
//! single descriptor snapshot derived from the headless state machines and the
//! shared telemetry contract.
//!
//! ## Keyboard navigation & focus-visible semantics
//!
//! Keyboard orchestration flows through the headless state machines exposed in
//! `rustic-ui-headless`. For example,
//! [`CheckboxState::on_key`](rustic_ui_headless::checkbox::CheckboxState::on_key),
//! [`RadioGroupState::on_key`](rustic_ui_headless::radio::RadioGroupState::on_key),
//! and [`SwitchState::on_key`](rustic_ui_headless::switch::SwitchState::on_key)
//! consume [`ControlKey`](rustic_ui_headless::interaction::ControlKey) inputs to
//! mirror the Material keyboard specification. The resulting focus-visible flag
//! is surfaced via
//! [`CheckboxState::focus_visible`](rustic_ui_headless::checkbox::CheckboxState::focus_visible),
//! [`RadioGroupState::focus_visible_index`](rustic_ui_headless::radio::RadioGroupState::focus_visible_index),
//! and [`SwitchState::focus_visible`](rustic_ui_headless::switch::SwitchState::focus_visible).
//! Builders lift those signals into `data-focus-visible` attributes so CSS
//! recipes in [`crate::checkbox`], [`crate::radio`], and [`crate::switch`] can
//! render WCAG-compliant focus rings only when the `:focus-visible` pseudo
//! class would have applied. This keeps keyboard navigation predictable across
//! SSR and hydration without forcing each adapter to reinvent the focus
//! semantics.
//!
//! ## Automation and telemetry hooks
//!
//! Global configuration hooks—[`register_selection_control_hook`],
//! [`register_radio_option_hook`], and [`register_radio_group_hook`]—allow
//! platform teams to inject analytics identifiers, automation selectors, or
//! custom ARIA attributes once per process. Hooks execute before the descriptor
//! finalises, so any metadata added by the hook participates in SSR and
//! hydration identically. They complement [`TelemetryHooks`](crate::telemetry::TelemetryHooks),
//! which downstream adapters use to emit spans, analytics payloads, focus
//! transitions, and state-change beacons through a centralised observability
//! pipeline. Because the hooks are stored in [`OnceLock`], registration is
//! deterministic even when multiple frameworks initialise concurrently.
//!
//! ## SSR and hydration guarantees
//!
//! Every builder records classes, ARIA metadata, and automation identifiers in
//! [`BTreeMap`]s to guarantee deterministic key ordering. The resulting
//! [`SelectionControlAttributes::themed_attributes`] output matches the themed
//! spreads consumed by client adapters and the
//! [`SelectionControlAttributes::to_ssr_html`] HTML serialiser. That symmetry
//! prevents checksum drift during hydration and ensures analytics selectors (for
//! example `data-automation-*`) match exactly between the SSR fragment and the
//! hydrated widget. Radio groups reuse [`RadioOptionAttributes`] for each option
//! so the option metadata flows through the same contract.
//!
//! ## Failure modes and operational safeguards
//!
//! * Hook registration functions return `Err` when invoked more than once. This
//!   guards against data races in shared bootstrap code and avoids overwriting
//!   production instrumentation unexpectedly.
//! * [`Style::new`](rustic_ui_styled_engine::Style::new) returns a `Result`; the
//!   builders assume the caller propagated errors emitted by the CSS compiler.
//! * Builders deduplicate CSS classes on build to prevent runaway SSR payloads
//!   when multiple adapters compose styles.
//! * State-machine guarded methods such as
//!   [`RadioGroupState::select`](rustic_ui_headless::radio::RadioGroupState::select)
//!   are no-ops when provided out-of-range indices or when the group is disabled,
//!   so hydration replays cannot inadvertently select a disallowed option.
//!
//! ## Examples
//!
//! ### Enterprise bootstrap across SSR and hydration
//!
//! ```rust,ignore
//! use std::sync::{Arc, OnceLock};
//!
//! use rustic_ui_material::selection_control::{
//!     register_selection_control_hook, SelectionControlAttributes,
//! };
//! use rustic_ui_material::telemetry::TelemetryHooks;
//! use rustic_ui_styled_engine::{css, Style};
//!
//! static TELEMETRY: OnceLock<Arc<TelemetryHooks>> = OnceLock::new();
//!
//! fn telemetry() -> &'static Arc<TelemetryHooks> {
//!     TELEMETRY.get_or_init(|| {
//!         let mut hooks = TelemetryHooks::default();
//!         hooks.analytics_id = Some("controls.enterprise".into());
//!         hooks.automation_id = Some("global-selection-control".into());
//!         Arc::new(hooks)
//!     })
//! }
//!
//! fn server_fragment() -> String {
//!     register_selection_control_hook(|builder| {
//!         builder
//!             .data("tenant", "platform-a")
//!             .automation_id("suite", "selection-controls");
//!     })
//!     .expect("hook installs only once");
//!
//!     SelectionControlAttributes::builder(
//!         "Receive product updates",
//!         Style::new(css!("display: inline-flex; align-items: center;")).unwrap(),
//!     )
//!     .attribute("role", "switch")
//!     .build()
//!     .to_ssr_html()
//! }
//!
//! fn hydrate_focus_state() {
//!     let hooks = telemetry();
//!     // Downstream adapters (e.g. `crate::switch::react`) call into
//!     // `instrument_render` with these hooks so analytics, focus transitions,
//!     // and automation metadata are replayed consistently on the client.
//!     assert!(hooks.analytics_id.as_deref() == Some("controls.enterprise"));
//! }
//! ```
//!
//! ### Coordinating radio telemetry with shared state machines
//!
//! ```rust,ignore
//! use std::sync::{Arc, OnceLock};
//!
//! use rustic_ui_headless::interaction::ControlKey;
//! use rustic_ui_headless::radio::{RadioGroupState, RadioOrientation};
//! use rustic_ui_material::selection_control::{
//!     register_radio_group_hook, RadioGroupAttributes, RadioOptionAttributes,
//! };
//! use rustic_ui_material::telemetry::{
//!     TelemetryContext, TelemetryHooks, TelemetryStateChangePayload,
//! };
//! use rustic_ui_styled_engine::{css, Style};
//!
//! static TELEMETRY: OnceLock<Arc<TelemetryHooks>> = OnceLock::new();
//!
//! fn telemetry() -> &'static Arc<TelemetryHooks> {
//!     TELEMETRY.get_or_init(|| {
//!         let mut hooks = TelemetryHooks::default();
//!         hooks.analytics_id = Some("controls.payment-method".into());
//!         hooks.automation_id = Some("payment-method".into());
//!         Arc::new(hooks)
//!     })
//! }
//!
//! fn server_radio_fragment() -> String {
//!     register_radio_group_hook(|builder| {
//!         builder.data("deployment", "checkout-edge");
//!     })
//!     .expect("hook installs only once");
//!
//!     RadioGroupAttributes::builder(
//!         Style::new(css!("display: flex; gap: var(--size-2); align-items: center;")).unwrap(),
//!     )
//!     .option(
//!         RadioOptionAttributes::builder("Visa", Style::new(css!("padding: 0.5rem;")).unwrap())
//!             .aria("role", "radio")
//!             .build(),
//!     )
//!     .option(
//!         RadioOptionAttributes::builder("Direct debit", Style::new(css!("padding: 0.5rem;")).unwrap())
//!             .aria("role", "radio")
//!             .build(),
//!     )
//!     .build()
//!     .to_ssr_html()
//! }
//!
//! fn client_radio_interaction() {
//!     let hooks = telemetry().clone();
//!     let mut state = RadioGroupState::uncontrolled(
//!         ["Visa".into(), "Direct debit".into(), "Wire transfer".into()],
//!         false,
//!         RadioOrientation::Horizontal,
//!         Some(0),
//!     );
//!     state.on_key(ControlKey::ArrowRight, move |index| {
//!         if let Some(callback) = &hooks.on_state_change {
//!             let payload = TelemetryStateChangePayload {
//!                 previous: "visa".into(),
//!                 next: format!("option-{index}"),
//!                 ..TelemetryStateChangePayload::default()
//!             };
//!             callback(
//!                 TelemetryContext::new("radio.payment-method")
//!                     .with_analytics(hooks.analytics_id.clone())
//!                     .with_automation(hooks.automation_id.clone()),
//!                 payload,
//!             );
//!         }
//!     });
//! }
//! ```
//!
//! ## See also
//!
//! * [`crate::checkbox`], [`crate::radio`], and [`crate::switch`] for concrete
//!   adapters that consume these builders.
//! * [`rustic_ui_headless::checkbox::CheckboxState`],
//!   [`rustic_ui_headless::radio::RadioGroupState`], and
//!   [`rustic_ui_headless::switch::SwitchState`] for the interaction state
//!   machines driving keyboard navigation and focus.
//! * See the `telemetry` module for the full instrumentation surface used
//!   throughout RusticUI selection controls.

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
