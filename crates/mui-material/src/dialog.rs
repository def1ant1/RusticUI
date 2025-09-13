use yew::prelude::*;
use mui_styled_engine::use_theme;

use crate::material_props;

material_props!(DialogProps {
    /// Whether the dialog is shown.
    open: bool,
    /// Dialog contents.
    children: Children,
});

/// Minimal dialog implementation that toggles visibility.
#[function_component(Dialog)]
pub fn dialog(props: &DialogProps) -> Html {
    if !props.open {
        return Html::default();
    }
    let theme = use_theme();
    let style = format!(
        "border:2px solid {};padding:{}px;",
        theme.palette.secondary,
        theme.spacing(3)
    );
    html! { <div style={style}>{ for props.children.iter() }</div> }
}
