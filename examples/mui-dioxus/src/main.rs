use dioxus::prelude::*;
use rustic_ui_system::use_theme;

/// Basic Dioxus component demonstrating integration with `mui-system`.
fn App(cx: Scope) -> Element {
    let theme = use_theme();
    let pad = theme.spacing(2);
    cx.render(rsx! {
        div {
            style: "padding: {pad}px;",
            "Hello from Dioxus + MUI!"
        }
    })
}

#[cfg(feature = "csr")]
fn main() {
    // Render directly in the browser. Tools like `dx serve` provide live reload.
    dioxus_web::launch(App);
}

#[cfg(feature = "ssr")]
fn main() {
    // Server-side rendering entry that outputs a static HTML string.
    // Real applications would embed this within a web framework.
    println!("{}", dioxus_ssr::render_lazy(App));
}
