//! Headless state machine modelling a generic form control shell.
//!
//! The goal is to centralise focus/error/required bookkeeping so rendering
//! adapters can project the same semantics across React-style SSR targets and
//! WebAssembly front-ends.  The state machine intentionally exposes both
//! controlled and uncontrolled APIs to accommodate automation-heavy enterprise
//! flows.  Controlled mode expects the caller to re-sync the value after
//! receiving [`FormControlChange`] whereas uncontrolled mode mutates the internal
//! buffer directly.  Both paths guarantee deterministic ordering which is vital
//! for replaying state transitions under concurrency stress tests.
//!
//! ## ARIA expectations
//!
//! * The root element must expose `role="group"` when `aria-labelledby` is
//!   present so assistive tech announces the relationship between label, helper
//!   text, and the control.
//! * `aria-describedby` should include helper text, validation messages, and any
//!   automation specific probe identifiers.
//! * `aria-invalid` is emitted whenever the control reports an error state.
//!
//! All attribute generation is handled by [`FormControlState::aria_attributes`]
//! which adapters can feed directly into DOM attribute builders.
//!
//! ## Concurrency and performance considerations
//!
//! The state machine keeps mutating operations O(1) and avoids heap allocations
//! when possible.  `String` fields are preallocated using enterprise friendly
//! defaults so high frequency updates (for example masked inputs) do not thrash
//! the allocator.  Clone operations are cheap due to the small data footprint,
//! keeping cross-thread hand-offs viable for multi-isolate runtimes.

use std::borrow::Cow;

/// Operating mode used when constructing a [`FormControlState`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FormControlMode {
    /// Caller owns the value and is responsible for synchronising it.
    Controlled,
    /// The state machine owns the value and mutates it internally.
    Uncontrolled,
}

/// Serializable change descriptor returned by mutation APIs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FormControlChange<'a> {
    /// The current value when operating in uncontrolled mode.
    pub value: Option<Cow<'a, str>>,
    /// Flag indicating whether the control became dirty.
    pub dirty: bool,
}

impl<'a> FormControlChange<'a> {
    fn dirty_value(value: Option<Cow<'a, str>>) -> Self {
        Self { value, dirty: true }
    }

    const fn inert() -> Self {
        Self {
            value: None,
            dirty: false,
        }
    }
}

/// Immutable configuration used to bootstrap [`FormControlState`].
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct FormControlConfig {
    /// Optional HTML id applied to the root element.
    pub id: Option<String>,
    /// List of ids describing helper or error text nodes.
    pub described_by: Vec<String>,
    /// Identifier for the label element.
    pub labelled_by: Option<String>,
    /// Whether the control is disabled.
    pub disabled: bool,
    /// Whether the control is required.
    pub required: bool,
    /// Whether the control currently reports a validation error.
    pub error: bool,
    /// Optional automation friendly name (e.g. analytics probe id).
    pub automation_id: Option<String>,
}

/// Headless state machine managing accessibility and focus metadata for form
/// elements.
#[derive(Debug, Clone)]
pub struct FormControlState {
    mode: FormControlMode,
    config: FormControlConfig,
    value: String,
    dirty: bool,
    focused: bool,
}

impl FormControlState {
    /// Construct a new state machine.
    pub fn new(
        initial_value: impl Into<String>,
        mode: FormControlMode,
        config: FormControlConfig,
    ) -> Self {
        Self {
            value: initial_value.into(),
            mode,
            config,
            dirty: false,
            focused: false,
        }
    }

    /// Returns whether the control is currently focused.
    #[inline]
    pub fn is_focused(&self) -> bool {
        self.focused
    }

    /// Update the focus flag.
    pub fn set_focused(&mut self, focused: bool) {
        self.focused = focused;
    }

    /// Returns whether the control is dirty.
    #[inline]
    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    /// Returns the current value as a [`&str`].
    #[inline]
    pub fn value(&self) -> &str {
        &self.value
    }

    /// Updates the internal value for uncontrolled controls.
    pub fn set_uncontrolled_value(&mut self, value: impl Into<String>) -> FormControlChange<'_> {
        if self.mode == FormControlMode::Controlled {
            return FormControlChange::inert();
        }
        let next = value.into();
        if next == self.value {
            return FormControlChange::inert();
        }
        self.value = next;
        self.dirty = true;
        FormControlChange::dirty_value(Some(Cow::Borrowed(&self.value)))
    }

    /// Surfaces the value that adapters should push to the UI when the state is
    /// controlled by an external store.
    pub fn controlled_value(&self) -> Option<&str> {
        (self.mode == FormControlMode::Controlled).then_some(self.value.as_str())
    }

    /// Record a controlled update provided by the caller.
    pub fn sync_controlled_value(&mut self, value: impl Into<String>) -> FormControlChange<'_> {
        if self.mode == FormControlMode::Controlled {
            let incoming = value.into();
            if incoming == self.value {
                return FormControlChange::inert();
            }
            self.value = incoming;
            self.dirty = true;
            FormControlChange::dirty_value(Some(Cow::Borrowed(&self.value)))
        } else {
            FormControlChange::inert()
        }
    }

    /// Toggle the validation error flag.
    pub fn set_error(&mut self, error: bool) {
        self.config.error = error;
    }

    /// Toggle the disabled flag.
    pub fn set_disabled(&mut self, disabled: bool) {
        self.config.disabled = disabled;
    }

    /// Toggle the required flag.
    pub fn set_required(&mut self, required: bool) {
        self.config.required = required;
    }

    /// Returns automation metadata.
    pub fn automation_id(&self) -> Option<&str> {
        self.config.automation_id.as_deref()
    }

    /// Returns the ARIA/data attributes describing the current state.
    pub fn aria_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::with_capacity(6);
        if let Some(id) = &self.config.id {
            attrs.push(("id", id.clone()));
        }
        if let Some(labelled_by) = &self.config.labelled_by {
            attrs.push(("aria-labelledby", labelled_by.clone()));
            attrs.push(("role", "group".into()));
        }
        if !self.config.described_by.is_empty() {
            attrs.push(("aria-describedby", self.config.described_by.join(" ")));
        }
        if self.config.error {
            attrs.push(("aria-invalid", "true".into()));
        }
        if self.config.required {
            attrs.push(("aria-required", "true".into()));
        }
        if self.config.disabled {
            attrs.push(("aria-disabled", "true".into()));
            attrs.push(("data-disabled", "true".into()));
        }
        if self.dirty {
            attrs.push(("data-dirty", "true".into()));
        }
        if self.focused {
            attrs.push(("data-focus", "within".into()));
        }
        if let Some(automation_id) = &self.config.automation_id {
            attrs.push(("data-automation-id", automation_id.clone()));
        }
        attrs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uncontrolled_updates_emit_dirty_change() {
        let mut state = FormControlState::new(
            "",
            FormControlMode::Uncontrolled,
            FormControlConfig::default(),
        );
        let change = state.set_uncontrolled_value("hello");
        assert!(change.dirty);
        assert_eq!(change.value.unwrap(), "hello");
        assert!(state.is_dirty());
    }

    #[test]
    fn controlled_updates_require_sync() {
        let mut state = FormControlState::new(
            "",
            FormControlMode::Controlled,
            FormControlConfig::default(),
        );
        let inert = state.set_uncontrolled_value("noop");
        assert!(!inert.dirty);
        let change = state.sync_controlled_value("value");
        assert!(change.dirty);
        assert_eq!(state.controlled_value(), Some("value"));
    }

    #[test]
    fn aria_attributes_include_flags() {
        let mut config = FormControlConfig::default();
        config.id = Some("fc".into());
        config.described_by = vec!["hint".into()];
        config.labelled_by = Some("label".into());
        config.error = true;
        config.required = true;
        config.disabled = true;
        config.automation_id = Some("contact.email".into());
        let mut state = FormControlState::new("", FormControlMode::Uncontrolled, config);
        state.set_focused(true);
        state.set_uncontrolled_value("v");
        let attrs = state.aria_attributes();
        assert!(attrs.iter().any(|(k, _)| k == &"aria-labelledby"));
        assert!(attrs.iter().any(|(k, v)| k == &"role" && v == "group"));
        assert!(attrs.iter().any(|(k, v)| k == &"data-dirty" && v == "true"));
        assert!(attrs
            .iter()
            .any(|(k, v)| k == &"data-focus" && v == "within"));
        assert!(attrs
            .iter()
            .any(|(k, v)| k == &"data-automation-id" && v == "contact.email"));
    }
}
