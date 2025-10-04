//! Binary entrypoint bridging native previews and WebAssembly hydration.

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    selection_controls_sycamore::render_cli_preview();
}

#[cfg(target_arch = "wasm32")]
fn main() {}

#[cfg(all(target_arch = "wasm32", feature = "csr"))]
#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn start() {
    selection_controls_sycamore::hydrate_web_app();
}
