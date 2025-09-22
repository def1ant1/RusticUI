//! Autocomplete state machine that combines Joy’s text field ergonomics with the
//! shared listbox/select patterns from Material.
//!
//! The struct embeds [`SelectState`](crate::select::SelectState) so typeahead,
//! highlight management, and controlled/uncontrolled behavior stay perfectly
//! aligned with the select widget.  Adapters only need to pipe user intents into
//! the exposed methods and apply the returned [`AutocompleteChange`] to their
//! DOM.  Rich documentation is sprinkled throughout so large applications can
//! audit and extend the behavior without reverse engineering hidden invariants.

use crate::aria;
use crate::select::SelectState;
use crate::selection::ControlStrategy;

/// Re-export [`ControlStrategy`] so consumers configuring the autocomplete do
/// not need to reach into the private `selection` module.
pub use crate::selection::ControlStrategy as AutocompleteControlStrategy;

/// Declarative configuration consumed by [`AutocompleteState`].
#[derive(Debug, Clone)]
pub struct AutocompleteConfig {
    /// Number of options currently rendered inside the listbox.
    pub option_count: usize,
    /// Optional index of the option that should start selected/highlighted.
    pub initial_selected: Option<usize>,
    /// Whether the popover starts open when uncontrolled.
    pub default_open: bool,
    /// Describes if the open state is controlled by a parent.
    pub open_control: ControlStrategy,
    /// Describes if the selected option is controlled by a parent.
    pub selection_control: ControlStrategy,
    /// When `true` the listbox opens as soon as the input receives focus.
    pub open_on_focus: bool,
    /// When `true` the input value is decoupled from the selected option.
    pub free_solo: bool,
    /// When `true` the entire widget is disabled.
    pub disabled: bool,
    /// Initial text rendered inside the input element.
    pub initial_input: String,
}

impl AutocompleteConfig {
    /// Enterprise friendly defaults mirroring Joy’s TypeScript implementation.
    pub fn enterprise_defaults(option_count: usize) -> Self {
        Self {
            option_count,
            initial_selected: None,
            default_open: false,
            open_control: ControlStrategy::Uncontrolled,
            selection_control: ControlStrategy::Uncontrolled,
            open_on_focus: true,
            free_solo: false,
            disabled: false,
            initial_input: String::new(),
        }
    }
}

impl Default for AutocompleteConfig {
    fn default() -> Self {
        Self::enterprise_defaults(0)
    }
}

/// Aggregate change notification emitted from [`AutocompleteState`] transitions.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct AutocompleteChange {
    /// Indicates whether the popover requested an open/close transition.
    pub opened: Option<bool>,
    /// Indicates that the input value changed as a result of an interaction.
    pub input_value: Option<String>,
    /// Indicates that a specific option index was selected.
    pub selection: Option<usize>,
}

/// Headless state machine coordinating the Joy autocomplete widget.
#[derive(Debug, Clone)]
pub struct AutocompleteState {
    select: SelectState,
    config: AutocompleteConfig,
    input_value: String,
    focused: bool,
}

impl AutocompleteState {
    /// Construct a new autocomplete state machine.
    pub fn new(config: AutocompleteConfig) -> Self {
        let select = SelectState::new(
            config.option_count,
            config.initial_selected,
            config.default_open,
            config.open_control,
            config.selection_control,
        );
        Self {
            input_value: config.initial_input.clone(),
            select,
            config,
            focused: false,
        }
    }

    /// Returns a shared reference to the internal [`SelectState`].
    #[inline]
    pub fn select_state(&self) -> &SelectState {
        &self.select
    }

    /// Returns a mutable reference to the internal [`SelectState`].
    #[inline]
    pub fn select_state_mut(&mut self) -> &mut SelectState {
        &mut self.select
    }

    /// Returns whether the listbox is currently open.
    #[inline]
    pub fn is_open(&self) -> bool {
        self.select.is_open()
    }

    /// Returns the current input value.
    #[inline]
    pub fn input_value(&self) -> &str {
        &self.input_value
    }

    /// Returns whether the widget is currently focused.
    #[inline]
    pub fn is_focused(&self) -> bool {
        self.focused
    }

    /// Returns whether the widget is disabled.
    #[inline]
    pub fn is_disabled(&self) -> bool {
        self.config.disabled
    }

    /// Programmatically focus the autocomplete.
    pub fn focus(&mut self) -> AutocompleteChange {
        self.focused = true;
        if self.config.disabled || !self.config.open_on_focus {
            return AutocompleteChange::default();
        }
        self.open_internal()
    }

    /// Programmatically blur the autocomplete.
    pub fn blur(&mut self) -> AutocompleteChange {
        self.focused = false;
        if self.config.disabled {
            return AutocompleteChange::default();
        }
        self.close_internal()
    }

    /// Request that the popover opens.
    pub fn open(&mut self) -> AutocompleteChange {
        if self.config.disabled {
            return AutocompleteChange::default();
        }
        self.open_internal()
    }

    /// Request that the popover closes.
    pub fn close(&mut self) -> AutocompleteChange {
        if self.config.disabled {
            return AutocompleteChange::default();
        }
        self.close_internal()
    }

    /// Toggle the popover visibility.
    pub fn toggle(&mut self) -> AutocompleteChange {
        if self.config.disabled {
            return AutocompleteChange::default();
        }
        if self.is_open() {
            self.close_internal()
        } else {
            self.open_internal()
        }
    }

    /// Synchronise the open flag when controlled externally.
    pub fn sync_open(&mut self, open: bool) {
        self.select.sync_open(open);
    }

    /// Synchronise the selected option when controlled externally.
    pub fn sync_selected(&mut self, selected: Option<usize>) {
        self.select.sync_selected(selected);
    }

    /// Update the number of rendered options.
    pub fn set_option_count(&mut self, count: usize) {
        self.select.set_option_count(count);
    }

    /// Toggle the disabled flag for a specific option.
    pub fn set_option_disabled(&mut self, index: usize, disabled: bool) {
        self.select.set_option_disabled(index, disabled);
    }

    /// Mutate the input value directly.
    pub fn set_input_value<S: Into<String>>(&mut self, value: S) -> AutocompleteChange {
        if self.config.disabled {
            return AutocompleteChange::default();
        }
        let value = value.into();
        self.input_value = value.clone();
        AutocompleteChange {
            input_value: Some(value),
            ..AutocompleteChange::default()
        }
    }

    /// Select the provided option index.
    ///
    /// The closure receives the resolved index and can optionally return the
    /// textual label that should be pushed back into the input.  Returning
    /// `None` keeps the current input untouched which is useful when the widget
    /// is operating in `free_solo` mode.
    pub fn select_index<F>(&mut self, index: usize, mut on_select: F) -> AutocompleteChange
    where
        F: FnMut(usize) -> Option<String>,
    {
        if self.config.disabled {
            return AutocompleteChange::default();
        }
        let mut change = AutocompleteChange::default();
        self.select.select(index, |idx| {
            change.selection = Some(idx);
            if !self.config.free_solo {
                if let Some(label) = on_select(idx) {
                    if !self.config.selection_control.is_controlled() {
                        self.input_value = label.clone();
                    }
                    change.input_value = Some(label);
                }
            } else {
                // Still invoke the callback so analytics/telemetry hooks fire.
                let _ = on_select(idx);
            }
        });
        change
    }

    /// Convenience accessor for the currently highlighted option.
    #[inline]
    pub fn highlighted(&self) -> Option<usize> {
        self.select.highlighted()
    }

    /// Override the highlighted option index.  Typically driven by pointer
    /// movement.
    #[inline]
    pub fn set_highlighted(&mut self, index: Option<usize>) {
        self.select.set_highlighted(index);
    }

    /// Build the ARIA/data attributes required on the `<input>` element.
    pub fn input_accessibility_attributes(
        &self,
        listbox_id: &str,
        active_id: Option<&str>,
    ) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::with_capacity(6);
        attrs.push(("role", "combobox".into()));
        let (expanded_key, expanded_value) = aria::aria_expanded(self.is_open());
        attrs.push((expanded_key, expanded_value.to_string()));
        attrs.push(("aria-controls", listbox_id.to_string()));
        let (popup_key, popup_value) = aria::aria_haspopup(aria::role_listbox());
        attrs.push((popup_key, popup_value.to_string()));
        attrs.push(("aria-autocomplete", "list".into()));
        if let Some(id) = active_id {
            attrs.push(("aria-activedescendant", id.to_string()));
        }
        aria::extend_disabled_attributes(&mut attrs, self.config.disabled);
        attrs
    }

    /// Build the ARIA attributes required on the listbox container.
    pub fn listbox_accessibility_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::with_capacity(2);
        attrs.push(("role", aria::role_listbox().into()));
        if !self.is_open() {
            attrs.push(("hidden", "true".into()));
        }
        attrs
    }

    fn open_internal(&mut self) -> AutocompleteChange {
        let mut change = AutocompleteChange::default();
        self.select.open(|flag| change.opened = Some(flag));
        change
    }

    fn close_internal(&mut self) -> AutocompleteChange {
        let mut change = AutocompleteChange::default();
        self.select.close(|flag| change.opened = Some(flag));
        change
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn focus_opens_when_configured() {
        let mut state = AutocompleteState::new(AutocompleteConfig {
            option_count: 3,
            open_on_focus: true,
            ..AutocompleteConfig::enterprise_defaults(3)
        });
        let change = state.focus();
        assert_eq!(change.opened, Some(true));
    }

    #[test]
    fn free_solo_leaves_input_intact() {
        let mut state = AutocompleteState::new(AutocompleteConfig {
            option_count: 2,
            free_solo: true,
            initial_input: "hello".into(),
            ..AutocompleteConfig::enterprise_defaults(2)
        });
        let change = state.select_index(0, |_| Some("option".into()));
        assert_eq!(change.selection, Some(0));
        assert!(change.input_value.is_none());
        assert_eq!(state.input_value(), "hello");
    }

    #[test]
    fn select_updates_input_when_not_free_solo() {
        let mut state = AutocompleteState::new(AutocompleteConfig {
            option_count: 2,
            free_solo: false,
            ..AutocompleteConfig::enterprise_defaults(2)
        });
        let change = state.select_index(1, |_| Some("Beta".into()));
        assert_eq!(change.selection, Some(1));
        assert_eq!(change.input_value, Some("Beta".into()));
        assert_eq!(state.input_value(), "Beta");
    }
}
