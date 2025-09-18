//! Material flavored button that builds on the headless [`ButtonState`].
//!
//! Rendering logic is centralized so the button looks and behaves the same
//! across all supported frameworks. The shared renderer uses
//! [`css_with_theme!`](mui_styled_engine::css_with_theme) to derive visual
//! styles from the active [`Theme`](mui_styled_engine::Theme) instead of hard
//! coded strings. The [`style_helpers::themed_class`](crate::style_helpers::themed_class)
//! and [`style_helpers::themed_attributes_html`](crate::style_helpers::themed_attributes_html)
//! helpers convert the generated [`Style`](mui_styled_engine::Style) into scoped
//! classes and attribute strings so SSR adapters do not need to hand-roll
//! concatenation logic. Accessibility attributes returned by [`ButtonState`]
//! travel through the same helper which keeps ARIA wiring consistent across all
//! adapters and ensures assistive technologies accurately describe the control.
//!
//! This module intentionally contains no framework specific code.  Instead it
//! exposes lightweight adapters which convert the shared state into view code
//! for each supported front-end framework.  This design minimizes repeated
//! business logic and keeps rendering concerns decoupled from behavior while
//! encouraging adapters to reuse the shared helpers documented above.

use mui_headless::button::ButtonState;
use mui_styled_engine::{css_with_theme, Style};

/// Shared properties accepted by all adapter implementations.
#[derive(Clone, Debug)]
pub struct ButtonProps {
    /// Text rendered inside the button.
    pub label: String,
}

impl ButtonProps {
    /// Convenience constructor used by examples and tests.
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
        }
    }
}

/// Shared rendering routine used by all adapters.
///
/// The function generates a scoped CSS class using [`css_with_theme!`]. The
/// macro pulls spacing, typography and palette tokens from the active
/// [`Theme`](mui_styled_engine::Theme) so the button's appearance automatically
/// tracks global design decisions. The returned class is attached to the
/// `<button>` element and the ARIA attributes emitted from [`ButtonState`] are
/// merged into the final tag to enhance accessibility for assistive
/// technologies.
fn render_html(props: &ButtonProps, state: &ButtonState) -> String {
    // Build an attribute string that includes the themed class and the latest
    // ARIA metadata from the state machine. The shared helper keeps adapters
    // extremely small while guaranteeing they all emit the same markup for SSR
    // and hydration scenarios.
    let attr_string = crate::style_helpers::themed_attributes_html(
        themed_button_style(),
        state.aria_attributes(),
    );

    // Final HTML representation. Individual adapters simply forward to this
    // function keeping rendering logic DRY and easy to evolve.
    format!("<button {}>{}</button>", attr_string, props.label)
}

/// Builds the [`Style`] powering the Material flavored button.
///
/// [`css_with_theme!`] exposes a `theme` binding so the macro can pull values
/// directly from the design tokens. We lean on that hook to derive:
///
/// * Background colors from the palette so primary/secondary overrides are
///   respected automatically.
/// * Typography settings (font family, weight and letter spacing) from the
///   theme's button ramp, ensuring text matches the Material spec without
///   sprinkling literal values around the codebase.
/// * Padding, radius and focus outlines from the shared spacing and Joy token
///   helpers which keeps spatial relationships consistent across components.
fn themed_button_style() -> Style {
    css_with_theme!(
        r#"
        background: ${background};
        color: ${text};
        padding: ${padding_y} ${padding_x};
        border: none;
        border-radius: ${radius};
        font-family: ${font_family};
        font-weight: ${font_weight};
        letter-spacing: ${letter_spacing};
        cursor: pointer;
        transition: background-color 160ms ease-in-out, box-shadow 160ms ease-in-out;

        &:hover {
            background: ${hover_background};
        }

        &:focus-visible {
            outline: ${focus_outline_width} solid ${focus_outline_color};
            outline-offset: 2px;
        }
    "#,
        background = theme.palette.primary.clone(),
        hover_background = theme.palette.secondary.clone(),
        text = theme.palette.background_paper.clone(),
        padding_y = format!("{}px", theme.spacing(1)),
        padding_x = format!("{}px", theme.spacing(2)),
        radius = format!("{}px", theme.joy.radius),
        font_family = theme.typography.font_family.clone(),
        font_weight = theme.typography.font_weight_medium.to_string(),
        letter_spacing = format!("{:.3}rem", theme.typography.button_letter_spacing),
        focus_outline_width = format!("{}px", theme.joy.focus_thickness),
        focus_outline_color = theme.palette.text_primary.clone()
    )
}

/// Collects the generated class alongside ARIA attributes emitted by
/// [`ButtonState`].
///
/// [`ButtonState::aria_attributes`] returns a small array containing `role`
/// metadata and the current pressed flag. Merging that array with the themed
/// class inside a single helper guarantees every framework adapter attaches the
/// same accessibility affordances without duplicating knowledge about how the
/// state machine works.
#[cfg_attr(not(test), allow(dead_code))]
fn themed_button_attributes(state: &ButtonState) -> Vec<(String, String)> {
    crate::style_helpers::themed_attributes(themed_button_style(), state.aria_attributes())
}

// ---------------------------------------------------------------------------
// Adapter implementations
// ---------------------------------------------------------------------------

/// Adapter targeting the [`yew`] framework.
pub mod yew {
    use super::*;

    /// Render the button into a plain HTML string using a theme aware style.
    ///
    /// The actual HTML generation is delegated to [`super::render_html`] so all
    /// frameworks share the same behavior.
    pub fn render(props: &ButtonProps, state: &ButtonState) -> String {
        super::render_html(props, state)
    }
}

/// Adapter targeting the [`leptos`] framework.
pub mod leptos {
    use super::*;

    /// Render the button into a plain HTML string using a theme aware style.
    /// This mirrors the [`yew`] adapter and keeps logic centralized.
    pub fn render(props: &ButtonProps, state: &ButtonState) -> String {
        super::render_html(props, state)
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn themed_attributes_include_class_and_aria_metadata() {
        let state = ButtonState::new(false, None);
        let attrs = themed_button_attributes(&state);

        // Ensure the generated attributes include the scoped class followed by
        // the ARIA metadata emitted by the state machine. This protects the
        // accessibility contract shared across every adapter.
        assert_eq!(attrs[0].0, "class");
        assert!(attrs.iter().any(|(key, _)| key == "role"));
        assert!(attrs.iter().any(|(key, _)| key == "aria-pressed"));
    }

    #[test]
    fn render_html_includes_theme_class_and_accessibility_attributes() {
        let props = ButtonProps::new("Submit");
        let state = ButtonState::new(false, None);
        let html = render_html(&props, &state);

        // The rendered HTML should surface the scoped class name so the CSS
        // emitted by `css_with_theme!` attaches to the element.
        assert!(html.contains("class=\""));
        // ARIA metadata describes the button semantics for assistive
        // technologies. Confirm the merged attributes survive formatting.
        assert!(html.contains("role=\"button\""));
        assert!(html.contains("aria-pressed=\"false\""));
        // The label should be rendered verbatim inside the button element.
        assert!(html.contains(">Submit<"));
    }
}

/// Adapter targeting the [`dioxus`] framework.
pub mod dioxus {
    use super::*;

    /// Render the button into a plain HTML string using a theme aware style.
    /// Delegates to [`super::render_html`] to avoid duplication.
    pub fn render(props: &ButtonProps, state: &ButtonState) -> String {
        super::render_html(props, state)
    }
}

/// Adapter targeting the [`sycamore`] framework.
pub mod sycamore {
    use super::*;

    /// Render the button into a plain HTML string using a theme aware style.
    /// Delegates to [`super::render_html`] just like the other adapters.
    pub fn render(props: &ButtonProps, state: &ButtonState) -> String {
        super::render_html(props, state)
    }
}
