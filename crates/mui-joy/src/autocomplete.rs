//! Joy autocomplete scaffolding built on top of the shared headless state.
//!
//! The wrapper exposes the strongly typed configuration and change stream so
//! renderers can focus on layout/theming.  When the visual implementation is
//! ready we simply plug the controller into the adapter without re-threading
//! business logic.

pub use mui_headless::autocomplete::{
    AutocompleteChange, AutocompleteConfig, AutocompleteControlStrategy, AutocompleteState,
};

/// Wrapper that owns an [`AutocompleteState`] ready to be plugged into Joy
/// renderers.
#[derive(Debug, Clone)]
pub struct AutocompleteController {
    /// Headless state machine handling all interactions.
    pub state: AutocompleteState,
}

impl AutocompleteController {
    /// Construct a new controller using [`AutocompleteConfig::enterprise_defaults`].
    pub fn new(config: AutocompleteConfig) -> Self {
        Self {
            state: AutocompleteState::new(config),
        }
    }

    /// Convenience helper building a controller with Joy defaults for a given
    /// option count.
    pub fn with_option_count(option_count: usize) -> Self {
        Self::new(AutocompleteConfig::enterprise_defaults(option_count))
    }
}
