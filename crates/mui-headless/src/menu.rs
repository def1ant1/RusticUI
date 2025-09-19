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
        Self {
            item_count,
            highlighted: if item_count > 0 { Some(0) } else { None },
            open: if open_mode.is_controlled() {
                false
            } else {
                default_open
            },
            open_mode,
            highlight_mode,
            typeahead: TypeaheadBuffer::new(TYPEAHEAD_TIMEOUT),
        }
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
        self.highlighted = clamp_index(self.highlighted, count);
        if self.highlighted.is_none() && count > 0 && !self.highlight_mode.is_controlled() {
            self.highlighted = Some(0);
        }
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
            self.highlighted = clamp_index(index, self.item_count);
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
        let next = matcher(query, self.highlighted, self.item_count);
        if self.highlight_mode.is_controlled() {
            next
        } else {
            if next.is_some() {
                self.highlighted = next;
            }
            self.highlighted
        }
    }

    /// Invoke the supplied callback for the highlighted menu item.
    pub fn activate_highlighted<F: FnMut(usize)>(&mut self, mut on_activate: F) {
        if let Some(index) = self.highlighted {
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
        if self.item_count == 0 {
            self.highlighted = None;
            return;
        }
        if self.highlight_mode.is_controlled() {
            self.highlighted = clamp_index(self.highlighted, self.item_count);
        } else if self.highlighted.is_none() {
            self.highlighted = Some(0);
        }
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

    fn disabled_lookup(query: &str, current: Option<usize>, len: usize) -> Option<usize> {
        // Delegate to the enabled matcher but treat the "b" query as disabled.
        if query == "b" {
            current
        } else {
            enabled_lookup(query, current, len)
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
            assert_eq!(
                last_seen, case.expect, "{}: highlight mismatch", case.name
            );
        }
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
    }

    #[test]
    fn typeahead_cases_handle_disabled_and_rapid_input() {
        // Menu items can be conceptually disabled by having the matcher reject
        // them.  The table clarifies how the buffer interacts with those
        // scenarios including the rapid-typeahead case where characters are
        // entered faster than the timeout window.
        struct Case {
            name: &'static str,
            expected: Option<usize>,
            matcher: fn(&str, Option<usize>, usize) -> Option<usize>,
            sequence: &'static [char],
        }

        let cases: [Case; 3] = [
            Case {
                name: "single_key_moves_highlight",
                expected: Some(1),
                matcher: enabled_lookup,
                sequence: &['b'],
            },
            Case {
                name: "disabled_item_prevents_focus_change",
                expected: Some(0),
                matcher: disabled_lookup,
                sequence: &['b'],
            },
            Case {
                name: "rapid_typeahead_uses_full_query",
                expected: Some(1),
                matcher: rapid_lookup,
                sequence: &['a', 'p'],
            },
        ];

        for case in cases {
            let mut state = MenuState::new(
                2,
                false,
                ControlStrategy::Uncontrolled,
                ControlStrategy::Uncontrolled,
            );
            state.set_highlighted(Some(0));
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
