//! Material Design components built on top of [`mui-styled-engine`].
//!
//! The crate currently provides a small subset of widgets such as [`button`],
//! [`card`], [`dialog`], [`app_bar`], [`text_field`], [`snackbar`], [`checkbox`],
//! [`radio`], [`select`], [`menu`], [`list`], [`table`] and [`switch`]. Each component consumes the shared [`Theme`]
//! provided by `mui-styled-engine` so applications have a single source of
//! truth for styling.
//!
//! # Example
//! ```rust,ignore
//! use rustic_ui_material::{Button, ButtonProps};
//! use rustic_ui_styled_engine::{ThemeProvider, Theme};
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
pub mod chip;
pub mod dialog;
pub mod drawer;
pub mod list;
pub mod macros;
pub mod menu;
pub mod radio;
mod render_helpers;
pub mod select;
mod selection_control;
pub mod snackbar;
mod style_helpers;
pub mod switch;
pub mod tab;
pub mod tab_panel;
pub mod table;
pub mod tabs;
pub mod text_field;
pub mod tooltip;

pub use rustic_ui_styled_engine::Theme;

/// Confirms that the crate links to `mui-styled-engine` and compiles.
pub fn placeholder() {
    rustic_ui_styled_engine::placeholder();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn placeholder_works() {
        placeholder();
    }
}
