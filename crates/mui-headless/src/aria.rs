//! Helpers for generating ARIA attributes.
//! Keeping these utilities centralized ensures accessibility semantics
//! stay consistent across framework adapters.

/// Returns the standard ARIA role for interactive buttons.
#[inline]
pub const fn role_button() -> &'static str {
    "button"
}

/// Compute the `aria-pressed` attribute for toggleable buttons.
#[inline]
pub const fn aria_pressed(pressed: bool) -> (&'static str, &'static str) {
    ("aria-pressed", if pressed { "true" } else { "false" })
}
