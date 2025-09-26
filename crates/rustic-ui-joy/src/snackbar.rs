//! Joy snackbar scaffolding exposing the shared queue/state machine.
//!
//! Enterprises frequently coordinate snackbar usage across micro-frontends.  The
//! [`SnackbarController`] centralises queue management so individual renderers
//! simply forward events into the controller and map the emitted
//! [`SnackbarChange`] into UI updates.

pub use rustic_ui_headless::snackbar::{SnackbarChange, SnackbarConfig, SnackbarMessage, SnackbarState};

/// Wrapper around [`SnackbarState`] that keeps the clock generic for tests.
#[derive(Debug, Clone)]
pub struct SnackbarController<T> {
    /// Headless snackbar state powering Joy adapters.
    pub state: SnackbarState<T>,
}

impl<T: Clone> SnackbarController<T> {
    /// Construct a controller using the system clock.
    pub fn new(config: SnackbarConfig) -> Self {
        Self {
            state: SnackbarState::new(config),
        }
    }
}
