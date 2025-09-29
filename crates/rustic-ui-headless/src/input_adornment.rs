//! Headless state describing start/end adornments for inputs.
//!
//! Adapters frequently need to coordinate adornment positioning, spacing tokens,
//! and ARIA metadata with the parent [`FormControlState`](crate::form_control::FormControlState).
//! This module provides a deterministic state machine with controlled and
//! uncontrolled toggles so framework integrations can expose ergonomic APIs for
//! automation heavy use cases.

/// Placement of the adornment relative to the control.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdornmentPosition {
    /// Adornment renders before the input value.
    Start,
    /// Adornment renders after the input value.
    End,
}

impl AdornmentPosition {
    const fn as_str(self) -> &'static str {
        match self {
            Self::Start => "start",
            Self::End => "end",
        }
    }
}

/// State machine powering input adornments.
#[derive(Debug, Clone)]
pub struct InputAdornmentState {
    position: AdornmentPosition,
    hidden: bool,
    inert: bool,
    automation_id: Option<String>,
}

impl InputAdornmentState {
    /// Construct a new adornment state.
    pub fn new(position: AdornmentPosition) -> Self {
        Self {
            position,
            hidden: false,
            inert: false,
            automation_id: None,
        }
    }

    /// Returns the configured position.
    pub fn position(&self) -> AdornmentPosition {
        self.position
    }

    /// Configure the automation id used for testing hooks.
    pub fn with_automation_id(mut self, id: impl Into<String>) -> Self {
        self.automation_id = Some(id.into());
        self
    }

    /// Toggle the hidden flag.
    pub fn set_hidden(&mut self, hidden: bool) {
        self.hidden = hidden;
    }

    /// Toggle the inert flag.
    pub fn set_inert(&mut self, inert: bool) {
        self.inert = inert;
    }

    /// Returns the ARIA/data attributes for the adornment element.
    pub fn aria_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::with_capacity(4);
        attrs.push(("data-adornment-position", self.position.as_str().into()));
        if self.hidden {
            attrs.push(("aria-hidden", "true".into()));
            attrs.push(("data-hidden", "true".into()));
        }
        if self.inert {
            attrs.push(("inert", "".into()));
        }
        if let Some(id) = &self.automation_id {
            attrs.push(("data-automation-id", id.clone()));
        }
        attrs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn attributes_reflect_visibility() {
        let mut state =
            InputAdornmentState::new(AdornmentPosition::Start).with_automation_id("price.start");
        state.set_hidden(true);
        state.set_inert(true);
        let attrs = state.aria_attributes();
        assert!(attrs
            .iter()
            .any(|(k, v)| k == &"aria-hidden" && v == "true"));
        assert!(attrs.iter().any(|(k, _)| k == &"inert"));
        assert!(attrs
            .iter()
            .any(|(k, v)| k == &"data-automation-id" && v == "price.start"));
    }
}
