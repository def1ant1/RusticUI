use crate::theme::Theme;

#[cfg(feature = "yew")]
mod yew_impl {
    use super::*;
    use yew::prelude::*;

    /// Provides the current [`Theme`] to descendant components via Yew's context system.
    #[derive(Properties, PartialEq)]
    pub struct ThemeProviderProps {
        pub theme: Theme,
        #[prop_or_default]
        pub children: Children,
    }

    #[function_component(ThemeProvider)]
    pub fn theme_provider(props: &ThemeProviderProps) -> Html {
        html! {
            <ContextProvider<Theme> context={props.theme.clone()}>
                { for props.children.iter() }
            </ContextProvider<Theme>>
        }
    }

    /// Retrieves the current theme from context. Components can call this helper
    /// instead of dealing with `use_context` directly which keeps the call sites concise.
    #[hook]
    pub fn use_theme() -> Theme {
        use_context::<Theme>().unwrap_or_default()
    }
}

#[cfg(feature = "yew")]
pub use yew_impl::{ThemeProvider, ThemeProviderProps, use_theme};

#[cfg(feature = "leptos")]
mod leptos_impl {
    use super::*;
    use leptos::*;

    /// Leptos variant of the [`ThemeProvider`].
    #[component]
    pub fn ThemeProvider(theme: Theme, children: Children) -> impl IntoView {
        provide_context(theme);
        view! { {children()} }
    }

    /// Access the current [`Theme`] from context.
    pub fn use_theme() -> Theme {
        use_context::<Theme>().unwrap_or_default()
    }
}

#[cfg(feature = "leptos")]
pub use leptos_impl::{ThemeProvider, use_theme};
