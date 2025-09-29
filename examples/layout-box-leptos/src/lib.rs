//! Leptos example showcasing RusticUI's `Box` primitive for responsive layouts.
//! The crate mirrors production ready structure with separate modules for layout
//! blueprints, component rendering, and SSR orchestration.

pub mod blueprint;
pub mod components;
#[cfg(feature = "ssr")]
pub mod ssr;

pub use blueprint::{panel_blueprint, PanelBlueprint};
pub use components::{HydrationPhase, LayoutBoxApp, LayoutBoxAppProps, HYDRATION_CONTAINER_ID};
#[cfg(feature = "ssr")]
pub use ssr::render_document;

/// Hydrates the SSR markup when executed in a browser. The helper resolves the
/// shared container id so CSR bundles and SSR snapshots stay in lockstep.
#[cfg(all(feature = "csr", target_arch = "wasm32"))]
pub fn hydrate() {
    use leptos::mount::hydrate_to;
    use leptos::*;

    let document = web_sys::window()
        .expect("browser context")
        .document()
        .expect("document");
    let root = document
        .get_element_by_id(HYDRATION_CONTAINER_ID)
        .unwrap_or_else(|| panic!("missing hydration root '#{}'", HYDRATION_CONTAINER_ID));
    hydrate_to(root, || view! { <LayoutBoxApp /> });
}

/// Non-wasm targets compile a no-op shim for [`hydrate`].
#[cfg(all(feature = "csr", not(target_arch = "wasm32")))]
pub fn hydrate() {}
