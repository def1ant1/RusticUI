//! Enterprise-grade Yew example demonstrating RusticUI's responsive grid
//! building blocks. The crate is structured as a miniature product surface:
//! a layout blueprint encodes breakpoints, dedicated components render the
//! blueprint, and an SSR module streams deterministic HTML for monitoring and
//! prerendering pipelines.

pub mod blueprint;
pub mod components;
#[cfg(feature = "ssr")]
pub mod ssr;

pub use blueprint::{layout_blueprint, SectionBlueprint};
pub use components::{HydrationPhase, LayoutGridApp, LayoutGridAppProps, HYDRATION_CONTAINER_ID};
#[cfg(feature = "ssr")]
pub use ssr::render_document;

/// Hydrates the server-rendered document when executing inside a browser.
///
/// The helper resolves the shared container id exported by [`components`] so
/// CSR and SSR builds target the exact same DOM node. The implementation is
/// kept in `lib.rs` so integration tests can trigger hydration without going
/// through the CLI binary.
#[cfg(all(feature = "csr", target_arch = "wasm32"))]
pub fn hydrate() {
    use wasm_bindgen::JsCast;
    use web_sys::HtmlElement;
    use yew::Renderer;

    let window = web_sys::window().expect("browser environment expected for hydration");
    let document = window
        .document()
        .expect("document should be available during hydration");
    let root = document
        .get_element_by_id(HYDRATION_CONTAINER_ID)
        .unwrap_or_else(|| panic!("missing hydration root '#{}'", HYDRATION_CONTAINER_ID));
    let root: HtmlElement = root
        .dyn_into()
        .expect("hydration root should be an HtmlElement for Yew");

    Renderer::<LayoutGridApp>::with_root(root).hydrate();
}

/// Non-wasm targets (tests, SSR binaries) compile this no-op shim so callers can
/// invoke [`hydrate`] without having to guard their code with platform checks.
#[cfg(all(feature = "csr", not(target_arch = "wasm32")))]
pub fn hydrate() {}
