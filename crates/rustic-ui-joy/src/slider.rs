//! Joy slider scaffolding mirroring the shared headless state machine.
//!
//! While the visual rendering is pending, adapters can start integrating the
//! [`SliderController`] to wire up keyboard/pointer handling and analytics.  The
//! controller simply wraps the reusable [`SliderState`] so state transitions stay
//! centralised.

pub use mui_headless::slider::{SliderChange, SliderConfig, SliderOrientation, SliderState};

/// Wrapper owning a [`SliderState`] for Joy renderers.
#[derive(Debug, Clone)]
pub struct SliderController {
    /// Headless state machine responsible for value updates.
    pub state: SliderState,
}

impl SliderController {
    /// Construct a controller using Joy friendly defaults.
    pub fn new(config: SliderConfig) -> Self {
        Self {
            state: SliderState::new(config),
        }
    }

    /// Convenience helper building a slider that spans the provided range.
    pub fn range(min: f64, max: f64) -> Self {
        Self::new(SliderConfig::enterprise_defaults(min, max))
    }
}
