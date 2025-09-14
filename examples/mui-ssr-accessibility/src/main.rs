use yew::prelude::*;
use yew::ServerRenderer;
use mui_material::{AppBar};
use mui_styled_engine::{StyledEngineProvider, Theme};

/// Demonstrates server-side rendering (SSR) with accessibility best
/// practices. The example renders an [`AppBar`] with an `aria-label` so
/// assistive technologies can describe its purpose to users.
#[function_component(App)]
fn app() -> Html {
    html! {
        <StyledEngineProvider theme={Theme::default()}>
            <AppBar title="SSR" aria_label="primary navigation" />
        </StyledEngineProvider>
    }
}

#[tokio::main]
async fn main() {
    // Render the application to an HTML string on the server.  The output can
    // be directly embedded into an HTTP response for fast first paint.
    let rendered = ServerRenderer::<App>::new().render().await;
    println!("{}", rendered);
}
