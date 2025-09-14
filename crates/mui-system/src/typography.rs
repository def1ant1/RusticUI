#![cfg(feature = "yew")]
use crate::style_props;
use yew::prelude::*;

/// Typography variants represented as semantic HTML tags.
#[derive(Clone, PartialEq)]
pub enum TypographyVariant {
    H1,
    H2,
    Body1,
}

/// Properties for the [`Typography`] component.
#[derive(Properties, PartialEq)]
pub struct TypographyProps {
    /// Which typography style to apply. Defaults to [`TypographyVariant::Body1`].
    #[prop_or_default]
    pub variant: Option<TypographyVariant>,
    /// Inline style string built with the [`style_props!`] macro.
    #[prop_or_default]
    pub style: String,
    /// Text or elements to render.
    #[prop_or_default]
    pub children: Children,
}

/// Displays text with semantic HTML tags and optional styling.
#[function_component(Typography)]
pub fn typography(props: &TypographyProps) -> Html {
    let tag = match props.variant {
        Some(TypographyVariant::H1) => "h1",
        Some(TypographyVariant::H2) => "h2",
        _ => "p",
    };
    let mut node = yew::virtual_dom::VTag::new(tag);
    if !props.style.is_empty() {
        node.add_attribute("style", props.style.clone());
    }
    node.add_children(props.children.iter());
    Html::VTag(node)
}
