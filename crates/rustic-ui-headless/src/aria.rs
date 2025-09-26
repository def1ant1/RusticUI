//! Helpers for generating ARIA attributes.
//! Keeping these utilities centralized ensures accessibility semantics
//! stay consistent across framework adapters.

use crate::toggle::ToggleCheckedState;

/// Enumerates the values accepted by the `aria-checked` attribute.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AriaChecked {
    /// Represents an unchecked control (`false`).
    False,
    /// Represents a checked control (`true`).
    True,
    /// Represents an indeterminate control (`mixed`).
    Mixed,
}

impl AriaChecked {
    /// Convert the enum variant into the string expected by automation tools.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::False => "false",
            Self::True => "true",
            Self::Mixed => "mixed",
        }
    }
}

impl From<bool> for AriaChecked {
    fn from(value: bool) -> Self {
        if value {
            Self::True
        } else {
            Self::False
        }
    }
}

impl From<ToggleCheckedState> for AriaChecked {
    fn from(value: ToggleCheckedState) -> Self {
        match value {
            ToggleCheckedState::Off => Self::False,
            ToggleCheckedState::On => Self::True,
            ToggleCheckedState::Indeterminate => Self::Mixed,
        }
    }
}

/// Returns the standard ARIA role for interactive buttons.
#[inline]
pub const fn role_button() -> &'static str {
    "button"
}

/// Returns the ARIA role for the listbox container element.
#[inline]
pub const fn role_listbox() -> &'static str {
    "listbox"
}

/// Returns the ARIA role for individual options within a listbox.
#[inline]
pub const fn role_option() -> &'static str {
    "option"
}

/// Returns the ARIA role used by checkbox controls.
#[inline]
pub const fn role_checkbox() -> &'static str {
    "checkbox"
}

/// Returns the ARIA role used by radio controls.
#[inline]
pub const fn role_radio() -> &'static str {
    "radio"
}

/// Returns the ARIA role used by switch controls.  `switch` was added in
/// ARIA 1.1 and maps closely to Material's design language.
#[inline]
pub const fn role_switch() -> &'static str {
    "switch"
}

/// Returns the ARIA role used by menu surfaces.
#[inline]
pub const fn role_menu() -> &'static str {
    "menu"
}

/// Returns the ARIA role used by interactive menu items.
#[inline]
pub const fn role_menuitem() -> &'static str {
    "menuitem"
}

/// Returns the ARIA role for tooltip surfaces.
#[inline]
pub const fn role_tooltip() -> &'static str {
    "tooltip"
}

/// Returns the ARIA role for tablist containers.
#[inline]
pub const fn role_tablist() -> &'static str {
    "tablist"
}

/// Returns the ARIA role for individual tabs within a tablist.
#[inline]
pub const fn role_tab() -> &'static str {
    "tab"
}

/// Returns the ARIA role for tab panels associated with tabs.
#[inline]
pub const fn role_tabpanel() -> &'static str {
    "tabpanel"
}

/// Returns the ARIA role for modal dialogs such as drawers.
#[inline]
pub const fn role_dialog() -> &'static str {
    "dialog"
}

/// Compute the `aria-pressed` attribute for toggleable buttons.
#[inline]
pub const fn aria_pressed(pressed: bool) -> (&'static str, &'static str) {
    ("aria-pressed", if pressed { "true" } else { "false" })
}

/// Compute the `aria-checked` attribute for selection controls.
#[inline]
pub const fn aria_checked(state: AriaChecked) -> (&'static str, &'static str) {
    ("aria-checked", state.as_str())
}

/// Internal utility converting boolean flags into optional attributes.
#[inline]
fn optional_bool_attribute(name: &'static str, active: bool) -> Option<(&'static str, String)> {
    active.then(|| (name, "true".to_string()))
}

/// Compute the `aria-disabled` attribute used across inputs.
///
/// The helper only emits metadata when the control is disabled so adapters can
/// avoid rendering noisy `aria-disabled="false"` tuples.  Consumers should
/// combine this with [`data_disabled`] so enterprise automation and styling
/// hooks receive the same declarative cues.
#[inline]
pub fn aria_disabled(disabled: bool) -> Option<(&'static str, String)> {
    optional_bool_attribute("aria-disabled", disabled)
}

/// Compute the `data-disabled` attribute paired with [`aria_disabled`].
///
/// Headless adapters use the attribute to expose a consistent styling/automation
/// hook across SSR and CSR renders without duplicating the disabled bookkeeping
/// logic.
#[inline]
pub fn data_disabled(disabled: bool) -> Option<(&'static str, String)> {
    optional_bool_attribute("data-disabled", disabled)
}

/// Push paired `aria-disabled`/`data-disabled` tuples into the provided buffer.
///
/// This helper centralises the boilerplate so state machines and adapters can
/// remain declarative — they simply supply the disabled flag and receive the
/// correct ARIA/data hooks when the control is inert.
#[inline]
pub fn extend_disabled_attributes(attrs: &mut Vec<(&'static str, String)>, disabled: bool) {
    if let Some(attr) = aria_disabled(disabled) {
        attrs.push(attr);
    }
    if let Some(attr) = data_disabled(disabled) {
        attrs.push(attr);
    }
}

/// Compute the `aria-expanded` attribute shared by disclosure widgets.
#[inline]
pub const fn aria_expanded(expanded: bool) -> (&'static str, &'static str) {
    ("aria-expanded", if expanded { "true" } else { "false" })
}

/// Compute the `aria-haspopup` attribute indicating the popup type.
#[inline]
pub const fn aria_haspopup(kind: &'static str) -> (&'static str, &'static str) {
    ("aria-haspopup", kind)
}

/// Compute the `aria-selected` attribute used by tabs and similar widgets.
#[inline]
pub const fn aria_selected(selected: bool) -> (&'static str, &'static str) {
    ("aria-selected", if selected { "true" } else { "false" })
}

/// Compute the `aria-controls` attribute linking tabs to their panels.
#[inline]
pub fn aria_controls(id: &str) -> (&'static str, &str) {
    ("aria-controls", id)
}

/// Compute the `aria-labelledby` attribute for elements referenced by labels.
#[inline]
pub fn aria_labelledby(id: &str) -> (&'static str, &str) {
    ("aria-labelledby", id)
}

/// Compute the `aria-describedby` attribute linking to additional context.
#[inline]
pub fn aria_describedby(id: &str) -> (&'static str, &str) {
    ("aria-describedby", id)
}

/// Compute the `aria-orientation` attribute for multi-directional widgets.
#[inline]
pub const fn aria_orientation(orientation: &'static str) -> (&'static str, &'static str) {
    ("aria-orientation", orientation)
}

/// Compute the `aria-modal` attribute for dialog style surfaces.
#[inline]
pub const fn aria_modal(modal: bool) -> (&'static str, &'static str) {
    ("aria-modal", if modal { "true" } else { "false" })
}

/// Compute the `aria-hidden` attribute that automation tools often assert.
#[inline]
pub const fn aria_hidden(hidden: bool) -> (&'static str, &'static str) {
    ("aria-hidden", if hidden { "true" } else { "false" })
}
