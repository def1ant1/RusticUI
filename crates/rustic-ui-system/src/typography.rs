/// Typography variants represented as semantic HTML tags.
#[derive(Clone, PartialEq)]
pub enum TypographyVariant {
    H1,
    H2,
    Body1,
}

#[cfg(feature = "yew")]
mod yew_impl {
    use super::*;
    use yew::prelude::*;

    /// Properties for the [`Typography`] component.
    #[derive(Properties, PartialEq)]
    pub struct TypographyProps {
        /// Which typography style to apply. Defaults to [`TypographyVariant::Body1`].
        #[prop_or_default]
        pub variant: Option<TypographyVariant>,
        /// Inline style string built with the `sx` helpers.
        #[prop_or_default]
        pub sx: String,
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
        if !props.sx.is_empty() {
            node.add_attribute("style", props.sx.clone());
        }
        node.add_children(props.children.iter());
        Html::VTag(Box::new(node))
    }
}

// Maintain the simple `Typography` import for single-framework builds while
// still supporting the explicit aliases expected by dual-adapter CI runs.
#[cfg(all(feature = "yew", not(feature = "leptos")))]
pub use yew_impl::{Typography, TypographyProps};

// Surface deterministic aliases so downstream crates can opt into the Yew
// adapter without relying on extra cfg annotations.
#[cfg(feature = "yew")]
pub use yew_impl::{Typography as TypographyYew, TypographyProps as TypographyPropsYew};

#[cfg(feature = "leptos")]
mod leptos_impl {
    use super::*;
    use leptos::*;

    /// Leptos variant of [`Typography`].
    #[component]
    pub fn Typography(
        #[prop(optional)] variant: Option<TypographyVariant>,
        #[prop(optional, into)] sx: String,
        children: Children,
    ) -> impl IntoView {
        let node: View = match variant {
            Some(TypographyVariant::H1) => {
                view! { <h1 style=sx.clone()>{children()}</h1> }.into_view()
            }
            Some(TypographyVariant::H2) => {
                view! { <h2 style=sx.clone()>{children()}</h2> }.into_view()
            }
            _ => view! { <p style=sx>{children()}</p> }.into_view(),
        };
        node
    }
}

// Provide the backwards compatible export for Leptos-only builds while adding a
// namespaced alias usable in mixed feature combinations.
#[cfg(all(feature = "leptos", not(feature = "yew")))]
pub use leptos_impl::Typography;

#[cfg(feature = "leptos")]
pub use leptos_impl::Typography as TypographyLeptos;
