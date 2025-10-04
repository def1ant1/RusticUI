#[cfg(not(target_arch = "wasm32"))]
fn main() {
    for (index, fragment) in selection_controls_yew::ssr_snapshots().iter().enumerate() {
        println!("SSR fragment #{index}:\n{fragment}\n");
    }
}

#[cfg(target_arch = "wasm32")]
fn main() {
    yew::Renderer::<selection_controls_yew::SelectionControlsDemo>::new().render();
}
