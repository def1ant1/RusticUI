use mui_styled_engine::{css_with_theme, use_theme};
use yew::prelude::*;

pub use crate::macros::{Color as SnackbarColor, Size as SnackbarSize, Variant as SnackbarVariant};

crate::material_component_props!(SnackbarProps {
    /// Message presented to the user.
    message: String,
});

/// Transient feedback component that briefly notifies the user about an
/// operation.  The component respects the active [`Theme`] and exposes the
/// usual MUI properties for color, variant and size.
#[function_component(Snackbar)]
pub fn snackbar(props: &SnackbarProps) -> Html {
    let theme = use_theme();
    let bg = match props.color {
        SnackbarColor::Primary => theme.palette.primary.clone(),
        SnackbarColor::Secondary => theme.palette.secondary.clone(),
    };
    let padding = match props.size {
        SnackbarSize::Small => "4px 8px",
        SnackbarSize::Medium => "8px 16px",
        SnackbarSize::Large => "16px 24px",
    };
    let border = match props.variant {
        SnackbarVariant::Outlined => format!("1px solid {}", bg),
        _ => String::from("none"),
    };
    let style = css_with_theme!(
        theme,
        r#"
        background: ${bg};
        color: #fff;
       padding: ${padding};
        border: ${border};
    "#,
        bg = bg,
        padding = padding,
        border = border
    );
    let class = style.get_class_name().to_string();

    html! {
        <div class={class} role="status">{ &props.message }</div>
    }
}
