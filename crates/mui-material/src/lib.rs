//! Material Design components built on top of [`mui-styled-engine`].
//!
//! The crate currently provides a small subset of widgets such as [`button`],
//! [`card`], [`dialog`], [`app_bar`], [`text_field`], [`snackbar`], [`checkbox`],
//! [`radio`], [`select`], [`menu`] and [`switch`]. Each component consumes the shared [`Theme`]
//! provided by `mui-styled-engine` so applications have a single source of
//! truth for styling.
//!
//! # Example
//! ```rust,ignore
//! use mui_material::{Button, ButtonProps};
//! use mui_styled_engine::{ThemeProvider, Theme};
//! use yew::prelude::*;
//!
//! #[function_component(App)]
//! fn app() -> Html {
//!     html! {
//!         <ThemeProvider theme={Theme::default()}>
//!             <Button label="Click me" />
//!         </ThemeProvider>
//!     }
//! }
//! ```

pub mod app_bar;
pub mod button;
pub mod card;
pub mod checkbox;
pub mod dialog;
pub mod macros;
pub mod menu;
pub mod radio;
pub mod select;
mod selection_control;
pub mod snackbar;
mod style_helpers;
pub mod switch;
pub mod text_field;

pub use mui_styled_engine::Theme;

/// Confirms that the crate links to `mui-styled-engine` and compiles.
pub fn placeholder() {
    mui_styled_engine::placeholder();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn placeholder_works() {
        placeholder();
    }
}
