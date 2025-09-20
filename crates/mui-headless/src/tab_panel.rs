//! Attribute helpers for tab panels.
//!
//! Panels mirror the tab attributes builder so adapters can declaratively wire
//! both sides of the relationship.  Keeping the helpers colocated makes it easy
//! to audit accessibility requirements while expanding the primitive.

use crate::aria;
use crate::tabs::TabsState;

/// Builder exposing ergonomic helpers for wiring tab panel elements.
#[derive(Debug, Clone)]
pub struct TabPanelAttributes<'a> {
    state: &'a TabsState,
    index: usize,
    id: Option<&'a str>,
    labelled_by: Option<&'a str>,
}

impl<'a> TabPanelAttributes<'a> {
    /// Create a new builder instance for the provided panel index.
    #[inline]
    pub fn new(state: &'a TabsState, index: usize) -> Self {
        Self {
            state,
            index,
            id: None,
            labelled_by: None,
        }
    }

    /// Attach an `id` attribute to the panel so tabs can reference it via
    /// `aria-controls`.
    #[inline]
    pub fn id(mut self, value: &'a str) -> Self {
        self.id = Some(value);
        self
    }

    /// Link the panel to its controlling tab using `aria-labelledby`.
    #[inline]
    pub fn labelled_by(mut self, value: &'a str) -> Self {
        self.labelled_by = Some(value);
        self
    }

    /// Returns the ARIA role for the tab panel element.
    #[inline]
    pub fn role(&self) -> &'static str {
        aria::role_tabpanel()
    }

    /// Returns the `id` attribute tuple when configured.
    #[inline]
    pub fn id_attr(&self) -> Option<(&'static str, &str)> {
        self.id.map(|value| ("id", value))
    }

    /// Returns the `aria-labelledby` tuple when configured.
    #[inline]
    pub fn aria_labelledby(&self) -> Option<(&'static str, &str)> {
        self.labelled_by.map(aria::aria_labelledby)
    }

    /// Returns whether the panel should be hidden from assistive tech.  The
    /// builder emits the boolean hidden attribute which DOM adapters can apply
    /// as-is or convert into framework specific properties.
    #[inline]
    pub fn hidden(&self) -> Option<(&'static str, &'static str)> {
        if self.state.is_selected(self.index) {
            None
        } else {
            Some(("hidden", "true"))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::selection::ControlStrategy;
    use crate::tabs::{ActivationMode, TabsOrientation};

    #[test]
    fn builder_marks_non_active_panels_hidden() {
        let state = TabsState::new(
            2,
            Some(0),
            ActivationMode::Automatic,
            TabsOrientation::Horizontal,
            ControlStrategy::Uncontrolled,
            ControlStrategy::Uncontrolled,
        );
        let hidden = state.panel(1).hidden();
        assert_eq!(hidden, Some(("hidden", "true")));
        let visible = state.panel(0).hidden();
        assert_eq!(visible, None);
    }
}
