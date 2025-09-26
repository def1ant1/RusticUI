//! Attribute helpers for individual tabs.
//!
//! The builder centralizes ARIA bookkeeping and provides convenient helpers so
//! adapters can focus on rendering logic instead of remembering every required
//! attribute pair.  The code intentionally includes detailed notes to outline
//! the expectations from WAI-ARIA Authoring Practices for future maintainers.

use crate::aria;
use crate::tabs::TabsState;

/// Builder exposing ergonomic helpers for wiring tab elements.
#[derive(Debug, Clone)]
pub struct TabAttributes<'a> {
    state: &'a TabsState,
    index: usize,
    id: Option<&'a str>,
    controls: Option<&'a str>,
}

impl<'a> TabAttributes<'a> {
    /// Create a new tab attribute builder for the provided state/index pair.
    #[inline]
    pub fn new(state: &'a TabsState, index: usize) -> Self {
        Self {
            state,
            index,
            id: None,
            controls: None,
        }
    }

    /// Attach an `id` attribute to the tab.  This is typically used to link the
    /// tab panel via `aria-labelledby`.
    #[inline]
    pub fn id(mut self, value: &'a str) -> Self {
        self.id = Some(value);
        self
    }

    /// Link the tab with its panel using `aria-controls`.
    #[inline]
    pub fn controls(mut self, value: &'a str) -> Self {
        self.controls = Some(value);
        self
    }

    /// Returns the ARIA role for the tab element.
    #[inline]
    pub fn role(&self) -> &'static str {
        aria::role_tab()
    }

    /// Returns the `id` attribute tuple when configured.
    #[inline]
    pub fn id_attr(&self) -> Option<(&'static str, &str)> {
        self.id.map(|value| ("id", value))
    }

    /// Returns the `aria-controls` tuple when configured.
    #[inline]
    pub fn aria_controls(&self) -> Option<(&'static str, &str)> {
        self.controls.map(aria::aria_controls)
    }

    /// Returns the `aria-selected` tuple reflecting whether the tab is active.
    #[inline]
    pub fn aria_selected(&self) -> (&'static str, &'static str) {
        aria::aria_selected(self.state.is_selected(self.index))
    }

    /// Returns the recommended `tabindex` tuple implementing the roving
    /// tabindex pattern.  The focused tab is tabbable while all others are
    /// removed from the natural tab order.
    #[inline]
    pub fn tabindex(&self) -> (&'static str, &'static str) {
        if self.state.is_focused(self.index) {
            ("tabindex", "0")
        } else {
            ("tabindex", "-1")
        }
    }

    /// Convenience getter to expose whether the tab is currently focused.
    #[inline]
    pub fn is_focused(&self) -> bool {
        self.state.is_focused(self.index)
    }

    /// Convenience getter to expose whether the tab is selected.
    #[inline]
    pub fn is_selected(&self) -> bool {
        self.state.is_selected(self.index)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::selection::ControlStrategy;
    use crate::tabs::{ActivationMode, TabsOrientation};

    #[test]
    fn builder_reports_selected_and_focused_state() {
        let state = TabsState::new(
            2,
            Some(1),
            ActivationMode::Automatic,
            TabsOrientation::Horizontal,
            ControlStrategy::Uncontrolled,
            ControlStrategy::Uncontrolled,
        );
        let attrs = state.tab(1).id("tab-1").controls("panel-1");
        assert_eq!(attrs.role(), "tab");
        assert_eq!(attrs.id_attr(), Some(("id", "tab-1")));
        assert_eq!(attrs.aria_controls(), Some(("aria-controls", "panel-1")));
        assert_eq!(attrs.aria_selected(), ("aria-selected", "true"));
        assert_eq!(attrs.tabindex(), ("tabindex", "0"));
        assert!(attrs.is_selected());
        assert!(attrs.is_focused());
    }

    #[test]
    fn builder_reflects_inactive_tab_state() {
        // Secondary tabs should stay untabbable until navigation reaches them
        // so screen reader users do not land on hidden panels accidentally.
        let state = TabsState::new(
            3,
            Some(0),
            ActivationMode::Manual,
            TabsOrientation::Horizontal,
            ControlStrategy::Uncontrolled,
            ControlStrategy::Uncontrolled,
        );
        let attrs = state.tab(2);
        assert_eq!(attrs.aria_selected(), ("aria-selected", "false"));
        assert_eq!(attrs.tabindex(), ("tabindex", "-1"));
        assert!(!attrs.is_selected());
        assert!(!attrs.is_focused());
        assert_eq!(attrs.id_attr(), None);
        assert_eq!(attrs.aria_controls(), None);
    }
}
