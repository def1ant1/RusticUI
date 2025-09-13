use yew::prelude::*;
use mui_styled_engine::use_theme;

use crate::material_props;

material_props!(CardProps {
    /// Content of the card.
    children: Children,
});

/// Simple container with themed border and padding.
#[function_component(Card)]
pub fn card(props: &CardProps) -> Html {
    let theme = use_theme();
    let style = format!(
        "border:1px solid {};padding:{}px;",
        theme.palette.primary,
        theme.spacing(2)
    );
    html! { <div style={style}>{ for props.children.iter() }</div> }
}
