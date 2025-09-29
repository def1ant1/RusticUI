#![deny(missing_docs)]
//! Headless state machine powering Material speed dial components.
//!
//! The state exposes deterministic keyboard handling, ARIA wiring, and
//! analytics hooks so framework adapters only need to forward events and merge
//! attribute pairs.  Controlling teams can decide whether the open/highlight
//! state is managed internally or externally by toggling the `ControlStrategy`
//! flags.

use crate::{
    aria,
    interaction::ControlKey,
    selection::{clamp_index, wrap_index, ControlStrategy},
};

/// Analytics signal emitted when the speed dial mutates.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpeedDialAnalyticsEvent {
    /// Logical telemetry channel surfaced on the root element.
    pub channel: String,
    /// Describes the action that triggered the event.
    pub kind: SpeedDialAnalyticsKind,
}

/// Enumerates the analytics events emitted by the speed dial.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpeedDialAnalyticsKind {
    /// The floating action button opened the speed dial.
    Opened,
    /// The speed dial closed, typically because focus left the menu or the user
    /// invoked the trigger again.
    Closed,
    /// An action button inside the speed dial was activated.
    Action {
        /// Zero-based action index that triggered the event.
        index: usize,
        /// Optional analytics tag associated with the activated action.
        tag: Option<String>,
    },
}

/// Outcome returned after processing a keyboard event.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct SpeedDialKeyboardOutcome {
    /// Which action should receive focus.
    pub highlighted: Option<usize>,
    /// Which action should be invoked.
    pub activated: Option<usize>,
    /// Analytics payload describing the change.
    pub analytics: Option<SpeedDialAnalyticsEvent>,
}

/// Builder describing the floating action button attributes.
#[derive(Debug, Clone)]
pub struct SpeedDialTriggerAttributes<'a> {
    state: &'a SpeedDialState,
    id: Option<&'a str>,
    analytics_tag: Option<&'a str>,
}

impl<'a> SpeedDialTriggerAttributes<'a> {
    fn new(state: &'a SpeedDialState) -> Self {
        Self {
            state,
            id: None,
            analytics_tag: None,
        }
    }

    /// Assign a DOM id to the trigger button.
    pub fn id(mut self, value: &'a str) -> Self {
        self.id = Some(value);
        self
    }

    /// Associate an analytics identifier with the trigger.
    pub fn analytics_tag(mut self, value: &'a str) -> Self {
        self.analytics_tag = Some(value);
        self
    }

    /// Collect ARIA/data attributes describing the trigger.
    pub fn as_pairs(&self) -> Vec<(&'static str, String)> {
        let mut pairs = Vec::with_capacity(8);
        pairs.push(("role", aria::role_button().to_string()));
        pairs.push((
            "aria-expanded",
            if self.state.open { "true" } else { "false" }.to_string(),
        ));
        let (popup_key, popup_value) = aria::aria_haspopup("menu");
        pairs.push((popup_key, popup_value.to_string()));
        if let Some(id) = self.id {
            pairs.push(("id", id.to_string()));
        }
        if let Some(channel) = self.state.analytics_channel.as_ref() {
            pairs.push(("data-rustic-analytics-channel", channel.clone()));
        }
        if let Some(tag) = self.analytics_tag {
            pairs.push(("data-rustic-analytics-id", tag.to_string()));
        }
        pairs.push((
            "data-rustic-speed-dial-state",
            if self.state.open { "open" } else { "closed" }.to_string(),
        ));
        pairs
    }
}

/// Builder describing the action list container.
#[derive(Debug, Clone)]
pub struct SpeedDialListAttributes<'a> {
    state: &'a SpeedDialState,
    id: Option<&'a str>,
}

impl<'a> SpeedDialListAttributes<'a> {
    fn new(state: &'a SpeedDialState) -> Self {
        Self { state, id: None }
    }

    /// Assign a DOM id to the list element.
    pub fn id(mut self, value: &'a str) -> Self {
        self.id = Some(value);
        self
    }

    /// Collect attributes describing the action list.
    pub fn as_pairs(&self) -> Vec<(&'static str, String)> {
        let mut pairs = Vec::with_capacity(6);
        pairs.push(("role", aria::role_menu().to_string()));
        if let Some(id) = self.id {
            pairs.push(("id", id.to_string()));
        }
        pairs.push(("data-rustic-speed-dial", "actions".to_string()));
        if let Some(channel) = self.state.analytics_channel.as_ref() {
            pairs.push(("data-rustic-analytics-channel", channel.clone()));
        }
        pairs
    }
}

/// Builder describing an individual action button.
#[derive(Debug, Clone)]
pub struct SpeedDialActionAttributes<'a> {
    state: &'a SpeedDialState,
    index: usize,
    id: Option<&'a str>,
    analytics_tag: Option<&'a str>,
    aria_label: Option<&'a str>,
}

impl<'a> SpeedDialActionAttributes<'a> {
    fn new(state: &'a SpeedDialState, index: usize) -> Self {
        Self {
            state,
            index,
            id: None,
            analytics_tag: None,
            aria_label: None,
        }
    }

    /// Assign a DOM id to the action button.
    pub fn id(mut self, value: &'a str) -> Self {
        self.id = Some(value);
        self
    }

    /// Associate an analytics identifier with the action.
    pub fn analytics_tag(mut self, value: &'a str) -> Self {
        self.analytics_tag = Some(value);
        self
    }

    /// Override the accessible label for the action.
    pub fn aria_label(mut self, value: &'a str) -> Self {
        self.aria_label = Some(value);
        self
    }

    /// Collect the configured attributes into reusable pairs.
    pub fn as_pairs(&self) -> Vec<(&'static str, String)> {
        let mut pairs = Vec::with_capacity(10);
        pairs.push(("role", aria::role_menuitem().to_string()));
        let highlighted = self.state.highlighted == Some(self.index);
        pairs.push(("tabindex", if highlighted { "0" } else { "-1" }.to_string()));
        if let Some(id) = self.id {
            pairs.push(("id", id.to_string()));
        }
        let disabled = self.state.is_action_disabled(self.index);
        if disabled {
            pairs.push(("aria-disabled", "true".into()));
            pairs.push(("data-disabled", "true".into()));
        }
        if let Some(tag) = self.analytics_tag.or_else(|| {
            self.state
                .action_tags
                .get(self.index)
                .and_then(|value| value.as_deref())
        }) {
            pairs.push(("data-rustic-analytics-id", tag.to_string()));
        }
        if let Some(channel) = self.state.analytics_channel.as_ref() {
            pairs.push(("data-rustic-analytics-channel", channel.clone()));
        }
        let label = self
            .aria_label
            .map(|value| value.to_string())
            .unwrap_or_else(|| format!("Speed dial action {}", self.index + 1));
        pairs.push(("aria-label", label));
        pairs.push(("data-rustic-speed-dial-index", self.index.to_string()));
        pairs
    }
}

/// Headless controller for Material speed dial menus.
#[derive(Debug, Clone)]
pub struct SpeedDialState {
    action_count: usize,
    open: bool,
    highlighted: Option<usize>,
    open_mode: ControlStrategy,
    highlight_mode: ControlStrategy,
    analytics_channel: Option<String>,
    action_tags: Vec<Option<String>>,
    disabled: Vec<bool>,
}

impl SpeedDialState {
    /// Construct a new speed dial controller.
    pub fn new(
        action_count: usize,
        default_open: bool,
        open_mode: ControlStrategy,
        highlight_mode: ControlStrategy,
    ) -> Self {
        let open = if open_mode.is_controlled() {
            false
        } else {
            default_open
        };
        let highlighted = if action_count > 0 { Some(0) } else { None };
        Self {
            action_count,
            open,
            highlighted,
            open_mode,
            highlight_mode,
            analytics_channel: None,
            action_tags: vec![None; action_count],
            disabled: vec![false; action_count],
        }
    }

    /// Returns whether the speed dial menu is expanded.
    pub fn is_open(&self) -> bool {
        self.open
    }

    /// Returns the currently highlighted action index.
    pub fn highlighted(&self) -> Option<usize> {
        self.highlighted
    }

    /// Builder for the floating action button.
    pub fn trigger_attributes(&self) -> SpeedDialTriggerAttributes<'_> {
        SpeedDialTriggerAttributes::new(self)
    }

    /// Builder for the action list container.
    pub fn list_attributes(&self) -> SpeedDialListAttributes<'_> {
        SpeedDialListAttributes::new(self)
    }

    /// Builder for an individual action button.
    pub fn action_attributes(&self, index: usize) -> SpeedDialActionAttributes<'_> {
        SpeedDialActionAttributes::new(self, index)
    }

    /// Configure the analytics channel surfaced on all rendered elements.
    pub fn set_analytics_channel(&mut self, channel: Option<impl Into<String>>) {
        self.analytics_channel = channel.map(Into::into);
    }

    /// Configure the analytics tag for a specific action index.
    pub fn set_action_analytics_tag(&mut self, index: usize, tag: Option<impl Into<String>>) {
        if index >= self.action_count {
            return;
        }
        if let Some(slot) = self.action_tags.get_mut(index) {
            *slot = tag.map(Into::into);
        }
    }

    /// Mark an action as disabled.
    pub fn set_action_disabled(&mut self, index: usize, disabled: bool) {
        if index >= self.action_count {
            return;
        }
        if let Some(slot) = self.disabled.get_mut(index) {
            *slot = disabled;
        }
        if disabled && self.highlighted == Some(index) {
            self.highlighted = self.advance(1);
        }
    }

    /// Returns whether the action at `index` is disabled.
    pub fn is_action_disabled(&self, index: usize) -> bool {
        self.disabled.get(index).copied().unwrap_or(true)
    }

    /// Update the number of rendered actions.
    pub fn set_action_count(&mut self, count: usize) {
        self.action_count = count;
        self.action_tags.resize(count, None);
        self.disabled.resize(count, false);
        self.highlighted = clamp_index(self.highlighted, count);
    }

    /// Synchronise the open state when controlled externally.
    pub fn sync_open(&mut self, open: bool) {
        if self.open_mode.is_controlled() {
            self.open = open;
        }
    }

    /// Synchronise the highlighted index when controlled externally.
    pub fn sync_highlighted(&mut self, index: Option<usize>) {
        if self.highlight_mode.is_controlled() {
            self.highlighted = clamp_index(index, self.action_count);
        }
    }

    /// Open the speed dial.
    pub fn open<F: FnOnce(bool)>(&mut self, notify: F) -> Option<SpeedDialAnalyticsEvent> {
        self.set_open(true, notify)
    }

    /// Close the speed dial.
    pub fn close<F: FnOnce(bool)>(&mut self, notify: F) -> Option<SpeedDialAnalyticsEvent> {
        self.set_open(false, notify)
    }

    /// Toggle the speed dial.
    pub fn toggle<F: FnOnce(bool)>(&mut self, notify: F) -> Option<SpeedDialAnalyticsEvent> {
        self.set_open(!self.open, notify)
    }

    /// Handle keyboard interaction returning the resulting intent.
    pub fn on_key<F: FnMut(SpeedDialSelection)>(
        &mut self,
        key: ControlKey,
        mut on_select: F,
    ) -> SpeedDialKeyboardOutcome {
        let mut outcome = SpeedDialKeyboardOutcome::default();
        match key {
            ControlKey::Enter | ControlKey::Space => {
                if !self.open {
                    outcome.analytics = self.open(|_| {});
                    outcome.highlighted = self.highlighted;
                } else if let Some(index) = self.highlighted {
                    if !self.is_action_disabled(index) {
                        let result = self.activate(index, &mut on_select);
                        outcome.activated = result.activated;
                        outcome.analytics = result.analytics;
                    }
                }
            }
            ControlKey::Home => {
                outcome.highlighted = self.apply_highlight(self.first_enabled());
            }
            ControlKey::End => {
                outcome.highlighted = self.apply_highlight(self.last_enabled());
            }
            _ if key.is_forward() => {
                outcome.highlighted = self.apply_highlight(self.advance(1));
            }
            _ if key.is_backward() => {
                outcome.highlighted = self.apply_highlight(self.advance(-1));
            }
            _ => {}
        }
        outcome
    }

    /// Activate a specific action.
    pub fn activate<F: FnMut(SpeedDialSelection)>(
        &mut self,
        index: usize,
        mut on_select: F,
    ) -> SpeedDialSelectionOutcome {
        if index >= self.action_count || self.is_action_disabled(index) {
            return SpeedDialSelectionOutcome::default();
        }
        if !self.highlight_mode.is_controlled() {
            self.highlighted = Some(index);
        }
        let analytics = self.analytics_payload(SpeedDialAnalyticsKind::Action {
            index,
            tag: self
                .action_tags
                .get(index)
                .and_then(|value| value.as_ref().map(|value| value.to_string())),
        });
        let selection = SpeedDialSelection {
            index,
            analytics: analytics.clone(),
        };
        on_select(selection);
        SpeedDialSelectionOutcome {
            activated: Some(index),
            analytics,
        }
    }

    fn set_open<F: FnOnce(bool)>(
        &mut self,
        next: bool,
        notify: F,
    ) -> Option<SpeedDialAnalyticsEvent> {
        if self.open == next {
            notify(next);
            return None;
        }
        if !self.open_mode.is_controlled() {
            self.open = next;
        }
        notify(next);
        if next {
            self.highlighted = self.first_enabled();
            self.analytics_payload(SpeedDialAnalyticsKind::Opened)
        } else {
            self.analytics_payload(SpeedDialAnalyticsKind::Closed)
        }
    }

    fn analytics_payload(&self, kind: SpeedDialAnalyticsKind) -> Option<SpeedDialAnalyticsEvent> {
        self.analytics_channel
            .as_ref()
            .map(|channel| SpeedDialAnalyticsEvent {
                channel: channel.clone(),
                kind,
            })
    }

    fn apply_highlight(&mut self, next: Option<usize>) -> Option<usize> {
        let normalized = clamp_index(next, self.action_count);
        if !self.highlight_mode.is_controlled() {
            self.highlighted = normalized;
        }
        normalized
    }

    fn first_enabled(&self) -> Option<usize> {
        (0..self.action_count).find(|&index| !self.is_action_disabled(index))
    }

    fn last_enabled(&self) -> Option<usize> {
        (0..self.action_count)
            .rev()
            .find(|&index| !self.is_action_disabled(index))
    }

    fn advance(&self, delta: isize) -> Option<usize> {
        if self.action_count == 0 {
            return None;
        }
        if self.highlighted.is_none() {
            return if delta.is_positive() {
                self.first_enabled()
            } else {
                self.last_enabled()
            };
        }
        let mut candidate = wrap_index(self.highlighted, delta, self.action_count);
        let mut attempts = 0;
        while let Some(index) = candidate {
            if !self.is_action_disabled(index) {
                return Some(index);
            }
            attempts += 1;
            if attempts >= self.action_count {
                break;
            }
            candidate = wrap_index(Some(index), delta, self.action_count);
        }
        None
    }
}

/// Selection payload emitted when an action is activated.
#[derive(Debug, Clone)]
pub struct SpeedDialSelection {
    /// Activated action index.
    pub index: usize,
    /// Optional analytics payload describing the activation.
    pub analytics: Option<SpeedDialAnalyticsEvent>,
}

/// Result returned by [`SpeedDialState::activate`] and [`SpeedDialState::on_key`].
#[derive(Debug, Clone, Default)]
pub struct SpeedDialSelectionOutcome {
    /// Activated action index when a selection occurs.
    pub activated: Option<usize>,
    /// Analytics payload emitted alongside the activation.
    pub analytics: Option<SpeedDialAnalyticsEvent>,
}
