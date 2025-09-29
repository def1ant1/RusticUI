//! Headless alert state machine for toast/snackbar integrations.
//!
//! Alerts frequently coordinate timers, severity styling, and ARIA
//! announcements. This state captures the relevant metadata so renderers can
//! apply consistent markup across all frameworks.

/// Severity level exposed to adapters for styling.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlertSeverity {
    /// Informational message.
    Info,
    /// Positive confirmation.
    Success,
    /// Warning that does not block the workflow.
    Warning,
    /// Critical failure.
    Error,
}

impl AlertSeverity {
    const fn as_str(self) -> &'static str {
        match self {
            Self::Info => "info",
            Self::Success => "success",
            Self::Warning => "warning",
            Self::Error => "error",
        }
    }
}

/// Headless alert state.
#[derive(Debug, Clone)]
pub struct AlertState {
    severity: AlertSeverity,
    open: bool,
    automation_id: Option<String>,
}

impl AlertState {
    /// Create a new alert state.
    pub fn new(severity: AlertSeverity) -> Self {
        Self {
            severity,
            open: true,
            automation_id: None,
        }
    }

    /// Attach an automation id.
    pub fn with_automation_id(mut self, id: impl Into<String>) -> Self {
        self.automation_id = Some(id.into());
        self
    }

    /// Toggle visibility.
    pub fn set_open(&mut self, open: bool) {
        self.open = open;
    }

    /// Returns whether the alert is visible.
    pub fn is_open(&self) -> bool {
        self.open
    }

    /// Returns the severity.
    pub fn severity(&self) -> AlertSeverity {
        self.severity
    }

    /// Returns ARIA/data attributes for the alert element.
    pub fn aria_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::with_capacity(4);
        attrs.push(("role", "alert".into()));
        attrs.push(("data-severity", self.severity.as_str().into()));
        if !self.open {
            attrs.push(("aria-hidden", "true".into()));
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
    fn aria_attributes_reflect_state() {
        let mut state = AlertState::new(AlertSeverity::Warning).with_automation_id("billing.alert");
        state.set_open(false);
        let attrs = state.aria_attributes();
        assert!(attrs
            .iter()
            .any(|(k, v)| k == &"data-severity" && v == "warning"));
        assert!(attrs
            .iter()
            .any(|(k, v)| k == &"aria-hidden" && v == "true"));
        assert!(attrs
            .iter()
            .any(|(k, v)| k == &"data-automation-id" && v == "billing.alert"));
    }
}
