//! State machine powering headless select/listbox components.
//!
//! The implementation keeps track of open state, the currently highlighted
//! option, the committed selection and a rolling typeahead buffer.  Framework
//! adapters can drive the state machine through the provided public API to
//! implement either controlled or uncontrolled widgets.

use crate::aria;
use crate::interaction::ControlKey;
use crate::selection::{clamp_index, wrap_index, ControlStrategy, TypeaheadBuffer};
use std::time::Duration;

/// Default timeout before the typeahead buffer resets.  The value mirrors the
/// recommendation from the WAI-ARIA authoring guide.
const TYPEAHEAD_TIMEOUT: Duration = Duration::from_millis(1000);

/// Headless select/listbox state machine.
#[derive(Debug, Clone)]
pub struct SelectState {
    option_count: usize,
    highlighted: Option<usize>,
    selected: Option<usize>,
    open: bool,
    open_mode: ControlStrategy,
    selection_mode: ControlStrategy,
    typeahead: TypeaheadBuffer,
}

impl SelectState {
    /// Create a new select state machine.
    ///
    /// * `option_count` — number of options currently rendered.
    /// * `initial_selected` — zero based index of the pre-selected option.
    /// * `default_open` — whether the popover starts open (uncontrolled mode).
    /// * `open_mode` — describes if the open state is controlled externally.
    /// * `selection_mode` — describes if the selected value is controlled.
    pub fn new(
        option_count: usize,
        initial_selected: Option<usize>,
        default_open: bool,
        open_mode: ControlStrategy,
        selection_mode: ControlStrategy,
    ) -> Self {
        let selected = clamp_index(initial_selected, option_count);
        let highlighted = selected.or_else(|| if option_count > 0 { Some(0) } else { None });
        Self {
            option_count,
            highlighted,
            selected,
            open: if open_mode.is_controlled() {
                false
            } else {
                default_open
            },
            open_mode,
            selection_mode,
            typeahead: TypeaheadBuffer::new(TYPEAHEAD_TIMEOUT),
        }
    }

    /// Returns the total number of options.
    #[inline]
    pub fn option_count(&self) -> usize {
        self.option_count
    }

    /// Synchronizes the internal option count with the UI.
    ///
    /// The method clamps the selection and highlighted indices to prevent
    /// referencing stale entries when options are dynamically removed.
    pub fn set_option_count(&mut self, count: usize) {
        self.option_count = count;
        self.selected = clamp_index(self.selected, count);
        self.highlighted = clamp_index(self.highlighted, count)
            .or_else(|| self.selected)
            .or_else(|| if count > 0 { Some(0) } else { None });
    }

    /// Returns whether the listbox popover is currently visible.
    #[inline]
    pub fn is_open(&self) -> bool {
        self.open
    }

    /// Returns the currently highlighted option index.
    #[inline]
    pub fn highlighted(&self) -> Option<usize> {
        self.highlighted
    }

    /// Returns the committed selection.
    #[inline]
    pub fn selected(&self) -> Option<usize> {
        self.selected
    }

    /// Imperatively set the open state (uncontrolled mode) or emit an intent to
    /// open the popover (controlled mode).
    pub fn open<F: FnOnce(bool)>(&mut self, notify: F) {
        self.set_open(true, notify);
    }

    /// Imperatively set the closed state (uncontrolled mode) or emit an intent
    /// to close the popover (controlled mode).
    pub fn close<F: FnOnce(bool)>(&mut self, notify: F) {
        self.set_open(false, notify);
    }

    /// Toggle between open and closed states.
    pub fn toggle<F: FnOnce(bool)>(&mut self, notify: F) {
        self.set_open(!self.open, notify);
    }

    /// Synchronize the open flag when the value is owned by the parent.
    pub fn sync_open(&mut self, open: bool) {
        self.open = open;
        if open {
            self.ensure_highlight();
        } else {
            self.typeahead.reset();
        }
    }

    /// Synchronize the selected option when the value is controlled by a
    /// parent.  The highlighted option is also aligned to the controlled value
    /// to preserve the active descendant relationship.
    pub fn sync_selected(&mut self, selected: Option<usize>) {
        self.selected = clamp_index(selected, self.option_count);
        if self.selection_mode.is_controlled() {
            if self.selected.is_some() {
                self.highlighted = self.selected;
            } else {
                self.highlighted = clamp_index(self.highlighted, self.option_count);
            }
        }
    }

    /// Manually override the highlighted index.  This is primarily used by
    /// adapters when focus moves via pointer interaction.
    pub fn set_highlighted(&mut self, index: Option<usize>) {
        self.highlighted = clamp_index(index, self.option_count);
    }

    /// Selects the provided option index, invoking the supplied callback.
    pub fn select<F: FnMut(usize)>(&mut self, index: usize, mut on_select: F) {
        if index >= self.option_count {
            return;
        }
        self.highlighted = Some(index);
        if !self.selection_mode.is_controlled() {
            self.selected = Some(index);
        }
        on_select(index);
    }

    /// Commits the current highlight if present.
    pub fn select_highlighted<F: FnMut(usize)>(&mut self, mut on_select: F) {
        if let Some(index) = self.highlighted {
            self.select(index, &mut on_select);
        }
    }

    /// Handle navigation keys by moving the highlight or committing the
    /// selection.  The method returns the new highlighted index so adapters can
    /// react (for example by scrolling the active option into view).
    pub fn on_key<F: FnMut(usize)>(&mut self, key: ControlKey, on_select: F) -> Option<usize> {
        match key {
            ControlKey::Enter | ControlKey::Space => {
                self.select_highlighted(on_select);
            }
            ControlKey::Home => {
                self.highlighted = if self.option_count > 0 { Some(0) } else { None };
            }
            ControlKey::End => {
                self.highlighted = if self.option_count > 0 {
                    Some(self.option_count - 1)
                } else {
                    None
                };
            }
            _ if key.is_forward() => {
                self.ensure_highlight();
                self.highlighted = wrap_index(self.highlighted, 1, self.option_count);
            }
            _ if key.is_backward() => {
                self.ensure_highlight();
                self.highlighted = wrap_index(self.highlighted, -1, self.option_count);
            }
            _ => {}
        }
        self.highlighted
    }

    /// Handle printable key input by updating the typeahead buffer and asking
    /// the provided matcher to resolve the index of the matching option.
    ///
    /// The matcher receives the full query, the currently highlighted index and
    /// the option count.  When it returns a new index the highlight (and
    /// selection for uncontrolled widgets) is updated before invoking the
    /// supplied callback.
    pub fn on_typeahead<F, G>(&mut self, ch: char, matcher: F, mut on_select: G)
    where
        F: Fn(&str, Option<usize>, usize) -> Option<usize>,
        G: FnMut(usize),
    {
        let query = self.typeahead.push(ch);
        if let Some(index) = matcher(query, self.highlighted, self.option_count) {
            self.highlighted = Some(index);
            if !self.selection_mode.is_controlled() {
                self.selected = Some(index);
            }
            on_select(index);
        }
    }

    /// Returns the ARIA role of the trigger element.  Select popovers are
    /// typically toggled by a button per the WAI-ARIA practices.
    #[inline]
    pub fn trigger_role(&self) -> &'static str {
        aria::role_button()
    }

    /// Returns the `aria-haspopup="listbox"` tuple for the trigger element.
    #[inline]
    pub fn trigger_haspopup(&self) -> (&'static str, &'static str) {
        aria::aria_haspopup("listbox")
    }

    /// Returns the `aria-expanded` attribute for the trigger element.
    #[inline]
    pub fn trigger_expanded(&self) -> (&'static str, &'static str) {
        aria::aria_expanded(self.open)
    }

    /// Returns the ARIA role for the list element (listbox).
    #[inline]
    pub fn list_role(&self) -> &'static str {
        aria::role_listbox()
    }

    /// Returns the ARIA role for an option element.
    #[inline]
    pub fn option_role(&self) -> &'static str {
        aria::role_option()
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
        if self.option_count == 0 {
            self.highlighted = None;
            return;
        }
        if self.highlighted.is_some() {
            self.highlighted = clamp_index(self.highlighted, self.option_count);
            if self.highlighted.is_some() {
                return;
            }
        }
        self.highlighted =
            self.selected
                .or_else(|| if self.option_count > 0 { Some(0) } else { None });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn noop(_: usize) {}

    fn sample_matcher(query: &str, _: Option<usize>, _: usize) -> Option<usize> {
        match query {
            "a" => Some(0),
            "ap" => Some(1),
            "c" => Some(2),
            _ => None,
        }
    }

    fn skip_disabled(query: &str, current: Option<usize>, len: usize) -> Option<usize> {
        if query == "c" {
            current
        } else {
            sample_matcher(query, current, len)
        }
    }

    #[test]
    fn keyboard_navigation_table_driven() {
        struct Case {
            name: &'static str,
            option_count: usize,
            initial_selected: Option<usize>,
            keys: &'static [ControlKey],
            expect_highlight: Option<usize>,
        }

        let cases = [
            Case {
                name: "wraps_backward_from_first",
                option_count: 3,
                initial_selected: Some(0),
                keys: &[ControlKey::ArrowUp],
                expect_highlight: Some(2),
            },
            Case {
                name: "wraps_forward_from_last",
                option_count: 3,
                initial_selected: Some(2),
                keys: &[ControlKey::ArrowDown],
                expect_highlight: Some(0),
            },
            Case {
                name: "home_key_moves_to_first",
                option_count: 5,
                initial_selected: Some(3),
                keys: &[ControlKey::Home],
                expect_highlight: Some(0),
            },
            Case {
                name: "end_key_moves_to_last",
                option_count: 5,
                initial_selected: Some(0),
                keys: &[ControlKey::End],
                expect_highlight: Some(4),
            },
            Case {
                name: "empty_select_has_no_highlight",
                option_count: 0,
                initial_selected: None,
                keys: &[ControlKey::ArrowDown, ControlKey::ArrowUp],
                expect_highlight: None,
            },
        ];

        for case in cases {
            let mut state = SelectState::new(
                case.option_count,
                case.initial_selected,
                false,
                ControlStrategy::Uncontrolled,
                ControlStrategy::Uncontrolled,
            );

            let mut last = state.highlighted();
            for key in case.keys {
                last = state.on_key(*key, noop);
            }
            assert_eq!(
                last, case.expect_highlight,
                "{}: unexpected highlight",
                case.name
            );
        }
    }

    #[test]
    fn controlled_vs_uncontrolled_selection_sync() {
        // Uncontrolled widgets update the backing field immediately.
        let mut uncontrolled = SelectState::new(
            3,
            Some(1),
            false,
            ControlStrategy::Uncontrolled,
            ControlStrategy::Uncontrolled,
        );
        uncontrolled.select(2, noop);
        assert_eq!(uncontrolled.selected(), Some(2));

        // Controlled widgets emit intents but require the parent to synchronize
        // state explicitly.
        let mut controlled = SelectState::new(
            3,
            Some(1),
            false,
            ControlStrategy::Uncontrolled,
            ControlStrategy::Controlled,
        );
        controlled.select(2, noop);
        assert_eq!(controlled.selected(), Some(1));
        controlled.sync_selected(Some(2));
        assert_eq!(controlled.selected(), Some(2));
        controlled.sync_selected(None);
        assert_eq!(controlled.selected(), None);
    }

    #[test]
    fn typeahead_cases_cover_disabled_and_rapid_sequences() {
        struct Case {
            name: &'static str,
            sequence: &'static [char],
            matcher: fn(&str, Option<usize>, usize) -> Option<usize>,
            expect_selected: Option<usize>,
            expect_highlight: Option<usize>,
        }

        let cases = [
            Case {
                name: "single_key_selects_and_highlights",
                sequence: &['c'],
                matcher: sample_matcher,
                expect_selected: Some(2),
                expect_highlight: Some(2),
            },
            Case {
                name: "disabled_option_does_not_select",
                sequence: &['c'],
                matcher: skip_disabled,
                expect_selected: Some(0),
                expect_highlight: Some(0),
            },
            Case {
                name: "rapid_sequence_uses_full_buffer",
                sequence: &['a', 'p'],
                matcher: sample_matcher,
                expect_selected: Some(1),
                expect_highlight: Some(1),
            },
        ];

        for case in cases {
            let mut state = SelectState::new(
                3,
                Some(0),
                false,
                ControlStrategy::Uncontrolled,
                ControlStrategy::Uncontrolled,
            );

            for ch in case.sequence {
                state.on_typeahead(*ch, case.matcher, noop);
            }

            assert_eq!(
                state.selected(),
                case.expect_selected,
                "{}: unexpected selection",
                case.name
            );
            assert_eq!(
                state.highlighted(),
                case.expect_highlight,
                "{}: unexpected highlight",
                case.name
            );
        }
    }

    #[test]
    fn open_state_and_aria_contract() {
        let mut uncontrolled = SelectState::new(
            2,
            Some(0),
            false,
            ControlStrategy::Uncontrolled,
            ControlStrategy::Uncontrolled,
        );
        let mut intents = Vec::new();
        uncontrolled.toggle(|flag| intents.push(flag));
        assert!(uncontrolled.is_open());
        assert_eq!(intents, vec![true]);
        assert_eq!(uncontrolled.trigger_role(), "button");
        assert_eq!(
            uncontrolled.trigger_haspopup(),
            ("aria-haspopup", "listbox")
        );
        assert_eq!(uncontrolled.trigger_expanded(), ("aria-expanded", "true"));
        assert_eq!(uncontrolled.list_role(), "listbox");
        assert_eq!(uncontrolled.option_role(), "option");

        let mut controlled = SelectState::new(
            2,
            Some(0),
            false,
            ControlStrategy::Controlled,
            ControlStrategy::Controlled,
        );
        let mut observed = Vec::new();
        controlled.open(|flag| observed.push(flag));
        assert!(!controlled.is_open());
        controlled.sync_open(true);
        assert!(controlled.is_open());
        controlled.sync_selected(Some(1));
        assert_eq!(controlled.highlighted(), Some(1));
        controlled.sync_open(false);
        assert!(!controlled.is_open());
    }
}
