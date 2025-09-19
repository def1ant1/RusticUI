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

    #[test]
    fn uncontrolled_highlight_updates() {
        let mut state = MenuState::new(
            3,
            true,
            ControlStrategy::Uncontrolled,
            ControlStrategy::Uncontrolled,
        );
        state.on_key(ControlKey::ArrowDown);
        assert_eq!(state.highlighted(), Some(1));
    }

    #[test]
    fn controlled_highlight_requires_sync() {
        let mut state = MenuState::new(
            3,
            false,
            ControlStrategy::Uncontrolled,
            ControlStrategy::Controlled,
        );
        state.on_key(ControlKey::ArrowDown);
        assert_eq!(state.highlighted(), Some(0));
        state.sync_highlighted(Some(2));
        assert_eq!(state.highlighted(), Some(2));
    }

    #[test]
    fn typeahead_updates_highlight() {
        let mut state = MenuState::new(
            3,
            false,
            ControlStrategy::Uncontrolled,
            ControlStrategy::Uncontrolled,
        );
        state.on_typeahead('b', |query, _, _| if query == "b" { Some(1) } else { None });
        assert_eq!(state.highlighted(), Some(1));
    }
}
