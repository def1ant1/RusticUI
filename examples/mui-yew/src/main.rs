use yew::prelude::*;
use rustic_ui_system::{theme::Theme, ThemeProvider, Box};

/// Yew application wired up with MUI theming and an SSR entry point.
#[function_component(App)]
fn app() -> Html {
    let theme = Theme::default();
    html! {
        <ThemeProvider theme={theme}>
            <Box sx="padding:1rem;">
                <p>{ "Hello from Yew + MUI!" }</p>
            </Box>
        </ThemeProvider>
    }
}

#[cfg(feature = "csr")]
fn main() {
    // Hydrate existing SSR markup when present, otherwise render from scratch.
    yew::Renderer::<App>::new().hydrate();
}

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    // Produce HTML on the server. Downstream frameworks can embed this output
    // in an HTTP response and serve the same bundle for client hydration.
    use yew::ServerRenderer;
    let html = ServerRenderer::<App>::new().render().await;
    println!("{html}");
}
