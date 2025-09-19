//! Helpers for generating ARIA attributes.
//! Keeping these utilities centralized ensures accessibility semantics
//! stay consistent across framework adapters.

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

/// Compute the `aria-pressed` attribute for toggleable buttons.
#[inline]
pub const fn aria_pressed(pressed: bool) -> (&'static str, &'static str) {
    ("aria-pressed", if pressed { "true" } else { "false" })
}

/// Compute the `aria-checked` attribute for selection controls.
#[inline]
pub const fn aria_checked(checked: bool) -> (&'static str, &'static str) {
    ("aria-checked", if checked { "true" } else { "false" })
}

/// Compute the `aria-disabled` attribute used across inputs.
#[inline]
pub const fn aria_disabled(disabled: bool) -> (&'static str, &'static str) {
    ("aria-disabled", if disabled { "true" } else { "false" })
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
