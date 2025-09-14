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
pub use yew_impl::{use_theme, ThemeProvider, ThemeProviderProps};

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
pub use leptos_impl::{use_theme, ThemeProvider};

#[cfg(feature = "dioxus")]
mod dioxus_impl {
    use super::*;

    /// Placeholder theme hook for Dioxus backends. Currently just returns
    /// [`Theme::default`] allowing compile-time integration tests to exercise
    /// the styling macros without pulling in a full Dioxus dependency.
    pub fn use_theme() -> Theme {
        Theme::default()
    }
}

#[cfg(feature = "dioxus")]
pub use dioxus_impl::use_theme;

#[cfg(feature = "sycamore")]
mod sycamore_impl {
    use super::*;

    /// Placeholder theme hook for Sycamore backends mirroring the behaviour of
    /// other frameworks. This keeps the API surface consistent while avoiding
    /// heavyweight dependencies in tests.
    pub fn use_theme() -> Theme {
        Theme::default()
    }
}

#[cfg(feature = "sycamore")]
pub use sycamore_impl::use_theme;

// Fallback implementation used when no front-end integration feature is enabled.
#[cfg(not(any(feature = "yew", feature = "leptos", feature = "dioxus", feature = "sycamore")))]
pub fn use_theme() -> Theme {
    Theme::default()
}
