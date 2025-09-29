#![deny(missing_docs)]
//! Headless breadcrumb controller that centralises ARIA, keyboard behaviour,
//! and analytics hooks.
//!
//! The state machine is intentionally declarative: adapters describe the number
//! of crumbs, mark the active page, and optionally supply automation/analytics
//! identifiers.  Keyboard navigation mirrors WAI-ARIA authoring practices for
//! breadcrumb trails by supporting horizontal traversal with <Left>/<Right>,
//! quick jumps via <Home>/<End>, and activation through <Enter>/<Space>.  The
//! resulting telemetry payload keeps audit logs consistent across frameworks
//! without leaking implementation details into application code.

use crate::{
    aria,
    interaction::ControlKey,
    selection::{clamp_index, wrap_index, ControlStrategy},
};

/// Analytics payload describing user interaction with the breadcrumb trail.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BreadcrumbsAnalyticsEvent {
    /// Logical telemetry channel surfaced via `data-rustic-analytics-channel`.
    pub channel: String,
    /// Zero-based index of the activated crumb.
    pub index: usize,
    /// Optional crumb specific tag configured by the adapter.
    pub crumb_tag: Option<String>,
}

/// Outcome produced after processing a keyboard event.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct BreadcrumbsKeyboardOutcome {
    /// Index that should receive focus.
    pub focused: Option<usize>,
    /// Index that should be activated (navigated to).
    pub activated: Option<usize>,
    /// Analytics payload describing the activation.
    pub analytics: Option<BreadcrumbsAnalyticsEvent>,
}

/// Builder describing the `<nav>` wrapper attributes.
#[derive(Debug, Clone)]
pub struct BreadcrumbsRootAttributes<'a> {
    state: &'a BreadcrumbsState,
    id: Option<&'a str>,
    aria_label: Option<&'a str>,
}

impl<'a> BreadcrumbsRootAttributes<'a> {
    /// Construct a new builder bound to the provided state.
    pub fn new(state: &'a BreadcrumbsState) -> Self {
        Self {
            state,
            id: None,
            aria_label: Some("Breadcrumb"),
        }
    }

    /// Assign a DOM id to the navigation region.
    pub fn id(mut self, value: &'a str) -> Self {
        self.id = Some(value);
        self
    }

    /// Override the accessible label announced by assistive technology.
    pub fn aria_label(mut self, value: &'a str) -> Self {
        self.aria_label = Some(value);
        self
    }

    /// Returns the `role` tuple.
    pub fn role(&self) -> &'static str {
        aria::role_navigation()
    }

    /// Optional `id` attribute tuple.
    pub fn id_attr(&self) -> Option<(&'static str, &str)> {
        self.id.map(|value| ("id", value))
    }

    /// Optional `aria-label` tuple.
    pub fn aria_label_attr(&self) -> Option<(&'static str, &str)> {
        self.aria_label.map(|value| ("aria-label", value))
    }

    /// Optional analytics hook attribute.
    pub fn analytics_attribute(&self) -> Option<(&'static str, &str)> {
        self.state
            .analytics_channel
            .as_deref()
            .map(|value| ("data-rustic-analytics-channel", value))
    }
}

/// Builder describing the ordered list wrapper attributes.
#[derive(Debug, Clone)]
pub struct BreadcrumbsListAttributes<'a> {
    state: &'a BreadcrumbsState,
    id: Option<&'a str>,
}

impl<'a> BreadcrumbsListAttributes<'a> {
    /// Create a new builder for the list container.
    pub fn new(state: &'a BreadcrumbsState) -> Self {
        Self { state, id: None }
    }

    /// Assign a DOM id to the list element.
    pub fn id(mut self, value: &'a str) -> Self {
        self.id = Some(value);
        self
    }

    /// Returns the `role` tuple.
    pub fn role(&self) -> &'static str {
        aria::role_list()
    }

    /// Optional `id` attribute tuple.
    pub fn id_attr(&self) -> Option<(&'static str, &str)> {
        self.id.map(|value| ("id", value))
    }

    /// Optional analytics attribute mirrored from the root.
    pub fn analytics_attribute(&self) -> Option<(&'static str, &str)> {
        self.state
            .analytics_channel
            .as_deref()
            .map(|value| ("data-rustic-analytics-channel", value))
    }
}

/// Builder describing the interactive crumb element.
#[derive(Debug, Clone)]
pub struct BreadcrumbItemAttributes<'a> {
    state: &'a BreadcrumbsState,
    index: usize,
    id: Option<&'a str>,
    href: Option<&'a str>,
    analytics_tag: Option<&'a str>,
}

impl<'a> BreadcrumbItemAttributes<'a> {
    fn new(state: &'a BreadcrumbsState, index: usize) -> Self {
        Self {
            state,
            index,
            id: None,
            href: None,
            analytics_tag: None,
        }
    }

    /// Assign a DOM id to the anchor.
    pub fn id(mut self, value: &'a str) -> Self {
        self.id = Some(value);
        self
    }

    /// Configure the `href` destination.
    pub fn href(mut self, value: &'a str) -> Self {
        self.href = Some(value);
        self
    }

    /// Associate an analytics identifier for the crumb.
    pub fn analytics_tag(mut self, value: &'a str) -> Self {
        self.analytics_tag = Some(value);
        self
    }

    /// Collect the configured attributes into reusable pairs.
    pub fn as_pairs(&self) -> Vec<(&'static str, String)> {
        let mut pairs = Vec::with_capacity(8);
        pairs.push(("role", aria::role_link().to_string()));
        let focused = self.state.focused == Some(self.index);
        pairs.push(("tabindex", if focused { "0" } else { "-1" }.to_string()));
        if let Some(id) = self.id {
            pairs.push(("id", id.to_string()));
        }
        if let Some(href) = self.href {
            pairs.push(("href", href.to_string()));
        }
        if self.state.current == Some(self.index) {
            let (key, value) = aria::aria_current("page");
            pairs.push((key, value.to_string()));
        }
        if let Some(tag) = self.analytics_tag {
            pairs.push(("data-rustic-analytics-id", tag.to_string()));
        }
        pairs.push(("data-rustic-breadcrumb-index", self.index.to_string()));
        if self.state.is_disabled(self.index) {
            pairs.push(("aria-disabled", "true".into()));
            pairs.push(("data-disabled", "true".into()));
        }
        pairs
    }
}

/// Builder describing the separator glyph between crumbs.
#[derive(Debug, Clone, Default)]
pub struct BreadcrumbSeparatorAttributes {
    id: Option<String>,
}

impl BreadcrumbSeparatorAttributes {
    /// Assign a DOM id to the separator for analytics tooling.
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Returns the attributes describing the separator element.
    pub fn as_pairs(&self) -> Vec<(&'static str, String)> {
        let mut pairs = vec![("role", aria::role_separator().to_string())];
        if let Some(id) = &self.id {
            pairs.push(("id", id.clone()));
        }
        pairs
    }
}

/// Headless breadcrumb state machine.
#[derive(Debug, Clone)]
pub struct BreadcrumbsState {
    item_count: usize,
    focused: Option<usize>,
    current: Option<usize>,
    focus_mode: ControlStrategy,
    selection_mode: ControlStrategy,
    analytics_channel: Option<String>,
    analytics_tags: Vec<Option<String>>,
    disabled: Vec<bool>,
}

impl BreadcrumbsState {
    /// Construct a new breadcrumb controller.
    pub fn new(
        item_count: usize,
        current: Option<usize>,
        focus_mode: ControlStrategy,
        selection_mode: ControlStrategy,
    ) -> Self {
        let current = clamp_index(current, item_count);
        let focused = current.or(if item_count > 0 { Some(0) } else { None });
        Self {
            item_count,
            focused,
            current,
            focus_mode,
            selection_mode,
            analytics_channel: None,
            analytics_tags: vec![None; item_count],
            disabled: vec![false; item_count],
        }
    }

    /// Returns the root attribute builder.
    pub fn root_attributes(&self) -> BreadcrumbsRootAttributes<'_> {
        BreadcrumbsRootAttributes::new(self)
    }

    /// Returns the ordered list attribute builder.
    pub fn list_attributes(&self) -> BreadcrumbsListAttributes<'_> {
        BreadcrumbsListAttributes::new(self)
    }

    /// Returns the item attribute builder for the provided index.
    pub fn item_attributes(&self, index: usize) -> BreadcrumbItemAttributes<'_> {
        BreadcrumbItemAttributes::new(self, index)
    }

    /// Returns a separator attribute builder.
    pub fn separator_attributes(&self) -> BreadcrumbSeparatorAttributes {
        BreadcrumbSeparatorAttributes::default()
    }

    /// Update the number of crumbs.
    pub fn set_item_count(&mut self, count: usize) {
        self.item_count = count;
        self.analytics_tags.resize(count, None);
        self.disabled.resize(count, false);
        self.focused = clamp_index(self.focused, count);
        self.current = clamp_index(self.current, count);
    }

    /// Configure the analytics channel.
    pub fn set_analytics_channel(&mut self, channel: Option<impl Into<String>>) {
        self.analytics_channel = channel.map(Into::into);
    }

    /// Configure an analytics tag for the crumb at `index`.
    pub fn set_item_analytics_tag(&mut self, index: usize, tag: Option<impl Into<String>>) {
        if index >= self.item_count {
            return;
        }
        if let Some(slot) = self.analytics_tags.get_mut(index) {
            *slot = tag.map(Into::into);
        }
    }

    /// Mark a crumb as disabled (non-navigable).
    pub fn set_item_disabled(&mut self, index: usize, disabled: bool) {
        if index >= self.item_count {
            return;
        }
        if let Some(slot) = self.disabled.get_mut(index) {
            *slot = disabled;
        }
        if disabled {
            if self.focused == Some(index) {
                self.focused = self.advance(1);
            }
            if self.current == Some(index) {
                self.current = None;
            }
        }
    }

    /// Returns whether the crumb is disabled.
    pub fn is_disabled(&self, index: usize) -> bool {
        self.disabled.get(index).copied().unwrap_or(true)
    }

    /// Synchronise the currently active crumb (controlled mode).
    pub fn sync_current(&mut self, index: Option<usize>) {
        if self.selection_mode.is_controlled() {
            self.current = clamp_index(index, self.item_count);
        }
    }

    /// Synchronise the focused crumb (controlled focus mode).
    pub fn sync_focused(&mut self, index: Option<usize>) {
        if self.focus_mode.is_controlled() {
            self.focused = clamp_index(index, self.item_count);
        }
    }

    /// Handle keyboard interactions and return the resulting intent.
    pub fn on_key<F: FnMut(BreadcrumbsActivation)>(
        &mut self,
        key: ControlKey,
        mut on_activate: F,
    ) -> BreadcrumbsKeyboardOutcome {
        let mut outcome = BreadcrumbsKeyboardOutcome::default();
        match key {
            ControlKey::Enter | ControlKey::Space => {
                if let Some(index) = self.focused {
                    if !self.is_disabled(index) {
                        outcome.activated = Some(index);
                        outcome.analytics = self.apply_activation(index, &mut on_activate);
                    }
                }
            }
            ControlKey::Home => {
                outcome.focused = self.apply_focus(self.first_enabled());
            }
            ControlKey::End => {
                outcome.focused = self.apply_focus(self.last_enabled());
            }
            _ if key.is_forward() => {
                outcome.focused = self.apply_focus(self.advance(1));
            }
            _ if key.is_backward() => {
                outcome.focused = self.apply_focus(self.advance(-1));
            }
            _ => {}
        }
        outcome
    }

    /// Imperatively activate a crumb.
    pub fn activate<F: FnMut(BreadcrumbsActivation)>(
        &mut self,
        index: usize,
        mut on_activate: F,
    ) -> Option<BreadcrumbsAnalyticsEvent> {
        if index >= self.item_count || self.is_disabled(index) {
            return None;
        }
        self.apply_focus(Some(index));
        self.apply_activation(index, &mut on_activate)
    }

    fn apply_focus(&mut self, next: Option<usize>) -> Option<usize> {
        let normalized = clamp_index(next, self.item_count);
        if !self.focus_mode.is_controlled() {
            self.focused = normalized;
        }
        normalized
    }

    fn apply_activation<F: FnMut(BreadcrumbsActivation)>(
        &mut self,
        index: usize,
        on_activate: &mut F,
    ) -> Option<BreadcrumbsAnalyticsEvent> {
        if !self.selection_mode.is_controlled() {
            self.current = Some(index);
        }
        let analytics = self.analytics_payload(index);
        on_activate(BreadcrumbsActivation {
            index,
            analytics: analytics.clone(),
        });
        analytics
    }

    fn analytics_payload(&self, index: usize) -> Option<BreadcrumbsAnalyticsEvent> {
        let channel = self.analytics_channel.as_ref()?;
        let tag = self
            .analytics_tags
            .get(index)
            .and_then(|value| value.as_ref().map(|value| value.to_string()));
        Some(BreadcrumbsAnalyticsEvent {
            channel: channel.clone(),
            index,
            crumb_tag: tag,
        })
    }

    fn first_enabled(&self) -> Option<usize> {
        (0..self.item_count).find(|&index| !self.is_disabled(index))
    }

    fn last_enabled(&self) -> Option<usize> {
        (0..self.item_count)
            .rev()
            .find(|&index| !self.is_disabled(index))
    }

    fn advance(&self, delta: isize) -> Option<usize> {
        if self.item_count == 0 {
            return None;
        }
        let mut candidate = wrap_index(self.focused, delta, self.item_count);
        let mut attempts = 0;
        while let Some(index) = candidate {
            if !self.is_disabled(index) {
                return Some(index);
            }
            attempts += 1;
            if attempts >= self.item_count {
                break;
            }
            candidate = wrap_index(Some(index), delta, self.item_count);
        }
        None
    }
}

/// Activation payload emitted when a crumb is triggered.
#[derive(Debug, Clone)]
pub struct BreadcrumbsActivation {
    /// Activated crumb index.
    pub index: usize,
    /// Optional analytics payload describing the activation.
    pub analytics: Option<BreadcrumbsAnalyticsEvent>,
}
