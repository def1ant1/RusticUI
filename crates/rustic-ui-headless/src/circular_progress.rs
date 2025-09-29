//! Deterministic circular progress state machine.

/// Modes supported by the progress indicator.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ProgressMode {
    /// Determinate progress where value represents the completed percentage.
    Determinate { value: f32 },
    /// Indeterminate progress.
    Indeterminate,
}

/// Circular progress state.
#[derive(Debug, Clone)]
pub struct CircularProgressState {
    mode: ProgressMode,
    automation_id: Option<String>,
}

impl CircularProgressState {
    /// Construct a new state.
    pub fn new(mode: ProgressMode) -> Self {
        Self {
            mode,
            automation_id: None,
        }
    }

    /// Attach an automation id used by integration tests.
    pub fn with_automation_id(mut self, id: impl Into<String>) -> Self {
        self.automation_id = Some(id.into());
        self
    }

    /// Update the determinate value, clamping between 0 and 1.
    pub fn set_value(&mut self, value: f32) {
        if let ProgressMode::Determinate { .. } = self.mode {
            self.mode = ProgressMode::Determinate {
                value: value.clamp(0.0, 1.0),
            };
        }
    }

    /// Returns the current mode.
    pub fn mode(&self) -> ProgressMode {
        self.mode
    }

    /// Returns ARIA/data attributes describing the progress indicator.
    pub fn aria_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::with_capacity(4);
        attrs.push(("role", "progressbar".into()));
        match self.mode {
            ProgressMode::Determinate { value } => {
                let percent = (value.clamp(0.0, 1.0) * 100.0).round();
                attrs.push(("aria-valuemin", "0".into()));
                attrs.push(("aria-valuemax", "100".into()));
                attrs.push(("aria-valuenow", percent.to_string()));
                attrs.push(("data-mode", "determinate".into()));
            }
            ProgressMode::Indeterminate => {
                attrs.push(("data-mode", "indeterminate".into()));
            }
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
    fn determinate_updates_clamp() {
        let mut state = CircularProgressState::new(ProgressMode::Determinate { value: 0.0 });
        state.set_value(1.5);
        if let ProgressMode::Determinate { value } = state.mode() {
            assert!((value - 1.0).abs() < f32::EPSILON);
        } else {
            panic!("expected determinate");
        }
    }
}
