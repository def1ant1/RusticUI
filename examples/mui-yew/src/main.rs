use yew::prelude::*;
use mui_system::Box;

/// Entry component showcasing a minimal Material UI setup in Yew.
/// The example favors static generation via `trunk build --release`
/// which yields a tiny, cacheable WebAssembly artifact.
#[function_component(App)]
fn app() -> Html {
    html! {
        <Box style="padding:1rem;">
            <p>{ "Hello from Yew + MUI!" }</p>
        </Box>
    }
}

fn main() {
    // Mount the application to the document body.
    yew::Renderer::<App>::new().render();
}
