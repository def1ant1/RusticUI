//! Headless listbox state machine powering Material data-display components.
//!
//! The state mirrors [`MenuState`](crate::menu::MenuState) but persists
//! selection so list based surfaces (plain lists, data tables) can provide rich
//! keyboard interaction, typeahead navigation and deterministic automation
//! hooks without depending on a framework runtime.  The machine intentionally
//! keeps the API small – enterprise teams frequently orchestrate selection from
//! analytics pipelines or RBAC engines, therefore the state exposes explicit
//! controlled/uncontrolled knobs via [`ControlStrategy`].

use crate::interaction::ControlKey;
use crate::selection::{clamp_index, wrap_index, ControlStrategy, TypeaheadBuffer};
use std::collections::BTreeSet;
use std::time::Duration;

const TYPEAHEAD_TIMEOUT: Duration = Duration::from_millis(1000);

/// Describes how many list items may be selected at once.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelectionMode {
    /// Selection is disabled; rows behave as static content.
    None,
    /// Only a single item may be selected. The highlighted row mirrors the
    /// selected value for screen reader parity.
    Single,
    /// Multiple items may be toggled on/off. `aria-multiselectable="true"` is
    /// exposed to consumer renderers when this mode is active.
    Multiple,
}

/// Normalized set wrapper ensuring deterministic ordering and bounds checks.
fn normalize_selection(indices: &[usize], len: usize) -> Vec<usize> {
    let mut set = BTreeSet::new();
    for index in indices.iter().copied() {
        if index < len {
            set.insert(index);
        }
    }
    set.into_iter().collect()
}

/// Remove any indices that fall outside the current item count.
fn prune_selection(selection: &mut Vec<usize>, len: usize) {
    selection.retain(|idx| *idx < len);
}

/// Headless state backing Material list and table renderers.
#[derive(Debug, Clone)]
pub struct ListState {
    item_count: usize,
    highlighted: Option<usize>,
    selection: Vec<usize>,
    selection_mode: SelectionMode,
    selection_strategy: ControlStrategy,
    highlight_strategy: ControlStrategy,
    typeahead: TypeaheadBuffer,
}

impl ListState {
    /// Construct a new list state instance.
    ///
    /// * `item_count` – number of items currently rendered.
    /// * `default_selection` – initial set of selected indices when
    ///   [`ControlStrategy::Uncontrolled`] is used for selection.
    /// * `selection_mode` – dictates how many items may be selected at once.
    /// * `selection_strategy` – whether the selection is controlled
    ///   externally.
    /// * `highlight_strategy` – whether focus/highlight is controlled
    ///   externally.
    pub(crate) fn new(
        item_count: usize,
        default_selection: &[usize],
        selection_mode: SelectionMode,
        selection_strategy: ControlStrategy,
        highlight_strategy: ControlStrategy,
    ) -> Self {
        let mut selection = if selection_strategy.is_controlled() {
            Vec::new()
        } else {
            normalize_selection(default_selection, item_count)
        };

        if matches!(selection_mode, SelectionMode::Single) {
            selection.truncate(1);
        }

        Self {
            item_count,
            highlighted: if item_count > 0 { Some(0) } else { None },
            selection,
            selection_mode,
            selection_strategy,
            highlight_strategy,
            typeahead: TypeaheadBuffer::new(TYPEAHEAD_TIMEOUT),
        }
    }

    /// Convenience constructor for uncontrolled lists where both selection and
    /// highlight are owned by the component itself.
    pub fn uncontrolled(
        item_count: usize,
        default_selection: &[usize],
        selection_mode: SelectionMode,
    ) -> Self {
        Self::new(
            item_count,
            default_selection,
            selection_mode,
            ControlStrategy::Uncontrolled,
            ControlStrategy::Uncontrolled,
        )
    }

    /// Convenience constructor for fully controlled lists where the host
    /// application owns both selection and highlight state.
    pub fn controlled(item_count: usize, selection_mode: SelectionMode) -> Self {
        Self::new(
            item_count,
            &[],
            selection_mode,
            ControlStrategy::Controlled,
            ControlStrategy::Controlled,
        )
    }

    /// Mixed control constructor allowing callers to independently control the
    /// selection and highlight behavior.
    pub fn with_control_modes(
        item_count: usize,
        default_selection: &[usize],
        selection_mode: SelectionMode,
        selection_controlled: bool,
        highlight_controlled: bool,
    ) -> Self {
        let selection_strategy = if selection_controlled {
            ControlStrategy::Controlled
        } else {
            ControlStrategy::Uncontrolled
        };
        let highlight_strategy = if highlight_controlled {
            ControlStrategy::Controlled
        } else {
            ControlStrategy::Uncontrolled
        };
        Self::new(
            item_count,
            default_selection,
            selection_mode,
            selection_strategy,
            highlight_strategy,
        )
    }

    /// Returns the number of items tracked by the state.
    #[inline]
    pub fn item_count(&self) -> usize {
        self.item_count
    }

    /// Returns the highlighted item index, if any.
    #[inline]
    pub fn highlighted(&self) -> Option<usize> {
        self.highlighted
    }

    /// Returns the current selection as a slice of indices.
    #[inline]
    pub fn selection(&self) -> &[usize] {
        &self.selection
    }

    /// Returns whether the selection includes the provided index.
    #[inline]
    pub fn is_selected(&self, index: usize) -> bool {
        self.selection.contains(&index)
    }

    /// Updates the number of tracked items.
    pub fn set_item_count(&mut self, count: usize) {
        self.item_count = count;
        self.highlighted = clamp_index(self.highlighted, count);
        prune_selection(&mut self.selection, count);
        if matches!(self.selection_mode, SelectionMode::Single) && self.selection.len() > 1 {
            self.selection.truncate(1);
        }
        if self.highlighted.is_none() && count > 0 && !self.highlight_strategy.is_controlled() {
            self.highlighted = Some(0);
        }
    }

    /// Synchronize the externally controlled highlight.
    pub fn sync_highlighted(&mut self, index: Option<usize>) {
        if self.highlight_strategy.is_controlled() {
            self.highlighted = clamp_index(index, self.item_count);
        }
    }

    /// Imperatively set the highlight when uncontrolled.
    pub fn set_highlighted(&mut self, index: Option<usize>) {
        if !self.highlight_strategy.is_controlled() {
            self.highlighted = clamp_index(index, self.item_count);
        }
    }

    /// Synchronize the selected indices when the parent owns the state.
    pub fn sync_selection(&mut self, indices: &[usize]) {
        if self.selection_strategy.is_controlled() {
            let mut next = normalize_selection(indices, self.item_count);
            if matches!(self.selection_mode, SelectionMode::Single) {
                next.truncate(1);
            }
            self.selection = next;
        }
    }

    /// Sets the current selection when uncontrolled. The provided closure is
    /// invoked with the resulting selection (even in uncontrolled mode) so
    /// analytics hooks remain informed.
    pub fn set_selection<F>(&mut self, indices: &[usize], mut notify: F)
    where
        F: FnMut(&[usize]),
    {
        let mut next = normalize_selection(indices, self.item_count);
        if matches!(self.selection_mode, SelectionMode::Single) {
            next.truncate(1);
        }
        notify(&next);
        if !self.selection_strategy.is_controlled() {
            self.selection = next;
        }
    }

    /// Toggle the provided index according to the active selection mode.
    pub fn toggle<F>(&mut self, index: usize, mut notify: F)
    where
        F: FnMut(&[usize]),
    {
        if matches!(self.selection_mode, SelectionMode::None) || index >= self.item_count {
            return;
        }

        let mut next = self.selection.clone();
        if matches!(self.selection_mode, SelectionMode::Single) {
            if next.first().copied() == Some(index) {
                next.clear();
            } else {
                next.clear();
                next.push(index);
            }
        } else if let Some(pos) = next.iter().position(|value| *value == index) {
            next.remove(pos);
        } else {
            next.push(index);
            next.sort_unstable();
        }

        notify(&next);
        if !self.selection_strategy.is_controlled() {
            self.selection = next;
        }
    }

    /// Clears the entire selection.
    pub fn clear<F>(&mut self, mut notify: F)
    where
        F: FnMut(&[usize]),
    {
        if matches!(self.selection_mode, SelectionMode::None) {
            return;
        }
        notify(&[]);
        if !self.selection_strategy.is_controlled() {
            self.selection.clear();
        }
    }

    /// Handle navigation keys returning the newly highlighted index. The
    /// caller is expected to ensure the item is visible (for example by
    /// scrolling it into view).
    pub fn on_key(&mut self, key: ControlKey) -> Option<usize> {
        let mut next = self.highlighted;
        match key {
            ControlKey::Home => {
                next = if self.item_count > 0 { Some(0) } else { None };
            }
            ControlKey::End => {
                next = if self.item_count > 0 {
                    Some(self.item_count - 1)
                } else {
                    None
                };
            }
            _ if key.is_forward() => {
                self.ensure_highlight();
                next = wrap_index(self.highlighted, 1, self.item_count);
            }
            _ if key.is_backward() => {
                self.ensure_highlight();
                next = wrap_index(self.highlighted, -1, self.item_count);
            }
            _ => {}
        }

        if self.highlight_strategy.is_controlled() {
            next
        } else {
            self.highlighted = next;
            self.highlighted
        }
    }

    /// Handle printable characters for typeahead navigation.
    pub fn on_typeahead<F>(&mut self, ch: char, matcher: F) -> Option<usize>
    where
        F: Fn(&str, Option<usize>, usize) -> Option<usize>,
    {
        let query = self.typeahead.push(ch);
        let next = matcher(query, self.highlighted, self.item_count);
        if self.highlight_strategy.is_controlled() {
            next
        } else {
            if let Some(next_idx) = next {
                self.highlighted = Some(next_idx);
            }
            self.highlighted
        }
    }

    /// Executes the callback with the highlighted index (if any).
    pub fn activate_highlighted<F>(&self, mut on_activate: F)
    where
        F: FnMut(usize),
    {
        if let Some(index) = self.highlighted {
            on_activate(index);
        }
    }

    fn ensure_highlight(&mut self) {
        if self.highlighted.is_none() && self.item_count > 0 {
            self.highlighted = Some(0);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interaction::ControlKey;

    #[test]
    fn uncontrolled_single_selection_toggles() {
        let mut state = ListState::uncontrolled(3, &[1], SelectionMode::Single);
        let mut history = Vec::new();
        state.toggle(2, |sel| history.push(sel.to_vec()));
        assert_eq!(state.selection(), &[2]);
        state.toggle(2, |sel| history.push(sel.to_vec()));
        assert!(state.selection().is_empty());
        assert_eq!(history, vec![vec![2], vec![]]);
    }

    #[test]
    fn controlled_selection_invokes_callbacks_without_mutating_state() {
        let mut state = ListState::with_control_modes(3, &[], SelectionMode::Multiple, true, false);
        let mut captured = Vec::new();
        state.toggle(1, |sel| captured.push(sel.to_vec()));
        assert!(state.selection().is_empty());
        assert_eq!(captured, vec![vec![1]]);
        state.sync_selection(&[1]);
        assert_eq!(state.selection(), &[1]);
    }

    #[test]
    fn navigation_wraps_around_items() {
        let mut state = ListState::uncontrolled(2, &[], SelectionMode::None);
        assert_eq!(state.on_key(ControlKey::ArrowDown), Some(1));
        assert_eq!(state.on_key(ControlKey::ArrowDown), Some(0));
    }

    #[test]
    fn typeahead_uses_matcher_and_updates_highlight() {
        let mut state = ListState::uncontrolled(3, &[], SelectionMode::None);
        let result =
            state.on_typeahead('a', |query, _, _| if query == "a" { Some(2) } else { None });
        assert_eq!(result, Some(2));
        assert_eq!(state.highlighted(), Some(2));
    }

    #[test]
    fn set_item_count_prunes_selection() {
        let mut state = ListState::uncontrolled(4, &[1, 3], SelectionMode::Multiple);
        state.set_item_count(2);
        assert_eq!(state.selection(), &[1]);
        assert_eq!(state.highlighted(), Some(0));
    }
}
