//! State machine powering headless tab interfaces.
//!
//! The implementation builds on top of the shared [`selection`] and
//! [`interaction`] utilities to provide orientation aware keyboard handling,
//! manual vs. automatic activation and ergonomic attribute builders for
//! framework adapters.  The goal is to make writing a production grade tab
//! implementation trivial: adapters only need to forward DOM events and wire up
//! identifiers while the state machine takes care of accessibility and
//! scalability concerns.

use crate::aria;
use crate::interaction::ControlKey;
use crate::selection::{clamp_index, wrap_index, ControlStrategy};

/// Determines how keyboard navigation affects the selected tab.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActivationMode {
    /// Manual activation keeps selection stable while roving focus moves.  The
    /// user must press <Space> or <Enter> (or use pointer input) to activate a
    /// tab.  This mirrors the default behaviour described by the WAI-ARIA
    /// Authoring Practices and matches how complex enterprise applications
    /// prefer to defer activation.
    Manual,
    /// Automatic activation commits selection immediately whenever focus moves.
    /// This is the ergonomics focused mode popular in consumer UI libraries.
    Automatic,
}

impl ActivationMode {
    #[inline]
    fn is_automatic(self) -> bool {
        matches!(self, Self::Automatic)
    }
}

/// Orientation of the tab list which controls arrow key semantics.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TabsOrientation {
    /// Horizontal tablists respond to left/right keys.
    Horizontal,
    /// Vertical tablists respond to up/down keys.
    Vertical,
}

impl TabsOrientation {
    /// Returns the ARIA string describing the orientation.
    #[inline]
    pub const fn as_aria(self) -> &'static str {
        match self {
            Self::Horizontal => "horizontal",
            Self::Vertical => "vertical",
        }
    }

    /// Returns whether a key represents forward movement for this orientation.
    #[inline]
    fn is_forward(self, key: ControlKey) -> bool {
        matches!(
            (self, key),
            (Self::Horizontal, ControlKey::ArrowRight) | (Self::Vertical, ControlKey::ArrowDown)
        )
    }

    /// Returns whether a key represents backward movement for this orientation.
    #[inline]
    fn is_backward(self, key: ControlKey) -> bool {
        matches!(
            (self, key),
            (Self::Horizontal, ControlKey::ArrowLeft) | (Self::Vertical, ControlKey::ArrowUp)
        )
    }
}

/// Captures the outcome of processing a keyboard event.  Adapters can inspect
/// the fields to drive DOM side-effects such as scrolling the focused tab into
/// view or updating controlled selection state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct TabKeyboardOutcome {
    /// The tab index that should receive focus after handling the key.
    pub focused: Option<usize>,
    /// The tab index that should be considered selected/active.
    pub selected: Option<usize>,
}

/// Builder for tablist ARIA attributes.  Reusing the builder from adapters keeps
/// stringly typed attribute names centralized and documented.
#[derive(Debug, Clone)]
pub struct TabListAttributes<'a> {
    orientation: TabsOrientation,
    id: Option<&'a str>,
    labelled_by: Option<&'a str>,
}

impl<'a> TabListAttributes<'a> {
    /// Construct a new builder instance.
    #[inline]
    pub fn new(state: &'a TabsState) -> Self {
        Self {
            orientation: state.orientation,
            id: None,
            labelled_by: None,
        }
    }

    /// Assign an ID to the tablist element.
    #[inline]
    pub fn id(mut self, value: &'a str) -> Self {
        self.id = Some(value);
        self
    }

    /// Link the tablist to a labelling element via `aria-labelledby`.
    #[inline]
    pub fn labelled_by(mut self, value: &'a str) -> Self {
        self.labelled_by = Some(value);
        self
    }

    /// Returns the `role="tablist"` tuple.
    #[inline]
    pub fn role(&self) -> &'static str {
        aria::role_tablist()
    }

    /// Returns the `aria-orientation` tuple.
    #[inline]
    pub fn orientation(&self) -> (&'static str, &'static str) {
        aria::aria_orientation(self.orientation.as_aria())
    }

    /// Returns the `id` tuple when configured.
    #[inline]
    pub fn id_attr(&self) -> Option<(&'static str, &str)> {
        self.id.map(|value| ("id", value))
    }

    /// Returns the `aria-labelledby` tuple when configured.
    #[inline]
    pub fn labelledby(&self) -> Option<(&'static str, &str)> {
        self.labelled_by.map(aria::aria_labelledby)
    }
}

/// Headless tab state machine.
#[derive(Debug, Clone)]
pub struct TabsState {
    pub(crate) tab_count: usize,
    pub(crate) selected: Option<usize>,
    pub(crate) focused: Option<usize>,
    pub(crate) activation: ActivationMode,
    pub(crate) orientation: TabsOrientation,
    selection_mode: ControlStrategy,
    focus_mode: ControlStrategy,
}

impl TabsState {
    /// Create a new tab state instance.
    ///
    /// * `tab_count` — number of tabs currently rendered.
    /// * `initial_selected` — index of the initially selected tab.
    /// * `activation` — whether selection follows focus automatically.
    /// * `orientation` — axis along which arrow keys move focus.
    /// * `selection_mode` — describes if the selected tab is externally
    ///   controlled.
    /// * `focus_mode` — describes if the focused tab (roving tabindex) is
    ///   controlled.
    pub fn new(
        tab_count: usize,
        initial_selected: Option<usize>,
        activation: ActivationMode,
        orientation: TabsOrientation,
        selection_mode: ControlStrategy,
        focus_mode: ControlStrategy,
    ) -> Self {
        let selected = clamp_index(initial_selected, tab_count);
        let focused = selected.or_else(|| if tab_count > 0 { Some(0) } else { None });
        Self {
            tab_count,
            selected,
            focused,
            activation,
            orientation,
            selection_mode,
            focus_mode,
        }
    }

    /// Returns the activation mode currently configured.
    #[inline]
    pub fn activation_mode(&self) -> ActivationMode {
        self.activation
    }

    /// Returns the tablist orientation.
    #[inline]
    pub fn orientation(&self) -> TabsOrientation {
        self.orientation
    }

    /// Returns the number of registered tabs.
    #[inline]
    pub fn tab_count(&self) -> usize {
        self.tab_count
    }

    /// Returns the selected tab index.
    #[inline]
    pub fn selected(&self) -> Option<usize> {
        self.selected
    }

    /// Returns the focused tab index (roving tabindex owner).
    #[inline]
    pub fn focused(&self) -> Option<usize> {
        self.focused
    }

    /// Returns whether the provided index is currently selected.
    #[inline]
    pub fn is_selected(&self, index: usize) -> bool {
        self.selected == Some(index)
    }

    /// Returns whether the provided index currently owns focus.
    #[inline]
    pub fn is_focused(&self, index: usize) -> bool {
        self.focused == Some(index)
    }

    /// Update the number of tabs rendered and clamp state to the new bounds.
    pub fn set_tab_count(&mut self, count: usize) {
        self.tab_count = count;
        self.selected = clamp_index(self.selected, count);
        self.focused = if self.focus_mode.is_controlled() {
            clamp_index(self.focused, count)
        } else {
            clamp_index(self.focused, count)
                .or(self.selected)
                .or_else(|| if count > 0 { Some(0) } else { None })
        };
    }

    /// Synchronize the selected tab when controlled externally.
    pub fn sync_selected(&mut self, selected: Option<usize>) {
        self.selected = clamp_index(selected, self.tab_count);
        if !self.focus_mode.is_controlled() {
            if let Some(index) = self.selected {
                self.focused = Some(index);
            } else {
                self.focused = clamp_index(self.focused, self.tab_count).or_else(|| {
                    if self.tab_count > 0 {
                        Some(0)
                    } else {
                        None
                    }
                });
            }
        }
    }

    /// Synchronize the focused tab when the roving tabindex is controlled.
    pub fn sync_focused(&mut self, focused: Option<usize>) {
        if self.focus_mode.is_controlled() {
            self.focused = clamp_index(focused, self.tab_count);
        }
    }

    /// Imperatively focus the provided tab (uncontrolled focus mode).
    pub fn set_focused(&mut self, focused: Option<usize>) {
        if !self.focus_mode.is_controlled() {
            self.focused = clamp_index(focused, self.tab_count);
        }
    }

    /// Activate the provided tab index, invoking the supplied callback.
    pub fn select<F: FnMut(usize)>(&mut self, index: usize, mut on_select: F) {
        if index >= self.tab_count {
            return;
        }
        if !self.focus_mode.is_controlled() {
            self.focused = Some(index);
        }
        if !self.selection_mode.is_controlled() {
            self.selected = Some(index);
        }
        on_select(index);
    }

    /// Activate the currently focused tab when present.
    pub fn select_focused<F: FnMut(usize)>(&mut self, on_select: F) {
        if let Some(index) = self.focused {
            self.select(index, on_select);
        }
    }

    /// Process a keyboard control key returning the resulting focus/selection.
    pub fn on_key<F: FnMut(usize)>(
        &mut self,
        key: ControlKey,
        mut on_select: F,
    ) -> TabKeyboardOutcome {
        let mut outcome = TabKeyboardOutcome::default();
        match key {
            ControlKey::Enter | ControlKey::Space => {
                if let Some(index) = self.focused {
                    if index < self.tab_count {
                        self.select(index, &mut on_select);
                        outcome.focused = Some(index);
                        outcome.selected = Some(index);
                    }
                }
            }
            ControlKey::Home => {
                if self.tab_count > 0 {
                    let next = Some(0);
                    outcome.focused = self.apply_focus(next);
                    if self.activation.is_automatic() {
                        outcome.selected = outcome.focused.map(|index| {
                            self.apply_selection(index, &mut on_select);
                            index
                        });
                    }
                }
            }
            ControlKey::End => {
                if self.tab_count > 0 {
                    let next = Some(self.tab_count - 1);
                    outcome.focused = self.apply_focus(next);
                    if self.activation.is_automatic() {
                        outcome.selected = outcome.focused.map(|index| {
                            self.apply_selection(index, &mut on_select);
                            index
                        });
                    }
                }
            }
            _ if self.orientation.is_forward(key) => {
                if self.tab_count > 0 {
                    self.ensure_focus();
                    let next = wrap_index(self.focused, 1, self.tab_count);
                    outcome.focused = self.apply_focus(next);
                    if self.activation.is_automatic() {
                        outcome.selected = outcome.focused.map(|index| {
                            self.apply_selection(index, &mut on_select);
                            index
                        });
                    }
                }
            }
            _ if self.orientation.is_backward(key) => {
                if self.tab_count > 0 {
                    self.ensure_focus();
                    let next = wrap_index(self.focused, -1, self.tab_count);
                    outcome.focused = self.apply_focus(next);
                    if self.activation.is_automatic() {
                        outcome.selected = outcome.focused.map(|index| {
                            self.apply_selection(index, &mut on_select);
                            index
                        });
                    }
                }
            }
            _ => {}
        }
        outcome
    }

    /// Returns a builder for tablist attributes.
    #[inline]
    pub fn list_attributes(&self) -> TabListAttributes<'_> {
        TabListAttributes::new(self)
    }

    /// Returns a builder for tab attributes.
    #[inline]
    pub fn tab(&self, index: usize) -> crate::tab::TabAttributes<'_> {
        crate::tab::TabAttributes::new(self, index)
    }

    /// Returns a builder for tab panel attributes.
    #[inline]
    pub fn panel(&self, index: usize) -> crate::tab_panel::TabPanelAttributes<'_> {
        crate::tab_panel::TabPanelAttributes::new(self, index)
    }

    fn ensure_focus(&mut self) {
        if self.tab_count == 0 {
            self.focused = None;
            return;
        }
        if self.focus_mode.is_controlled() {
            self.focused = clamp_index(self.focused, self.tab_count);
        } else {
            self.focused = clamp_index(self.focused, self.tab_count)
                .or(self.selected)
                .or(Some(0));
        }
    }

    fn apply_focus(&mut self, next: Option<usize>) -> Option<usize> {
        let next = clamp_index(next, self.tab_count);
        if !self.focus_mode.is_controlled() {
            self.focused = next;
        }
        next
    }

    fn apply_selection<F: FnMut(usize)>(&mut self, index: usize, on_select: &mut F) {
        if !self.selection_mode.is_controlled() {
            self.selected = Some(index);
        }
        if !self.focus_mode.is_controlled() {
            self.focused = Some(index);
        }
        on_select(index);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::selection::ControlStrategy;

    #[test]
    fn automatic_activation_selects_on_arrow_navigation() {
        let mut state = TabsState::new(
            3,
            Some(0),
            ActivationMode::Automatic,
            TabsOrientation::Horizontal,
            ControlStrategy::Uncontrolled,
            ControlStrategy::Uncontrolled,
        );
        let mut selected = Vec::new();
        state.on_key(ControlKey::ArrowRight, |index| selected.push(index));
        assert_eq!(state.selected(), Some(1));
        assert_eq!(state.focused(), Some(1));
        assert_eq!(selected, vec![1]);
    }

    #[test]
    fn manual_activation_defers_selection_until_explicit_request() {
        let mut state = TabsState::new(
            3,
            Some(0),
            ActivationMode::Manual,
            TabsOrientation::Horizontal,
            ControlStrategy::Uncontrolled,
            ControlStrategy::Uncontrolled,
        );
        let mut selected = Vec::new();
        let outcome = state.on_key(ControlKey::ArrowRight, |index| selected.push(index));
        assert_eq!(state.selected(), Some(0));
        assert_eq!(state.focused(), Some(1));
        assert_eq!(outcome.selected, None);
        assert!(selected.is_empty());

        state.on_key(ControlKey::Space, |index| selected.push(index));
        assert_eq!(state.selected(), Some(1));
        assert_eq!(selected, vec![1]);
    }

    #[test]
    fn orientation_switches_arrow_key_mapping() {
        let mut state = TabsState::new(
            4,
            Some(2),
            ActivationMode::Automatic,
            TabsOrientation::Vertical,
            ControlStrategy::Uncontrolled,
            ControlStrategy::Uncontrolled,
        );
        let mut selected = Vec::new();
        state.on_key(ControlKey::ArrowDown, |index| selected.push(index));
        assert_eq!(state.selected(), Some(3));
        state.on_key(ControlKey::ArrowUp, |index| selected.push(index));
        assert_eq!(state.selected(), Some(2));
        assert_eq!(selected, vec![3, 2]);
    }

    #[test]
    fn controlled_focus_returns_intent_without_mutating_internal_state() {
        let mut state = TabsState::new(
            3,
            Some(0),
            ActivationMode::Manual,
            TabsOrientation::Horizontal,
            ControlStrategy::Uncontrolled,
            ControlStrategy::Controlled,
        );
        let mut selected = Vec::new();
        let outcome = state.on_key(ControlKey::ArrowRight, |index| selected.push(index));
        assert_eq!(state.focused(), Some(0));
        assert_eq!(outcome.focused, Some(1));
        assert!(selected.is_empty());
        state.sync_focused(outcome.focused);
        assert_eq!(state.focused(), Some(1));
    }

    #[test]
    fn tablist_attribute_builder_exposes_expected_pairs() {
        let state = TabsState::new(
            2,
            Some(0),
            ActivationMode::Automatic,
            TabsOrientation::Horizontal,
            ControlStrategy::Uncontrolled,
            ControlStrategy::Uncontrolled,
        );
        let attrs = state.list_attributes().id("tabs").labelled_by("tabs-label");
        assert_eq!(attrs.role(), "tablist");
        assert_eq!(attrs.orientation(), ("aria-orientation", "horizontal"));
        assert_eq!(attrs.id_attr(), Some(("id", "tabs")));
        assert_eq!(attrs.labelledby(), Some(("aria-labelledby", "tabs-label")));
    }
}
