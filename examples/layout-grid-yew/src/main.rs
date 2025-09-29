#[cfg(all(feature = "csr", target_arch = "wasm32"))]
fn main() {
    layout_grid_yew::hydrate();
}

#[cfg(all(feature = "csr", not(feature = "ssr"), not(target_arch = "wasm32")))]
fn main() {}

#[cfg(all(feature = "ssr", not(target_arch = "wasm32")))]
#[tokio::main]
async fn main() {
    let document = layout_grid_yew::render_document().await;
    println!("{document}");
}
