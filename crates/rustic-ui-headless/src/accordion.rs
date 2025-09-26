//! Accordion state machine coordinating disclosure panels for Joy UI.
//!
//! The implementation mirrors the controlled/uncontrolled approach used across
//! Material and Joy primitives.  Each accordion item tracks whether it is
//! expanded and disabled while the group enforces either single-expansion or
//! multi-expansion policies.  The API is intentionally declarative so adapters
//! only forward interaction intents (toggle, expand, collapse) and receive a
//! structured [`AccordionItemChange`] describing what changed.  Enterprise
//! automation suites can inspect that change log to assert that ARIA hooks and
//! DOM mutations fire in the expected order.

use crate::aria;

/// Aggregated change notification for an accordion item.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct AccordionItemChange {
    /// When set the item changed its expanded state.  Adapters typically pair
    /// this with DOM mutations (showing or hiding the panel) and accessibility
    /// attributes.
    pub expanded: Option<bool>,
}

impl AccordionItemChange {
    fn expanded(expanded: bool) -> Self {
        Self {
            expanded: Some(expanded),
        }
    }

    fn merge(self, other: AccordionItemChange) -> Self {
        if other.expanded.is_some() {
            other
        } else {
            self
        }
    }
}

#[derive(Debug, Clone)]
struct AccordionItemState {
    expanded: bool,
    disabled: bool,
}

impl AccordionItemState {
    fn new(expanded: bool) -> Self {
        Self {
            expanded,
            disabled: false,
        }
    }
}

/// High level accordion orchestrator.
#[derive(Debug, Clone)]
pub struct AccordionGroupState {
    allow_multiple: bool,
    items: Vec<AccordionItemState>,
}

impl AccordionGroupState {
    /// Build a new accordion group.
    ///
    /// * `item_count` — number of accordion items currently rendered.
    /// * `allow_multiple` — whether multiple items may be expanded at once.
    /// * `default_expanded` — optional indices that should start expanded.
    pub fn new(item_count: usize, allow_multiple: bool, default_expanded: &[usize]) -> Self {
        let mut items = Vec::with_capacity(item_count);
        for index in 0..item_count {
            let expanded = default_expanded.contains(&index);
            items.push(AccordionItemState::new(expanded));
        }
        let mut state = Self {
            allow_multiple,
            items,
        };
        if !allow_multiple {
            state.enforce_single_expansion();
        }
        state
    }

    /// Returns how many items the accordion currently manages.
    #[inline]
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Returns whether the provided index is expanded.
    #[inline]
    pub fn is_expanded(&self, index: usize) -> bool {
        self.items
            .get(index)
            .map(|item| item.expanded)
            .unwrap_or(false)
    }

    /// Returns whether the provided index is disabled.
    #[inline]
    pub fn is_disabled(&self, index: usize) -> bool {
        self.items
            .get(index)
            .map(|item| item.disabled)
            .unwrap_or(true)
    }

    /// Update the disabled flag for a specific item.
    pub fn set_disabled(&mut self, index: usize, disabled: bool) {
        if let Some(item) = self.items.get_mut(index) {
            item.disabled = disabled;
        }
    }

    /// Expand a specific accordion item.
    pub fn expand<F: FnOnce(usize, bool)>(
        &mut self,
        index: usize,
        notify: F,
    ) -> AccordionItemChange {
        if !self.toggleable(index) {
            return AccordionItemChange::default();
        }
        let change = self.set_expanded(index, true);
        notify(index, true);
        change
    }

    /// Collapse a specific accordion item.
    pub fn collapse<F: FnOnce(usize, bool)>(
        &mut self,
        index: usize,
        notify: F,
    ) -> AccordionItemChange {
        if !self.toggleable(index) {
            return AccordionItemChange::default();
        }
        let change = self.set_expanded(index, false);
        notify(index, false);
        change
    }

    /// Toggle a specific accordion item.
    pub fn toggle<F: FnOnce(usize, bool)>(
        &mut self,
        index: usize,
        notify: F,
    ) -> AccordionItemChange {
        if !self.toggleable(index) {
            return AccordionItemChange::default();
        }
        let next = !self.is_expanded(index);
        let change = self.set_expanded(index, next);
        notify(index, next);
        change
    }

    /// Synchronise expanded state when controlled externally.
    pub fn sync_expanded(&mut self, index: usize, expanded: bool) {
        if let Some(item) = self.items.get_mut(index) {
            item.expanded = expanded;
            if expanded && !self.allow_multiple {
                self.collapse_others(index);
            }
        }
    }

    /// Ensure the internal vector has the desired size.
    pub fn set_item_count(&mut self, count: usize) {
        if count == self.items.len() {
            return;
        }
        self.items
            .resize_with(count, || AccordionItemState::new(false));
        if !self.allow_multiple {
            self.enforce_single_expansion();
        }
    }

    /// Build the ARIA/data attributes for the summary button.
    pub fn summary_accessibility_attributes(
        &self,
        index: usize,
        panel_id: &str,
    ) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::with_capacity(4);
        attrs.push(("role", aria::role_button().into()));
        let (expanded_key, expanded_value) = aria::aria_expanded(self.is_expanded(index));
        attrs.push((expanded_key, expanded_value.to_string()));
        attrs.push(("aria-controls", panel_id.to_string()));
        aria::extend_disabled_attributes(&mut attrs, self.is_disabled(index));
        attrs
    }

    /// Build the ARIA/data attributes for the details panel.
    pub fn details_accessibility_attributes(
        &self,
        index: usize,
        summary_id: &str,
    ) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::with_capacity(4);
        attrs.push(("role", "region".into()));
        attrs.push(("aria-labelledby", summary_id.to_string()));
        if !self.is_expanded(index) {
            attrs.push(("hidden", "true".into()));
        }
        attrs
    }

    fn toggleable(&self, index: usize) -> bool {
        if let Some(item) = self.items.get(index) {
            !item.disabled
        } else {
            false
        }
    }

    fn set_expanded(&mut self, index: usize, expanded: bool) -> AccordionItemChange {
        if let Some(item) = self.items.get_mut(index) {
            if item.expanded == expanded {
                return AccordionItemChange::default();
            }
            item.expanded = expanded;
            let mut change = AccordionItemChange::expanded(expanded);
            if expanded && !self.allow_multiple {
                change = change.merge(self.collapse_others(index));
            }
            change
        } else {
            AccordionItemChange::default()
        }
    }

    fn collapse_others(&mut self, keep: usize) -> AccordionItemChange {
        let mut change = AccordionItemChange::default();
        for (index, item) in self.items.iter_mut().enumerate() {
            if index == keep {
                continue;
            }
            if item.expanded {
                item.expanded = false;
                change = change.merge(AccordionItemChange::expanded(false));
            }
        }
        change
    }

    fn enforce_single_expansion(&mut self) {
        let mut first_expanded = None;
        for (index, item) in self.items.iter_mut().enumerate() {
            if item.expanded {
                if first_expanded.is_none() {
                    first_expanded = Some(index);
                } else {
                    item.expanded = false;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_expansion_collapse_previous() {
        let mut group = AccordionGroupState::new(3, false, &[0]);
        assert!(group.is_expanded(0));
        assert!(!group.is_expanded(1));
        let change = group.toggle(1, |_, _| {});
        assert!(change.expanded.is_some());
        assert!(!group.is_expanded(0));
        assert!(group.is_expanded(1));
    }

    #[test]
    fn multiple_expansion_allows_parallel_panels() {
        let mut group = AccordionGroupState::new(3, true, &[]);
        group.expand(0, |_, _| {});
        group.expand(1, |_, _| {});
        assert!(group.is_expanded(0));
        assert!(group.is_expanded(1));
    }

    #[test]
    fn disabled_items_ignore_toggle_requests() {
        let mut group = AccordionGroupState::new(2, false, &[]);
        group.set_disabled(1, true);
        let change = group.toggle(1, |_, _| panic!("should not toggle"));
        assert_eq!(change, AccordionItemChange::default());
        assert!(!group.is_expanded(1));
    }

    #[test]
    fn summary_attributes_reflect_state() {
        let group = AccordionGroupState::new(1, false, &[0]);
        let attrs = group.summary_accessibility_attributes(0, "panel");
        assert!(attrs
            .iter()
            .any(|(k, v)| *k == "aria-controls" && v == "panel"));
        assert!(attrs
            .iter()
            .any(|(k, v)| *k == "aria-expanded" && v == "true"));
    }
}
