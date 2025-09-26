//! Joy stepper state machine coordinating multi-step workflows.
//!
//! The implementation mirrors Material’s linear/non-linear stepper semantics so
//! enterprise experiences can reuse the same automation suites across design
//! systems.  Every transition returns a [`StepperChange`] describing which step
//! became active and which steps toggled their completion flag.  Framework
//! adapters can transform that into DOM mutations, analytics events, or custom
//! logging without duplicating the underlying rules.

use crate::aria;

/// Configuration describing how the stepper behaves.
#[derive(Debug, Clone)]
pub struct StepperConfig {
    /// Total number of steps managed by the workflow.
    pub step_count: usize,
    /// Whether the stepper enforces sequential completion.
    pub linear: bool,
    /// Optional index of the initial active step.
    pub initial_active: Option<usize>,
}

impl StepperConfig {
    /// Enterprise defaults that match Joy’s TypeScript implementation.
    pub fn enterprise_defaults(step_count: usize) -> Self {
        Self {
            step_count,
            linear: true,
            initial_active: if step_count > 0 { Some(0) } else { None },
        }
    }
}

impl Default for StepperConfig {
    fn default() -> Self {
        Self::enterprise_defaults(0)
    }
}

#[derive(Debug, Clone)]
struct StepState {
    completed: bool,
    disabled: bool,
}

impl StepState {
    fn new() -> Self {
        Self {
            completed: false,
            disabled: false,
        }
    }
}

/// Aggregate change metadata emitted from the stepper.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct StepperChange {
    /// New active step if it changed.
    pub active: Option<usize>,
    /// Index of the step whose completion flag toggled.
    pub completed: Option<usize>,
}

impl StepperChange {
    fn merge(mut self, other: StepperChange) -> StepperChange {
        if other.active.is_some() {
            self.active = other.active;
        }
        if other.completed.is_some() {
            self.completed = other.completed;
        }
        self
    }
}

/// High level stepper orchestrator.
#[derive(Debug, Clone)]
pub struct StepperState {
    steps: Vec<StepState>,
    linear: bool,
    active: Option<usize>,
}

impl StepperState {
    /// Construct a new stepper from the provided configuration.
    pub fn new(config: StepperConfig) -> Self {
        let mut steps = Vec::with_capacity(config.step_count);
        steps.resize_with(config.step_count, StepState::new);
        let active = config
            .initial_active
            .and_then(|index| {
                if index < config.step_count {
                    Some(index)
                } else {
                    None
                }
            })
            .or_else(|| if config.step_count > 0 { Some(0) } else { None });
        Self {
            steps,
            linear: config.linear,
            active,
        }
    }

    /// Returns the total number of steps.
    #[inline]
    pub fn step_count(&self) -> usize {
        self.steps.len()
    }

    /// Returns the index of the active step.
    #[inline]
    pub fn active(&self) -> Option<usize> {
        self.active
    }

    /// Returns whether the provided step index is disabled.
    #[inline]
    pub fn is_disabled(&self, index: usize) -> bool {
        self.steps
            .get(index)
            .map(|step| step.disabled)
            .unwrap_or(true)
    }

    /// Returns whether the provided step is completed.
    #[inline]
    pub fn is_completed(&self, index: usize) -> bool {
        self.steps
            .get(index)
            .map(|step| step.completed)
            .unwrap_or(false)
    }

    /// Mark a step as disabled or enabled.
    pub fn set_step_disabled(&mut self, index: usize, disabled: bool) {
        if let Some(step) = self.steps.get_mut(index) {
            step.disabled = disabled;
            if disabled && Some(index) == self.active {
                self.active = self
                    .next_available(index, 1)
                    .or_else(|| self.next_available(index, -1));
            }
        }
    }

    /// Mark the provided step as completed.
    pub fn set_step_completed(&mut self, index: usize, completed: bool) -> StepperChange {
        if let Some(step) = self.steps.get_mut(index) {
            if step.completed == completed {
                return StepperChange::default();
            }
            step.completed = completed;
            StepperChange {
                completed: Some(index),
                ..StepperChange::default()
            }
        } else {
            StepperChange::default()
        }
    }

    /// Complete the active step and optionally advance.
    pub fn complete_active(&mut self) -> StepperChange {
        if let Some(active) = self.active {
            let mut change = self.set_step_completed(active, true);
            if self.linear {
                if let Some(next) = self.next_available(active, 1) {
                    change = change.merge(self.set_active(Some(next)));
                }
            }
            change
        } else {
            StepperChange::default()
        }
    }

    /// Move to the next available step.
    pub fn next(&mut self) -> StepperChange {
        if let Some(active) = self.active {
            if let Some(next) = self.next_available(active, 1) {
                return self.set_active(Some(next));
            }
        }
        StepperChange::default()
    }

    /// Move to the previous available step.
    pub fn previous(&mut self) -> StepperChange {
        if let Some(active) = self.active {
            if let Some(prev) = self.next_available(active, -1) {
                return self.set_active(Some(prev));
            }
        }
        StepperChange::default()
    }

    /// Set the active step explicitly.
    pub fn set_active(&mut self, index: Option<usize>) -> StepperChange {
        if index == self.active {
            return StepperChange::default();
        }
        if let Some(index) = index {
            if index >= self.steps.len() || self.is_disabled(index) {
                return StepperChange::default();
            }
            if self.linear && !self.can_visit(index) {
                return StepperChange::default();
            }
            self.active = Some(index);
            StepperChange {
                active: Some(index),
                ..StepperChange::default()
            }
        } else {
            self.active = None;
            StepperChange {
                active: None,
                ..StepperChange::default()
            }
        }
    }

    /// Reset the stepper to its initial state clearing completion flags.
    pub fn reset(&mut self) {
        for step in &mut self.steps {
            step.completed = false;
            step.disabled = false;
        }
        self.active = if self.steps.is_empty() { None } else { Some(0) };
    }

    /// Compute the attributes for the clickable step button.
    pub fn step_button_attributes(&self, index: usize) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::with_capacity(4);
        attrs.push(("role", aria::role_button().into()));
        let (selected_key, selected_value) = aria::aria_selected(Some(index) == self.active);
        attrs.push((selected_key, selected_value.to_string()));
        aria::extend_disabled_attributes(&mut attrs, self.is_disabled(index));
        attrs
    }

    /// Returns a lightweight status descriptor for indicators.
    pub fn step_status(&self, index: usize) -> StepStatus {
        if self.is_disabled(index) {
            StepStatus::Disabled
        } else if Some(index) == self.active {
            StepStatus::Active
        } else if self.is_completed(index) {
            StepStatus::Completed
        } else {
            StepStatus::Pending
        }
    }

    fn next_available(&self, start: usize, delta: isize) -> Option<usize> {
        let len = self.steps.len() as isize;
        let mut index = start as isize + delta;
        while index >= 0 && index < len {
            let idx = index as usize;
            if !self.is_disabled(idx) {
                return Some(idx);
            }
            index += delta;
        }
        None
    }

    fn can_visit(&self, index: usize) -> bool {
        for i in 0..index {
            if !self.is_completed(i) {
                return false;
            }
        }
        true
    }
}

/// Describes the visual status of an individual step indicator.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StepStatus {
    /// Step has not been visited yet.
    Pending,
    /// Step is currently active.
    Active,
    /// Step has been completed.
    Completed,
    /// Step has been disabled.
    Disabled,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn linear_stepper_blocks_unfinished_steps() {
        let mut state = StepperState::new(StepperConfig::enterprise_defaults(3));
        assert_eq!(state.set_active(Some(2)), StepperChange::default());
        state.complete_active();
        assert_eq!(state.active(), Some(1));
    }

    #[test]
    fn non_linear_stepper_allows_random_access() {
        let mut state = StepperState::new(StepperConfig {
            step_count: 4,
            linear: false,
            initial_active: Some(0),
        });
        let change = state.set_active(Some(3));
        assert_eq!(change.active, Some(3));
        assert_eq!(state.active(), Some(3));
    }

    #[test]
    fn completion_updates_status() {
        let mut state = StepperState::new(StepperConfig::enterprise_defaults(2));
        state.complete_active();
        assert_eq!(state.step_status(0), StepStatus::Completed);
        assert_eq!(state.step_status(1), StepStatus::Active);
    }
}
