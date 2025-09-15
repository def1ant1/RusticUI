//! Material flavored button that builds on the headless [`ButtonState`].
//!
//! Rendering logic is centralized so the button looks and behaves the same
//! across all supported frameworks. The shared renderer uses
//! [`css_with_theme!`](mui_styled_engine::css_with_theme) to derive visual
//! styles from the active [`Theme`](mui_styled_engine::Theme) instead of hard
//! coded strings. The [`style_helpers::themed_class`](crate::style_helpers::themed_class)
//! function converts the generated [`Style`](mui_styled_engine::Style) into a
//! scoped class, documenting how the styled engine registers CSS. Accessibility
//! attributes returned by [`ButtonState`] are merged using the
//! [`mui_utils::collect_attributes`] helper which keeps ARIA wiring consistent
//! across all adapters so assistive technologies accurately describe the
//! control.
//!
//! This module intentionally contains no framework specific code.  Instead it
//! exposes lightweight adapters which convert the shared state into view code
//! for each supported front-end framework.  This design minimizes repeated
//! business logic and keeps rendering concerns decoupled from behavior while
//! encouraging adapters to reuse the shared helpers documented above.

use mui_headless::button::ButtonState;
use mui_styled_engine::css_with_theme;
use mui_utils::{attributes_to_html, collect_attributes};

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
/// macro pulls colors and spacing from the active [`Theme`] so the button's
/// appearance automatically tracks global design tokens. The returned class is
/// attached to the `<button>` element and the ARIA attributes emitted from
/// [`ButtonState`] are merged into the final tag to enhance accessibility.
fn render_html(props: &ButtonProps, state: &ButtonState) -> String {
    // Build a themed style block. We intentionally keep the CSS minimal since
    // this example returns plain HTML rather than framework specific nodes.
    let class = crate::style_helpers::themed_class(css_with_theme!(
        r#"
        background: ${bg};
        color: #fff;
        padding: 8px 16px;
        border: none;
    "#,
        // Use the primary palette color so the button automatically adapts to
        // custom themes without manual tweaking.
        bg = theme.palette.primary.clone()
    ));

    // Compose the HTML attributes with the reusable helpers so SSR adapters
    // mirror the DOM exposed by WebAssembly frameworks without bespoke code.
    let attrs = collect_attributes(Some(class), state.aria_attributes());
    let attr_string = attributes_to_html(&attrs);

    // Final HTML representation. Individual adapters simply forward to this
    // function keeping rendering logic DRY and easy to evolve.
    format!("<button {}>{}</button>", attr_string, props.label)
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
