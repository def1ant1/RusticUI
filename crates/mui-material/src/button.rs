//! Material flavored button that builds on the headless [`ButtonState`].
//!
//! This module intentionally contains no framework specific code.  Instead it
//! exposes lightweight adapters which convert the shared state into view code
//! for each supported front-end framework.  This design minimizes repeated
//! business logic and keeps rendering concerns decoupled from behavior.

use mui_headless::button::ButtonState;

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

// ---------------------------------------------------------------------------
// Adapter implementations
// ---------------------------------------------------------------------------

/// Adapter targeting the [`yew`] framework.
pub mod yew {
    use super::*;

    /// Render the button into a plain HTML string.
    ///
    /// In a real application this would construct a `yew::Html` node, however
    /// for testing and portability we simply return a string representation.
    pub fn render(props: &ButtonProps, state: &ButtonState) -> String {
        let attrs = state.aria_attributes();
        format!(
            "<button role=\"{}\" {}>{}</button>",
            attrs[0].1,
            format!("{}=\"{}\"", attrs[1].0, attrs[1].1),
            props.label
        )
    }
}

/// Adapter targeting the [`leptos`] framework.
pub mod leptos {
    use super::*;

    pub fn render(props: &ButtonProps, state: &ButtonState) -> String {
        let attrs = state.aria_attributes();
        format!(
            "<button role=\"{}\" {}>{}</button>",
            attrs[0].1,
            format!("{}=\"{}\"", attrs[1].0, attrs[1].1),
            props.label
        )
    }
}

/// Adapter targeting the [`dioxus`] framework.
pub mod dioxus {
    use super::*;

    pub fn render(props: &ButtonProps, state: &ButtonState) -> String {
        let attrs = state.aria_attributes();
        format!(
            "<button role=\"{}\" {}>{}</button>",
            attrs[0].1,
            format!("{}=\"{}\"", attrs[1].0, attrs[1].1),
            props.label
        )
    }
}

/// Adapter targeting the [`sycamore`] framework.
pub mod sycamore {
    use super::*;

    pub fn render(props: &ButtonProps, state: &ButtonState) -> String {
        let attrs = state.aria_attributes();
        format!(
            "<button role=\"{}\" {}>{}</button>",
            attrs[0].1,
            format!("{}=\"{}\"", attrs[1].0, attrs[1].1),
            props.label
        )
    }
}
