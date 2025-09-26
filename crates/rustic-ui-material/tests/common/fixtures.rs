//! Shared fixtures feeding the Joy parity integration tests.
//!
//! The helpers centralize representative props and headless states so every
//! adapter comparison exercises the same automation identifiers, focus
//! transitions and accessibility metadata. Keeping the data builders in one
//! place dramatically reduces maintenance overhead when design tokens evolve
//! because tests for each framework automatically reuse the updated contract.

use std::time::Duration;

use rustic_ui_headless::button::ButtonState;
use rustic_ui_headless::chip::{ChipConfig, ChipState};
use rustic_ui_headless::dialog::DialogState;
use rustic_ui_material::button::ButtonProps;
use rustic_ui_material::chip::ChipProps;
use rustic_ui_material::dialog::DialogSurfaceOptions;

/// Build button props representing a primary action used throughout the tests.
#[must_use]
pub fn button_props() -> ButtonProps {
    ButtonProps::new("Launch Joyride")
}

/// Construct a default [`ButtonState`] mirroring an idle toggle.
#[must_use]
pub fn button_state_default() -> ButtonState {
    ButtonState::new(false, None)
}

/// Assemble chip props with automation identifiers and delete affordance labels.
#[must_use]
pub fn chip_props() -> ChipProps {
    ChipProps::new("Joy filter")
        .with_automation_id("joy-chip")
        .with_delete_label("Remove joy filter")
        .with_delete_icon("✕")
}

/// Construct a chip state with timers disabled so trailing controls appear immediately.
#[must_use]
pub fn chip_state_focused() -> ChipState {
    let mut config = ChipConfig::default();
    config.show_delay = Duration::ZERO;
    config.hide_delay = Duration::ZERO;
    config.delete_delay = Duration::ZERO;
    let mut state = ChipState::new(config);
    let _ = state.focus();
    state
}

/// Create an open dialog state to exercise SSR renderers.
#[must_use]
pub fn dialog_state_open() -> DialogState {
    DialogState::uncontrolled(true)
}

/// Provide deterministic surface overrides for the dialog renderers.
#[must_use]
pub fn dialog_surface_options() -> DialogSurfaceOptions {
    let mut surface = DialogSurfaceOptions::default();
    surface.id = Some("joy-dialog".into());
    surface.labelled_by = Some("joy-dialog-heading".into());
    surface.described_by = Some("joy-dialog-description".into());
    surface.analytics_id = Some("joy-modal".into());
    surface
}

/// Return representative dialog body markup used by every adapter snapshot.
#[must_use]
pub fn dialog_body_markup() -> String {
    "<h2 id=\"joy-dialog-heading\">Joy UX</h2><p id=\"joy-dialog-description\">Amplify delight across adapters.</p>"
        .into()
}

/// Optional aria label supplementing the heading metadata.
#[must_use]
pub fn dialog_aria_label() -> Option<String> {
    Some("Joy interaction dialog".into())
}
