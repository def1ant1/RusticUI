//! Joy stepper scaffolding bridging the shared headless workflow engine.
//!
//! The [`StepperController`] exposes the same state machine used by Material so
//! teams can consolidate automation and analytics.  Once visual components are
//! added they simply render according to the controller’s change stream.

pub use mui_headless::stepper::{StepStatus, StepperChange, StepperConfig, StepperState};

/// Wrapper around [`StepperState`] prepared for Joy adapters.
#[derive(Debug, Clone)]
pub struct StepperController {
    /// Headless state machine powering the workflow.
    pub state: StepperState,
}

impl StepperController {
    /// Construct a controller with Joy defaults.
    pub fn new(config: StepperConfig) -> Self {
        Self {
            state: StepperState::new(config),
        }
    }

    /// Convenience helper that builds a linear stepper with the provided count.
    pub fn linear(step_count: usize) -> Self {
        Self::new(StepperConfig::enterprise_defaults(step_count))
    }
}
