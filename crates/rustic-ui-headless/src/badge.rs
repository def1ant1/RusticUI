//! Headless badge logic consolidating count formatting and ARIA semantics.
//!
//! The badge supports numeric counters and dot indicators.  Renderers query the
//! evaluated tokens and merge them with theme driven styles keeping
//! accessibility rules consistent across frameworks.

/// Headless representation of a badge.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BadgeState {
    value: Option<i32>,
    max: i32,
    show_zero: bool,
    dot: bool,
}

impl BadgeState {
    /// Creates a badge with an optional numeric value.
    pub fn new(value: Option<i32>) -> Self {
        Self {
            value,
            max: 99,
            show_zero: false,
            dot: false,
        }
    }

    /// Configures the maximum number shown before truncation.
    pub fn with_max(mut self, max: i32) -> Self {
        self.max = max.max(0);
        self
    }

    /// Forces the badge to remain visible when the value is zero.
    pub fn with_show_zero(mut self, show_zero: bool) -> Self {
        self.show_zero = show_zero;
        self
    }

    /// Configures whether the badge renders as a dot indicator.
    pub fn with_dot(mut self, dot: bool) -> Self {
        self.dot = dot;
        self
    }

    /// Returns the formatted value shown by renderers.
    pub fn display_value(&self) -> Option<String> {
        if self.dot {
            return None;
        }

        match self.value {
            Some(v) if v > self.max => Some(format!("{}+", self.max)),
            Some(v) if v == 0 && !self.show_zero => None,
            Some(v) => Some(v.to_string()),
            None => None,
        }
    }

    /// Returns ARIA attributes for the badge container.
    pub fn accessibility_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::with_capacity(2);
        if let Some(display) = self.display_value() {
            attrs.push(("aria-label", format!("{} notifications", display)));
            attrs.push(("data-badge-count", display));
        } else if self.dot {
            attrs.push(("aria-label", "Notifications".to_string()));
        }
        attrs
    }

    /// Returns whether the badge should render.
    pub fn is_visible(&self) -> bool {
        self.dot || self.display_value().is_some()
    }

    /// Indicates whether the badge is configured as a dot indicator.
    pub fn is_dot(&self) -> bool {
        self.dot
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_value_truncates_at_max() {
        let state = BadgeState::new(Some(120)).with_max(99);
        assert_eq!(state.display_value(), Some("99+".to_string()));
    }

    #[test]
    fn zero_is_hidden_by_default() {
        let state = BadgeState::new(Some(0));
        assert!(!state.is_visible());
    }

    #[test]
    fn dot_badges_skip_numeric_value() {
        let state = BadgeState::new(None).with_dot(true);
        assert!(state.display_value().is_none());
        assert!(state.is_visible());
    }
}
