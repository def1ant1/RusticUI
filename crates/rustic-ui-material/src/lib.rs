//! Material Design components built on top of [`rustic_ui_styled_engine`].
//!
//! The crate currently provides a small subset of widgets such as [`button`],
//! [`card`], [`dialog`], [`app_bar`], [`text_field`], [`snackbar`], [`checkbox`],
//! [`radio`], [`select`], [`menu`], [`list`], [`table`] and [`switch`]. Each component consumes the shared [`Theme`]
//! provided by `rustic_ui_styled_engine` so applications have a single source of
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

/// Confirms that the crate links to `rustic_ui_styled_engine` and compiles.
pub fn placeholder() {
    rustic_ui_styled_engine::placeholder();
}

#[cfg(feature = "compat-mui")]
#[doc = "Deprecated compatibility shim exposing the crate under the legacy `mui_material` name.\n\
Activate the `compat-mui` feature only while migrating to `rustic_ui_material`.\n\
The alias will be purged in an upcoming pre-1.0 release."]
#[deprecated(
    since = "0.1.0",
    note = "Migrate to `rustic_ui_material`. The `mui_material` compatibility alias will be removed."
)]
pub use crate as mui_material;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn placeholder_works() {
        placeholder();
    }
}
