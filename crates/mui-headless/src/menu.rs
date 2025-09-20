//! State machine for building accessible menu button patterns.
//!
//! The menu manages disclosure state, keyboard focus (highlight) and a
//! typeahead buffer allowing for rapid navigation.  Unlike the select state
//! machine, menu activation is ephemeral — callbacks fire for highlighted items
//! but no persistent selection is stored.

use crate::aria;
use crate::interaction::ControlKey;
use crate::selection::{clamp_index, wrap_index, ControlStrategy, TypeaheadBuffer};
use std::time::Duration;

const TYPEAHEAD_TIMEOUT: Duration = Duration::from_millis(1000);

/// Headless menu button state machine.
#[derive(Debug, Clone)]
pub struct MenuState {
    item_count: usize,
    /// Tracks whether each menu item index is disabled.
    ///
    /// We intentionally mirror [`item_count`] with a `Vec<bool>` so framework
    /// adapters can declaratively toggle interactivity (for example during SSR
    /// diffing or hydration) without re-sending the entire menu collection. The
    /// headless state can therefore be cloned for deterministic snapshots while
    /// still supporting O(1) enable/disable updates.
    disabled: Vec<bool>,
    highlighted: Option<usize>,
    open: bool,
    open_mode: ControlStrategy,
    highlight_mode: ControlStrategy,
    typeahead: TypeaheadBuffer,
}

impl MenuState {
    /// Construct a new menu state instance.
    ///
    /// * `item_count` — number of menu items currently rendered.
    /// * `default_open` — whether the menu starts visible when uncontrolled.
    /// * `open_mode` — describes if the open state is controlled externally.
    /// * `highlight_mode` — describes if focus management is controlled.
    pub fn new(
        item_count: usize,
        default_open: bool,
        open_mode: ControlStrategy,
        highlight_mode: ControlStrategy,
    ) -> Self {
        let mut state = Self {
            item_count,
            disabled: vec![false; item_count],
            highlighted: if item_count > 0 { Some(0) } else { None },
            open: if open_mode.is_controlled() {
                false
            } else {
                default_open
            },
            open_mode,
            highlight_mode,
            typeahead: TypeaheadBuffer::new(TYPEAHEAD_TIMEOUT),
        };
        // Ensure that freshly constructed states immediately respect disabled
        // bookkeeping so adapters can mark items inert before the first render
        // without leaving the highlight stranded on a disabled entry.
        state.ensure_highlight();
        state
    }

    /// Returns whether the menu surface is currently expanded.
    #[inline]
    pub fn is_open(&self) -> bool {
        self.open
    }

    /// Returns the highlighted menu item.
    #[inline]
    pub fn highlighted(&self) -> Option<usize> {
        self.highlighted
    }

    /// Update the number of rendered menu items.
    pub fn set_item_count(&mut self, count: usize) {
        self.item_count = count;
        self.disabled.resize(count, false);
        self.highlighted = clamp_index(self.highlighted, count);
        self.reconcile_disabled_state();
    }

    /// Returns whether the menu item at the given index is enabled.
    #[inline]
    pub fn is_item_enabled(&self, index: usize) -> bool {
        index < self.item_count && !self.disabled.get(index).copied().unwrap_or(true)
    }

    /// Returns whether the menu item at the given index is disabled.
    #[inline]
    pub fn is_item_disabled(&self, index: usize) -> bool {
        !self.is_item_enabled(index)
    }

    /// Toggle the disabled flag for a menu item.
    ///
    /// Callers can flip individual indices in response to async data loads,
    /// RBAC signals, or feature flags while letting the state machine advance
    /// the highlight to the next enabled entry in uncontrolled mode.
    pub fn set_item_disabled(&mut self, index: usize, disabled: bool) {
        if index >= self.item_count {
            return;
        }
        if let Some(slot) = self.disabled.get_mut(index) {
            *slot = disabled;
        }
        self.reconcile_disabled_state();
    }

    /// Synchronize the open flag when controlled by the parent.
    pub fn sync_open(&mut self, open: bool) {
        self.open = open;
        if open {
            self.ensure_highlight();
        } else {
            self.typeahead.reset();
        }
    }

    /// Synchronize the highlighted item when focus is controlled externally.
    pub fn sync_highlighted(&mut self, index: Option<usize>) {
        if self.highlight_mode.is_controlled() {
            self.highlighted = clamp_index(index, self.item_count);
        }
    }

    /// Imperatively set the highlighted item (uncontrolled mode).
    pub fn set_highlighted(&mut self, index: Option<usize>) {
        if !self.highlight_mode.is_controlled() {
            self.highlighted = self.normalize_index(index);
        }
    }

    /// Request the menu to open.
    pub fn open<F: FnOnce(bool)>(&mut self, notify: F) {
        self.set_open(true, notify);
    }

    /// Request the menu to close.
    pub fn close<F: FnOnce(bool)>(&mut self, notify: F) {
        self.set_open(false, notify);
    }

    /// Toggle the disclosure state.
    pub fn toggle<F: FnOnce(bool)>(&mut self, notify: F) {
        self.set_open(!self.open, notify);
    }

    /// Handle navigation keys.  Returns the new highlight so adapters can ensure
    /// the item is visible (for example by scrolling into view).
    pub fn on_key(&mut self, key: ControlKey) -> Option<usize> {
        let mut next = self.highlighted;
        match key {
            ControlKey::Home => {
                next = self.first_enabled_index();
            }
            ControlKey::End => {
                next = self.last_enabled_index();
            }
            _ if key.is_forward() => {
                self.ensure_highlight();
                next = self.advance_enabled(self.highlighted, 1);
            }
            _ if key.is_backward() => {
                self.ensure_highlight();
                next = self.advance_enabled(self.highlighted, -1);
            }
            _ => {}
        }
        if self.highlight_mode.is_controlled() {
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
        if let Some(index) = matcher(query, self.highlighted, self.item_count) {
            if self.is_item_disabled(index) {
                let normalized = self.normalize_index(Some(index));
                if !self.highlight_mode.is_controlled() {
                    self.highlighted = normalized;
                }
                return if self.highlight_mode.is_controlled() {
                    normalized
                } else {
                    self.highlighted
                };
            }
            let normalized = self.normalize_index(Some(index));
            if self.highlight_mode.is_controlled() {
                return normalized;
            }
            if normalized.is_some() {
                self.highlighted = normalized;
            }
            return self.highlighted;
        }
        if self.highlight_mode.is_controlled() {
            None
        } else {
            self.highlighted
        }
    }

    /// Invoke the supplied callback for the highlighted menu item.
    pub fn activate_highlighted<F: FnMut(usize)>(&mut self, mut on_activate: F) {
        if let Some(index) = self.highlighted {
            if self.is_item_disabled(index) {
                return;
            }
            on_activate(index);
        }
    }

    /// Returns the ARIA role for the trigger button.
    #[inline]
    pub fn trigger_role(&self) -> &'static str {
        aria::role_button()
    }

    /// Returns the `aria-haspopup="menu"` tuple for the trigger element.
    #[inline]
    pub fn trigger_haspopup(&self) -> (&'static str, &'static str) {
        aria::aria_haspopup("menu")
    }

    /// Returns the `aria-expanded` tuple for the trigger element.
    #[inline]
    pub fn trigger_expanded(&self) -> (&'static str, &'static str) {
        aria::aria_expanded(self.open)
    }

    /// Returns the ARIA role for the menu surface.
    #[inline]
    pub fn menu_role(&self) -> &'static str {
        aria::role_menu()
    }

    /// Returns the ARIA role for menu items.
    #[inline]
    pub fn item_role(&self) -> &'static str {
        aria::role_menuitem()
    }

    fn set_open<F: FnOnce(bool)>(&mut self, next: bool, notify: F) {
        if !self.open_mode.is_controlled() {
            self.open = next;
        }
        if next {
            self.ensure_highlight();
        } else {
            self.typeahead.reset();
        }
        notify(next);
    }

    fn ensure_highlight(&mut self) {
        if self.item_count == 0 || !self.has_enabled_items() {
            self.highlighted = None;
            return;
        }
        if self.highlight_mode.is_controlled() {
            self.highlighted = clamp_index(self.highlighted, self.item_count);
        } else if self.highlighted.is_none() {
            self.highlighted = self.first_enabled_index();
        } else if let Some(index) = self.highlighted {
            if self.is_item_disabled(index) {
                self.highlighted = self
                    .advance_enabled(Some(index), 1)
                    .or_else(|| self.advance_enabled(Some(index), -1))
                    .or_else(|| self.first_enabled_index());
            }
        }
    }

    fn reconcile_disabled_state(&mut self) {
        if self.item_count == 0 {
            self.disabled.clear();
            self.highlighted = None;
            return;
        }
        if !self.has_enabled_items() {
            if !self.highlight_mode.is_controlled() {
                self.highlighted = None;
            }
            return;
        }
        if self.highlight_mode.is_controlled() {
            self.highlighted = clamp_index(self.highlighted, self.item_count);
            return;
        }
        if let Some(index) = self.highlighted {
            if self.is_item_disabled(index) {
                self.highlighted = self
                    .advance_enabled(Some(index), 1)
                    .or_else(|| self.advance_enabled(Some(index), -1));
            }
        }
        if self.highlighted.is_none() {
            self.highlighted = self.first_enabled_index();
        }
    }

    fn has_enabled_items(&self) -> bool {
        self.disabled
            .iter()
            .take(self.item_count)
            .any(|flag| !*flag)
    }

    fn first_enabled_index(&self) -> Option<usize> {
        if self.item_count == 0 {
            return None;
        }
        (0..self.item_count).find(|index| self.is_item_enabled(*index))
    }

    fn last_enabled_index(&self) -> Option<usize> {
        if self.item_count == 0 {
            return None;
        }
        (0..self.item_count)
            .rev()
            .find(|index| self.is_item_enabled(*index))
    }

    fn advance_enabled(&self, current: Option<usize>, delta: isize) -> Option<usize> {
        if self.item_count == 0 || !self.has_enabled_items() {
            return None;
        }
        let mut base = match clamp_index(current, self.item_count) {
            Some(index) => index,
            None => {
                return if delta >= 0 {
                    self.first_enabled_index()
                } else {
                    self.last_enabled_index()
                };
            }
        };
        for _ in 0..self.item_count {
            base = wrap_index(Some(base), delta, self.item_count)?;
            if self.is_item_enabled(base) {
                return Some(base);
            }
        }
        None
    }

    fn normalize_index(&self, index: Option<usize>) -> Option<usize> {
        let index = clamp_index(index, self.item_count);
        if let Some(current) = index {
            if self.is_item_enabled(current) {
                return Some(current);
            }
            return self
                .advance_enabled(Some(current), 1)
                .or_else(|| self.advance_enabled(Some(current), -1));
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn enabled_lookup(query: &str, _: Option<usize>, _: usize) -> Option<usize> {
        match query {
            "a" => Some(0),
            "b" => Some(1),
            "ap" => Some(1),
            _ => None,
        }
    }

    fn rapid_lookup(query: &str, _: Option<usize>, _: usize) -> Option<usize> {
        match query {
            "ap" => Some(1),
            "a" => Some(0),
            _ => None,
        }
    }

    #[test]
    fn keyboard_navigation_table_driven() {
        // Scenario definitions keep expectations close to the inputs so future
        // maintainers can easily extend coverage without having to mentally
        // simulate the full interaction sequence.
        struct Case {
            name: &'static str,
            item_count: usize,
            initial_highlight: Option<usize>,
            keys: &'static [ControlKey],
            expect: Option<usize>,
        }

        let cases = [
            Case {
                name: "wraps_from_last_to_first",
                item_count: 3,
                initial_highlight: Some(2),
                keys: &[ControlKey::ArrowDown],
                expect: Some(0),
            },
            Case {
                name: "wraps_from_first_to_last",
                item_count: 3,
                initial_highlight: Some(0),
                keys: &[ControlKey::ArrowUp],
                expect: Some(2),
            },
            Case {
                name: "home_key_jumps_to_start",
                item_count: 5,
                initial_highlight: Some(3),
                keys: &[ControlKey::Home],
                expect: Some(0),
            },
            Case {
                name: "end_key_jumps_to_tail",
                item_count: 5,
                initial_highlight: Some(0),
                keys: &[ControlKey::End],
                expect: Some(4),
            },
            Case {
                name: "empty_menu_never_highlights",
                item_count: 0,
                initial_highlight: None,
                keys: &[ControlKey::ArrowDown, ControlKey::ArrowUp],
                expect: None,
            },
        ];

        for case in cases {
            let mut state = MenuState::new(
                case.item_count,
                false,
                ControlStrategy::Uncontrolled,
                ControlStrategy::Uncontrolled,
            );
            state.set_highlighted(case.initial_highlight);

            // Drive the sequence defined in the table and capture the final
            // highlight to assert that wrap-around logic behaves as designed.
            let mut last_seen = state.highlighted();
            for key in case.keys {
                last_seen = state.on_key(*key);
            }
            assert_eq!(last_seen, case.expect, "{}: highlight mismatch", case.name);
        }
    }

    #[test]
    fn keyboard_navigation_skips_disabled_items() {
        let mut state = MenuState::new(
            4,
            false,
            ControlStrategy::Uncontrolled,
            ControlStrategy::Uncontrolled,
        );
        state.set_item_disabled(1, true);
        state.set_item_disabled(2, true);
        state.set_highlighted(Some(0));

        // Moving forward should jump over disabled entries and wrap when needed.
        let forward = state.on_key(ControlKey::ArrowDown);
        assert_eq!(forward, Some(3));
        assert_eq!(state.highlighted(), Some(3));

        // Moving backward should likewise skip disabled items.
        let backward = state.on_key(ControlKey::ArrowUp);
        assert_eq!(backward, Some(0));
        assert_eq!(state.highlighted(), Some(0));
    }

    #[test]
    fn controlled_vs_uncontrolled_highlight_sync() {
        // Controlled highlight mode should defer state updates until the
        // component owner explicitly synchronizes a value.  Uncontrolled mode
        // mutates immediately for ergonomics.
        let mut uncontrolled = MenuState::new(
            3,
            false,
            ControlStrategy::Uncontrolled,
            ControlStrategy::Uncontrolled,
        );
        uncontrolled.on_key(ControlKey::ArrowDown);
        assert_eq!(uncontrolled.highlighted(), Some(1));

        let mut controlled = MenuState::new(
            3,
            false,
            ControlStrategy::Uncontrolled,
            ControlStrategy::Controlled,
        );
        let response = controlled.on_key(ControlKey::ArrowDown);
        // The internal highlight should not move because the owner is expected
        // to call `sync_highlighted` after handling the navigation intent.
        assert_eq!(controlled.highlighted(), Some(0));
        assert_eq!(response, Some(1));
        controlled.sync_highlighted(response);
        assert_eq!(controlled.highlighted(), Some(1));

        // Disabling the controlled highlight index should not mutate the stored
        // value — the parent component owns reconciliation.
        controlled.set_item_disabled(1, true);
        assert_eq!(controlled.highlighted(), Some(1));
        // A subsequent navigation intent still reports the next enabled index.
        assert_eq!(controlled.on_key(ControlKey::ArrowDown), Some(2));
    }

    #[test]
    fn typeahead_cases_handle_disabled_and_rapid_input() {
        // Menu items can be disabled directly via `set_item_disabled`. The
        // table clarifies how the buffer interacts with those scenarios
        // including the rapid-typeahead case where characters are entered
        // faster than the timeout window.
        struct Case {
            name: &'static str,
            expected: Option<usize>,
            matcher: fn(&str, Option<usize>, usize) -> Option<usize>,
            setup: fn(&mut MenuState),
            sequence: &'static [char],
        }

        fn disabled_matcher(query: &str, _: Option<usize>, _: usize) -> Option<usize> {
            match query {
                "b" => Some(1),
                _ => None,
            }
        }

        let cases: [Case; 3] = [
            Case {
                name: "single_key_moves_highlight",
                expected: Some(1),
                matcher: enabled_lookup,
                setup: |_| {},
                sequence: &['b'],
            },
            Case {
                name: "disabled_item_advances_to_next_enabled",
                expected: Some(2),
                matcher: disabled_matcher,
                setup: |state| state.set_item_disabled(1, true),
                sequence: &['b'],
            },
            Case {
                name: "rapid_typeahead_uses_full_query",
                expected: Some(1),
                matcher: rapid_lookup,
                setup: |_| {},
                sequence: &['a', 'p'],
            },
        ];

        for case in cases {
            let mut state = MenuState::new(
                3,
                false,
                ControlStrategy::Uncontrolled,
                ControlStrategy::Uncontrolled,
            );
            state.set_highlighted(Some(0));
            (case.setup)(&mut state);
            for ch in case.sequence {
                state.on_typeahead(*ch, case.matcher);
            }
            assert_eq!(
                state.highlighted(),
                case.expected,
                "{}: highlight mismatch after typeahead",
                case.name
            );
        }
    }

    #[test]
    fn disabling_highlight_advances_or_clears_focus() {
        let mut state = MenuState::new(
            3,
            false,
            ControlStrategy::Uncontrolled,
            ControlStrategy::Uncontrolled,
        );
        state.set_highlighted(Some(1));
        state.set_item_disabled(1, true);
        assert!(state.is_item_disabled(1));
        assert_eq!(state.highlighted(), Some(2));
        state.set_item_disabled(2, true);
        assert_eq!(state.highlighted(), Some(0));
        state.set_item_disabled(0, true);
        assert_eq!(state.highlighted(), None);
        state.set_item_disabled(0, false);
        assert!(state.is_item_enabled(0));
        assert_eq!(state.highlighted(), Some(0));
    }

    #[test]
    fn activate_highlighted_ignores_disabled_items() {
        let mut state = MenuState::new(
            2,
            false,
            ControlStrategy::Uncontrolled,
            ControlStrategy::Controlled,
        );
        state.sync_highlighted(Some(1));
        state.set_item_disabled(1, true);
        let mut observed = Vec::new();
        state.activate_highlighted(|index| observed.push(index));
        assert!(observed.is_empty());
        state.sync_highlighted(Some(0));
        state.set_item_disabled(0, false);
        state.activate_highlighted(|index| observed.push(index));
        assert_eq!(observed, vec![0]);
    }

    #[test]
    fn open_state_control_and_aria_contract() {
        let mut uncontrolled = MenuState::new(
            1,
            false,
            ControlStrategy::Uncontrolled,
            ControlStrategy::Uncontrolled,
        );
        let mut observed = Vec::new();
        uncontrolled.toggle(|open| observed.push(open));
        // Uncontrolled menus mutate internal state and still notify observers so
        // renderers can react (for example by toggling CSS classes).
        assert!(uncontrolled.is_open());
        assert_eq!(observed, vec![true]);
        assert_eq!(uncontrolled.trigger_role(), "button");
        assert_eq!(uncontrolled.trigger_haspopup(), ("aria-haspopup", "menu"));
        assert_eq!(uncontrolled.trigger_expanded(), ("aria-expanded", "true"));
        assert_eq!(uncontrolled.menu_role(), "menu");
        assert_eq!(uncontrolled.item_role(), "menuitem");

        let mut controlled = MenuState::new(
            1,
            false,
            ControlStrategy::Controlled,
            ControlStrategy::Controlled,
        );
        let mut open_events = Vec::new();
        controlled.open(|intent| open_events.push(intent));
        // Controlled menus emit intents but leave the internal flag untouched
        // until the host application calls `sync_open` with the actual value.
        assert!(!controlled.is_open());
        assert_eq!(open_events, vec![true]);
        controlled.sync_open(true);
        assert!(controlled.is_open());
        controlled.sync_open(false);
        assert!(!controlled.is_open());
    }
}
