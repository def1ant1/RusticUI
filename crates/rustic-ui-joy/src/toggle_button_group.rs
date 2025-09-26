//! Joy toggle button group scaffolding wrapping the headless implementation.
//!
//! Exposing the controller early lets adapters experiment with layouts while we
//! finish the styled components.  Automation suites can hook into
//! [`ToggleButtonGroupChange`] without depending on JSX/TSX internals.

pub use mui_headless::toggle_button_group::{
    ToggleButtonGroupChange, ToggleButtonGroupConfig, ToggleButtonGroupState,
};

/// Wrapper around [`ToggleButtonGroupState`] that mirrors Joy’s ergonomics.
#[derive(Debug, Clone)]
pub struct ToggleButtonGroupController {
    /// Headless state machine powering the toggle group.
    pub state: ToggleButtonGroupState,
}

impl ToggleButtonGroupController {
    /// Construct a controller using Joy defaults.
    pub fn new(config: ToggleButtonGroupConfig) -> Self {
        Self {
            state: ToggleButtonGroupState::new(config),
        }
    }

    /// Helper that creates a non-exclusive group with the provided count.
    pub fn with_button_count(button_count: usize) -> Self {
        Self::new(ToggleButtonGroupConfig::enterprise_defaults(button_count))
    }
}
