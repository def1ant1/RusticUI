#[cfg(any(feature = "yew", feature = "dioxus", feature = "sycamore"))]
use mui_styled_engine::{use_theme, Theme};

#[cfg(feature = "yew")]
use yew::prelude::*;

use crate::material_props;

#[cfg(any(feature = "yew", feature = "dioxus", feature = "sycamore"))]
fn resolve_style(theme: &Theme) -> String {
    format!(
        "border:2px solid {};padding:{}px;",
        theme.palette.secondary,
        theme.spacing(3)
    )
}

#[cfg(feature = "yew")]
material_props!(DialogProps {
    /// Whether the dialog is shown.
    open: bool,
    /// Dialog contents.
    children: Children,
});

#[cfg(feature = "yew")]
mod yew_impl {
    use super::*;

    /// Minimal dialog implementation that toggles visibility.
    #[function_component(Dialog)]
    pub fn dialog(props: &DialogProps) -> Html {
        if !props.open {
            return Html::default();
        }
        let theme = use_theme();
        let style = resolve_style(&theme);
        html! { <div style={style}>{ for props.children.iter() }</div> }
    }
}

#[cfg(feature = "yew")]
pub use yew_impl::{Dialog, DialogProps};

#[cfg(feature = "dioxus")]
mod dioxus_impl {
    use super::*;

    #[derive(Default, Clone, PartialEq)]
    pub struct DialogProps {
        pub open: bool,
    }

    pub fn Dialog(props: DialogProps) {
        if !props.open {
            return;
        }
        let theme = use_theme();
        let _ = resolve_style(&theme);
    }
}

#[cfg(feature = "dioxus")]
pub use dioxus_impl::{Dialog, DialogProps};

#[cfg(feature = "sycamore")]
mod sycamore_impl {
    use super::*;

    #[derive(Default, Clone, PartialEq)]
    pub struct DialogProps {
        pub open: bool,
    }

    pub fn Dialog(props: DialogProps) {
        if !props.open {
            return;
        }
        let theme = use_theme();
        let _ = resolve_style(&theme);
    }
}

#[cfg(feature = "sycamore")]
pub use sycamore_impl::{Dialog, DialogProps};
