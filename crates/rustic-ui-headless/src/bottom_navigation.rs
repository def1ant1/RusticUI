#![deny(missing_docs)]
//! Headless state machine powering Material flavored bottom navigation bars.
//!
//! The implementation keeps ARIA wiring, keyboard orchestration, and analytics
//! hooks centralized so framework adapters can remain declarative.  Integrations
//! only need to forward DOM events and configure identifiers; the state machine
//! handles controlled/uncontrolled flows, roving focus, disabled item semantics,
//! and event payload generation for telemetry pipelines.  The goal is to let
//! enterprise teams stamp out consistent navigation shells without bespoke
//! boilerplate in every codebase.

use crate::{
    aria,
    interaction::ControlKey,
    selection::{clamp_index, wrap_index, ControlStrategy},
};

/// Describes how keyboard interaction updates the selected destination.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BottomNavigationActivationMode {
    /// Selection changes immediately as focus moves.  This mirrors the default
    /// Material behaviour where navigating with the keyboard activates the
    /// highlighted destination.
    Automatic,
    /// Selection is committed explicitly via <Enter> or <Space>.  Enterprise
    /// surfaces frequently adopt this mode so content panes do not re-render
    /// until the user confirms their intent.
    Manual,
}

impl BottomNavigationActivationMode {
    #[inline]
    fn is_automatic(self) -> bool {
        matches!(self, Self::Automatic)
    }
}

/// Outcome produced after processing a keyboard event.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct BottomNavigationKeyboardOutcome {
    /// Destination that should receive focus after the key press.
    pub focused: Option<usize>,
    /// Destination that should be considered selected.
    pub selected: Option<usize>,
    /// Analytics payload describing the selection transition.
    pub analytics: Option<BottomNavigationAnalyticsEvent>,
}

/// Analytics payload emitted when a destination is activated.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BottomNavigationAnalyticsEvent {
    /// Logical channel that downstream telemetry should attribute the event to.
    pub channel: String,
    /// Zero-based index of the activated destination.
    pub index: usize,
    /// Optional identifier supplied by the adapter for the activated item.
    pub item_tag: Option<String>,
}

/// Builder describing the root navigation element.
#[derive(Debug, Clone)]
pub struct BottomNavigationAttributes<'a> {
    state: &'a BottomNavigationState,
    id: Option<&'a str>,
    labelled_by: Option<&'a str>,
}

impl<'a> BottomNavigationAttributes<'a> {
    /// Create a new builder instance bound to the provided state.
    pub fn new(state: &'a BottomNavigationState) -> Self {
        Self {
            state,
            id: None,
            labelled_by: None,
        }
    }

    /// Assign a DOM id to the navigation region.
    pub fn id(mut self, value: &'a str) -> Self {
        self.id = Some(value);
        self
    }

    /// Link the navigation region to an external label.
    pub fn labelled_by(mut self, value: &'a str) -> Self {
        self.labelled_by = Some(value);
        self
    }

    /// Returns the ARIA role describing the container.
    pub fn role(&self) -> &'static str {
        aria::role_tablist()
    }

    /// Optional id attribute tuple.
    pub fn id_attr(&self) -> Option<(&'static str, &str)> {
        self.id.map(|value| ("id", value))
    }

    /// Optional `aria-labelledby` attribute tuple.
    pub fn labelledby(&self) -> Option<(&'static str, &str)> {
        self.labelled_by.map(aria::aria_labelledby)
    }

    /// Optional analytics attribute applied to the root wrapper.
    pub fn analytics_attribute(&self) -> Option<(&'static str, &str)> {
        self.state
            .analytics_channel
            .as_deref()
            .map(|value| ("data-rustic-analytics-channel", value))
    }
}

/// Builder describing a single navigation destination.
#[derive(Debug, Clone)]
pub struct BottomNavigationItemAttributes<'a> {
    state: &'a BottomNavigationState,
    index: usize,
    id: Option<&'a str>,
    controls: Option<&'a str>,
    analytics_tag: Option<&'a str>,
}

impl<'a> BottomNavigationItemAttributes<'a> {
    /// Internal helper for constructing a builder instance.
    fn new(state: &'a BottomNavigationState, index: usize) -> Self {
        Self {
            state,
            index,
            id: None,
            controls: None,
            analytics_tag: None,
        }
    }

    /// Assign the DOM id for the interactive element.
    pub fn id(mut self, value: &'a str) -> Self {
        self.id = Some(value);
        self
    }

    /// Declare the `aria-controls` relationship linking to the associated panel.
    pub fn controls(mut self, value: &'a str) -> Self {
        self.controls = Some(value);
        self
    }

    /// Associate a stable analytics tag for the destination.
    pub fn analytics_tag(mut self, value: &'a str) -> Self {
        self.analytics_tag = Some(value);
        self
    }

    /// Collect the ARIA and data attributes describing the destination.
    pub fn as_pairs(&self) -> Vec<(&'static str, String)> {
        let mut pairs = Vec::with_capacity(6);
        pairs.push(("role", aria::role_tab().to_string()));
        let selected = self.state.selected == Some(self.index);
        let focused = self.state.focused == Some(self.index);
        pairs.push((
            aria::aria_selected(selected).0,
            aria::aria_selected(selected).1.to_string(),
        ));
        let tabindex = if focused { 0 } else { -1 };
        pairs.push(("tabindex", tabindex.to_string()));
        if let Some(id) = self.id {
            pairs.push(("id", id.to_string()));
        }
        if let Some(controls) = self.controls {
            pairs.push(("aria-controls", controls.to_string()));
        }
        if let Some(tag) = self.analytics_tag {
            pairs.push(("data-rustic-analytics-id", tag.to_string()));
        }
        pairs.push((
            "data-rustic-bottom-navigation-index",
            self.index.to_string(),
        ));
        if let Some(disabled) = self.state.disabled.get(self.index) {
            if *disabled {
                pairs.push(("aria-disabled", "true".into()));
                pairs.push(("data-disabled", "true".into()));
            }
        }
        pairs
    }
}

/// Headless controller managing bottom navigation state.
#[derive(Debug, Clone)]
pub struct BottomNavigationState {
    item_count: usize,
    selected: Option<usize>,
    focused: Option<usize>,
    activation: BottomNavigationActivationMode,
    selection_mode: ControlStrategy,
    focus_mode: ControlStrategy,
    analytics_channel: Option<String>,
    analytics_tags: Vec<Option<String>>,
    disabled: Vec<bool>,
}

impl BottomNavigationState {
    /// Construct a new state machine instance.
    pub fn new(
        item_count: usize,
        default_selected: Option<usize>,
        activation: BottomNavigationActivationMode,
        selection_mode: ControlStrategy,
        focus_mode: ControlStrategy,
    ) -> Self {
        let selected = clamp_index(default_selected, item_count);
        let focused = selected.or(if item_count > 0 { Some(0) } else { None });
        Self {
            item_count,
            selected,
            focused,
            activation,
            selection_mode,
            focus_mode,
            analytics_channel: None,
            analytics_tags: vec![None; item_count],
            disabled: vec![false; item_count],
        }
    }

    /// Returns the builder for the container attributes.
    pub fn root_attributes(&self) -> BottomNavigationAttributes<'_> {
        BottomNavigationAttributes::new(self)
    }

    /// Returns the builder for an item at the specified index.
    pub fn item_attributes(&self, index: usize) -> BottomNavigationItemAttributes<'_> {
        BottomNavigationItemAttributes::new(self, index)
    }

    /// Configure an analytics channel surfaced via `data-rustic-analytics-channel`.
    pub fn set_analytics_channel(&mut self, channel: Option<impl Into<String>>) {
        self.analytics_channel = channel.map(Into::into);
    }

    /// Assign an analytics tag used when emitting selection events for an item.
    pub fn set_item_analytics_tag(&mut self, index: usize, tag: Option<impl Into<String>>) {
        if index >= self.item_count {
            return;
        }
        if let Some(slot) = self.analytics_tags.get_mut(index) {
            *slot = tag.map(Into::into);
        }
    }

    /// Update the disabled flag for a destination.
    pub fn set_item_disabled(&mut self, index: usize, disabled: bool) {
        if index >= self.item_count {
            return;
        }
        if let Some(slot) = self.disabled.get_mut(index) {
            *slot = disabled;
        }
        if self.focused == Some(index) && disabled {
            self.focused = self.next_enabled(index as isize + 1);
        }
        if self.selected == Some(index) && disabled {
            self.selected = self.next_enabled(index as isize + 1);
        }
    }

    /// Returns whether the destination is disabled.
    pub fn is_disabled(&self, index: usize) -> bool {
        self.disabled.get(index).copied().unwrap_or(true)
    }

    /// Synchronize the selected destination for controlled adapters.
    pub fn sync_selected(&mut self, index: Option<usize>) {
        self.selected = clamp_index(index, self.item_count);
    }

    /// Synchronize the focused destination when the adapter owns roving tabindex.
    pub fn sync_focused(&mut self, index: Option<usize>) {
        self.focused = clamp_index(index, self.item_count);
    }

    /// Returns the currently selected destination.
    pub fn selected(&self) -> Option<usize> {
        self.selected
    }

    /// Returns the currently focused destination.
    pub fn focused(&self) -> Option<usize> {
        self.focused
    }

    /// Process a keyboard interaction and return the resulting focus/selection.
    pub fn on_key<F: FnMut(BottomNavigationSelection)>(
        &mut self,
        key: ControlKey,
        mut on_select: F,
    ) -> BottomNavigationKeyboardOutcome {
        let mut outcome = BottomNavigationKeyboardOutcome::default();
        match key {
            ControlKey::Enter | ControlKey::Space => {
                if let Some(index) = self.focused {
                    if !self.is_disabled(index) {
                        outcome.selected = Some(index);
                        outcome.focused = Some(index);
                        outcome.analytics = self.apply_selection(index, &mut on_select);
                    }
                }
            }
            ControlKey::Home => {
                let next = self.first_enabled();
                outcome.focused = self.apply_focus(next);
                if self.activation.is_automatic() {
                    if let Some(index) = outcome.focused {
                        outcome.selected = Some(index);
                        outcome.analytics = self.apply_selection(index, &mut on_select);
                    }
                }
            }
            ControlKey::End => {
                let next = self.last_enabled();
                outcome.focused = self.apply_focus(next);
                if self.activation.is_automatic() {
                    if let Some(index) = outcome.focused {
                        outcome.selected = Some(index);
                        outcome.analytics = self.apply_selection(index, &mut on_select);
                    }
                }
            }
            _ if key.is_forward() => {
                let next = self.advance(1);
                outcome.focused = self.apply_focus(next);
                if self.activation.is_automatic() {
                    if let Some(index) = outcome.focused {
                        outcome.selected = Some(index);
                        outcome.analytics = self.apply_selection(index, &mut on_select);
                    }
                }
            }
            _ if key.is_backward() => {
                let next = self.advance(-1);
                outcome.focused = self.apply_focus(next);
                if self.activation.is_automatic() {
                    if let Some(index) = outcome.focused {
                        outcome.selected = Some(index);
                        outcome.analytics = self.apply_selection(index, &mut on_select);
                    }
                }
            }
            _ => {}
        }
        outcome
    }

    /// Activate the currently focused destination.
    pub fn select_focused<F: FnMut(BottomNavigationSelection)>(
        &mut self,
        mut on_select: F,
    ) -> Option<BottomNavigationAnalyticsEvent> {
        if let Some(index) = self.focused {
            if !self.is_disabled(index) {
                return self.apply_selection(index, &mut on_select);
            }
        }
        None
    }

    /// Activate a specific destination by index.
    pub fn select_index<F: FnMut(BottomNavigationSelection)>(
        &mut self,
        index: usize,
        mut on_select: F,
    ) -> Option<BottomNavigationAnalyticsEvent> {
        if index >= self.item_count || self.is_disabled(index) {
            return None;
        }
        self.apply_focus(Some(index));
        self.apply_selection(index, &mut on_select)
    }

    /// Resize the navigation when the number of destinations changes.
    pub fn set_item_count(&mut self, count: usize) {
        self.item_count = count;
        self.analytics_tags.resize(count, None);
        self.disabled.resize(count, false);
        self.focused = clamp_index(self.focused, count);
        self.selected = clamp_index(self.selected, count);
    }

    fn apply_focus(&mut self, next: Option<usize>) -> Option<usize> {
        let normalized = clamp_index(next, self.item_count);
        if !self.focus_mode.is_controlled() {
            self.focused = normalized;
        }
        normalized
    }

    fn apply_selection<F: FnMut(BottomNavigationSelection)>(
        &mut self,
        index: usize,
        on_select: &mut F,
    ) -> Option<BottomNavigationAnalyticsEvent> {
        if !self.focus_mode.is_controlled() {
            self.focused = Some(index);
        }
        if !self.selection_mode.is_controlled() {
            self.selected = Some(index);
        }
        let analytics = self.analytics_payload(index);
        on_select(BottomNavigationSelection {
            index,
            analytics: analytics.clone(),
        });
        analytics
    }

    fn analytics_payload(&self, index: usize) -> Option<BottomNavigationAnalyticsEvent> {
        let channel = self.analytics_channel.as_ref()?;
        let tag = self
            .analytics_tags
            .get(index)
            .and_then(|value| value.as_ref().map(|value| value.to_string()));
        Some(BottomNavigationAnalyticsEvent {
            channel: channel.clone(),
            index,
            item_tag: tag,
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

    fn next_enabled(&self, start: isize) -> Option<usize> {
        if self.item_count == 0 {
            return None;
        }
        let mut current = ((start % self.item_count as isize) + self.item_count as isize)
            % self.item_count as isize;
        for _ in 0..self.item_count {
            let index = current as usize;
            if !self.is_disabled(index) {
                return Some(index);
            }
            current = (current + 1) % self.item_count as isize;
        }
        None
    }

    fn advance(&self, delta: isize) -> Option<usize> {
        if self.item_count == 0 {
            return None;
        }
        if self.focused.is_none() {
            return if delta.is_positive() {
                self.first_enabled()
            } else {
                self.last_enabled()
            };
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

/// Selection notification emitted by [`BottomNavigationState`] when a destination
/// changes.
#[derive(Debug, Clone)]
pub struct BottomNavigationSelection {
    /// Selected index.
    pub index: usize,
    /// Optional analytics payload.
    pub analytics: Option<BottomNavigationAnalyticsEvent>,
}
