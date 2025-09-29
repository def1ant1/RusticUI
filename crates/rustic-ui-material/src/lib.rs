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

pub mod accordion;
#[cfg(feature = "feedback")]
pub mod alert;
pub mod app_bar;
pub mod avatar;
#[cfg(feature = "feedback")]
pub mod backdrop;
pub mod badge;
pub mod r#box;
pub mod button;
pub mod card;
pub mod checkbox;
pub mod chip;
#[cfg(feature = "progress")]
pub mod circular_progress;
pub mod container;
pub mod dialog;
pub mod divider;
pub mod drawer;
#[cfg(feature = "forms")]
pub mod form_control;
pub mod grid;
pub mod hidden;
pub mod image_list;
#[cfg(feature = "forms")]
pub mod input_adornment;
#[cfg(feature = "progress")]
pub mod linear_progress;
pub mod list;
pub mod macros;
pub mod menu;
pub mod paper;
pub mod radio;
mod render_helpers;
pub mod select;
mod selection_control;
#[cfg(feature = "progress")]
pub mod skeleton;
#[cfg(feature = "forms")]
pub mod slider;
pub mod snackbar;
pub mod stack;
mod style_helpers;
pub mod switch;
pub mod tab;
pub mod tab_panel;
pub mod table;
pub mod tabs;
pub mod text_field;
pub mod tooltip;
pub mod typography;

pub use rustic_ui_styled_engine::Theme;

pub use crate::r#box::{render_box, BoxAdapterProps, BoxRenderOutput};
pub use accordion::{
    render_accordion, AccordionAdapterProps, AccordionItemDescriptor, AccordionRenderOutput,
};
#[cfg(feature = "feedback")]
pub use alert::{render_alert, AlertAdapterProps, AlertRenderOutput};
pub use avatar::{render_avatar, AvatarRenderOutput};
#[cfg(feature = "feedback")]
pub use backdrop::{render_backdrop, BackdropAdapterProps, BackdropRenderOutput};
pub use badge::{render_badge, BadgeRenderOutput};
#[cfg(feature = "progress")]
pub use circular_progress::{
    render_circular_progress, CircularProgressAdapterProps, CircularProgressRenderOutput,
};
pub use container::{render_container, ContainerAdapterProps, ContainerRenderOutput};
pub use divider::{render_divider, DividerAdapterProps, DividerRenderOutput};
#[cfg(feature = "forms")]
pub use form_control::{render_form_control, FormControlAdapterProps, FormControlRenderOutput};
pub use grid::{render_grid, GridAdapterProps, GridRenderOutput};
pub use hidden::{render_hidden, HiddenAdapterProps, HiddenRenderOutput};
pub use image_list::{render_image_list, ImageListAdapterProps, ImageListRenderOutput};
#[cfg(feature = "forms")]
pub use input_adornment::{
    render_input_adornment, InputAdornmentAdapterProps, InputAdornmentRenderOutput,
};
#[cfg(feature = "progress")]
pub use linear_progress::{
    render_linear_progress, LinearProgressAdapterProps, LinearProgressRenderOutput,
};
pub use paper::{render_paper, PaperRenderOutput};
#[cfg(feature = "progress")]
pub use skeleton::{render_skeleton, SkeletonAdapterProps, SkeletonRenderOutput};
#[cfg(feature = "forms")]
pub use slider::{render_slider, SliderAdapterProps, SliderRenderOutput};
pub use stack::{render_stack, StackAdapterProps, StackRenderOutput};
pub use typography::{render_typography, TypographyRenderOutput};

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
