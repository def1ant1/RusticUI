//! Scoped class utilities shared by the framework adapters.
//!
//! The system components used to concatenate inline style strings and bolt them
//! directly onto DOM nodes. While convenient, that approach prevented us from
//! reusing Material UI's styling engine which expects CSS rules to be registered
//! globally and referenced via deterministic class names.  This module provides
//! a small wrapper around the [`Style`] handle returned by
//! [`css_with_theme!`](rustic_ui_styled_engine::css_with_theme) so adapters can convert
//! raw CSS declarations into a class while keeping the underlying style alive
//! for as long as the component is mounted.
//!
//! By funnelling the registration logic through a dedicated type we avoid
//! copy‑pasting the same macro invocation across every adapter, document how the
//! theme is retrieved and guarantee that future integrations (for example
//! Sycamore or Dioxus) can lean on the exact same lifecycle.  The struct also
//! exposes accessors for the generated class name and stylesheet which keeps our
//! integration tests and documentation snippets straightforward.

use rustic_ui_styled_engine::Style;

/// Wrapper storing the scoped class produced by the styled engine alongside the
/// [`Style`] handle that keeps the rules mounted in the document.
#[derive(Clone, Debug)]
pub struct ScopedClass {
    class: String,
    #[allow(dead_code)]
    style: Style,
}

impl ScopedClass {
    /// Wraps an existing [`Style`] handle returned by `css_with_theme!` and
    /// captures the generated class name.
    pub fn from_style(style: Style) -> Self {
        let class = style.get_class_name().to_string();
        Self { class, style }
    }

    /// Registers an arbitrary CSS declaration string with the styled engine.
    ///
    /// This variant is used by system components that assemble large dynamic
    /// style strings (for example when flattening responsive props). The
    /// resulting class is functionally identical to one produced by
    /// `css_with_theme!`; it simply bypasses the macro so runtime generated CSS
    /// can be recorded without hitting the macro parser's compile-time
    /// restrictions.
    pub fn from_declarations(declarations: String) -> Self {
        let style = Style::new(declarations).expect("valid css");
        Self::from_style(style)
    }

    /// Returns the generated class name so adapters can attach it to DOM nodes.
    pub fn class(&self) -> &str {
        &self.class
    }

    /// Provides read-only access to the underlying [`Style`] handle. Keeping
    /// the handle alive is important because dropping it immediately would
    /// unmount the CSS from the registry and strip visual styling from the
    /// component.
    #[allow(dead_code)]
    pub fn style(&self) -> &Style {
        &self.style
    }

    /// Exposes the generated stylesheet for integration tests and documentation
    /// examples. We return a `&str` to avoid needless allocations when callers
    /// simply want to assert on fragments of the CSS.
    #[allow(dead_code)]
    pub fn stylesheet(&self) -> &str {
        self.style.get_style_str()
    }
}
