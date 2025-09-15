use leptos::*;
use mui_system::{theme::Theme, ThemeProvider, Box};

/// Root application component demonstrating a minimal theming setup
/// using `mui-system` primitives within Leptos.
#[component]
fn App() -> impl IntoView {
    let theme = Theme::default();
    view! {
        <ThemeProvider theme>
            // `Box` mirrors the Material UI container component and accepts
            // familiar system properties. Inline styles keep the example short.
            <Box sx="padding:1rem;">
                <p>{ "Hello from Leptos + MUI!" }</p>
            </Box>
        </ThemeProvider>
    }
}

#[cfg(feature = "csr")]
fn main() {
    // Client-side rendering mounts the app directly to `document.body`.
    mount_to_body(|| view! { <App/> });
}

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    // Server-side rendering path used during pre-rendering. In a real service
    // the resulting HTML would be returned from a framework like Axum.
    use leptos::ssr::render_to_string;
    println!("{}", render_to_string(|| view! { <App/> }));
}
