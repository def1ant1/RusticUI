//! Linear progress state machine with determinate and buffer modes.

/// Supported modes for linear progress indicators.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LinearProgressMode {
    /// Indeterminate animation.
    Indeterminate,
    /// Determinate value in the range `[0, 1]`.
    Determinate { value: f32 },
    /// Buffer mode exposes both the value and the buffer fill.
    Buffer { value: f32, buffer: f32 },
}

/// Linear progress state.
#[derive(Debug, Clone)]
pub struct LinearProgressState {
    mode: LinearProgressMode,
    automation_id: Option<String>,
}

impl LinearProgressState {
    /// Create a new state instance.
    pub fn new(mode: LinearProgressMode) -> Self {
        Self {
            mode,
            automation_id: None,
        }
    }

    /// Attach an automation identifier.
    pub fn with_automation_id(mut self, id: impl Into<String>) -> Self {
        self.automation_id = Some(id.into());
        self
    }

    /// Update the determinate value.
    pub fn set_value(&mut self, value: f32) {
        match self.mode {
            LinearProgressMode::Determinate { .. } => {
                self.mode = LinearProgressMode::Determinate {
                    value: value.clamp(0.0, 1.0),
                };
            }
            LinearProgressMode::Buffer { buffer, .. } => {
                self.mode = LinearProgressMode::Buffer {
                    value: value.clamp(0.0, 1.0),
                    buffer,
                };
            }
            LinearProgressMode::Indeterminate => {}
        }
    }

    /// Update the buffer fill when in buffer mode.
    pub fn set_buffer(&mut self, buffer: f32) {
        if let LinearProgressMode::Buffer { value, .. } = self.mode {
            self.mode = LinearProgressMode::Buffer {
                value,
                buffer: buffer.clamp(0.0, 1.0),
            };
        }
    }

    /// Returns the current rendering mode.
    pub fn mode(&self) -> LinearProgressMode {
        self.mode
    }

    /// Returns the ARIA/data attributes for the indicator.
    pub fn aria_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::with_capacity(6);
        attrs.push(("role", "progressbar".into()));
        match self.mode {
            LinearProgressMode::Indeterminate => {
                attrs.push(("data-mode", "indeterminate".into()));
            }
            LinearProgressMode::Determinate { value } => {
                let percent = (value.clamp(0.0, 1.0) * 100.0).round();
                attrs.push(("aria-valuemin", "0".into()));
                attrs.push(("aria-valuemax", "100".into()));
                attrs.push(("aria-valuenow", percent.to_string()));
                attrs.push(("data-mode", "determinate".into()));
            }
            LinearProgressMode::Buffer { value, buffer } => {
                let value_percent = (value.clamp(0.0, 1.0) * 100.0).round();
                let buffer_percent = (buffer.clamp(0.0, 1.0) * 100.0).round();
                attrs.push(("aria-valuemin", "0".into()));
                attrs.push(("aria-valuemax", "100".into()));
                attrs.push(("aria-valuenow", value_percent.to_string()));
                attrs.push(("data-buffer", buffer_percent.to_string()));
                attrs.push(("data-mode", "buffer".into()));
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
    fn buffer_updates_preserve_range() {
        let mut state = LinearProgressState::new(LinearProgressMode::Buffer {
            value: 0.0,
            buffer: 0.5,
        });
        state.set_value(1.2);
        state.set_buffer(-0.4);
        if let LinearProgressMode::Buffer { value, buffer } = state.mode {
            assert!((value - 1.0).abs() < f32::EPSILON);
            assert!((buffer - 0.0).abs() < f32::EPSILON);
        } else {
            panic!("expected buffer mode");
        }
    }
}
