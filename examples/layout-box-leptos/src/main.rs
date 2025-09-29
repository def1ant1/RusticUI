#[cfg(all(feature = "csr", target_arch = "wasm32"))]
fn main() {
    layout_box_leptos::hydrate();
}

#[cfg(all(feature = "csr", not(feature = "ssr"), not(target_arch = "wasm32")))]
fn main() {}

#[cfg(all(feature = "ssr", not(target_arch = "wasm32")))]
fn main() {
    let document = layout_box_leptos::render_document();
    println!("{document}");
}
