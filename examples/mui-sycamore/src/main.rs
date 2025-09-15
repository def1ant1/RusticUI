use sycamore::prelude::*;
use mui_system::use_theme;

/// Demonstrates obtaining theme values within a Sycamore view.
#[component]
fn App<G: Html>(cx: Scope) -> View<G> {
    let theme = use_theme();
    view! { cx,
        div(style=format!("padding:{}px;", theme.spacing(2))) {
            "Hello from Sycamore + MUI!"
        }
    }
}

#[cfg(feature = "csr")]
fn main() {
    // Client-side rendering that mounts the app into the DOM.
    sycamore::render(|cx| view! { cx, App {} });
}

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    // Server-side rendering entry point returning a static HTML string.
    println!("{}", sycamore::render_to_string(|cx| view! { cx, App {} }));
}
