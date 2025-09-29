//! Leptos showcase for RusticUI's pagination navigation components.
//!
//! The example focuses on automation-first patterns: controlled pagination
//! state, SSR parity, telemetry logging, and extensive inline commentary for
//! enterprise teams adopting RusticUI.

pub mod app;
#[cfg(feature = "ssr")]
pub mod ssr;
pub mod telemetry;

pub use app::{PaginationApp, HYDRATION_CONTAINER_ID};
#[cfg(feature = "ssr")]
pub use ssr::render_document;

/// Hydrate the SSR output when running in the browser.
#[cfg(all(feature = "csr", target_arch = "wasm32"))]
pub fn hydrate() {
    use leptos::*;

    #[wasm_bindgen::prelude::wasm_bindgen(start)]
    pub fn start() {
        mount_to_body(|| view! { <PaginationApp /> });
    }
}

#[cfg(all(feature = "csr", not(target_arch = "wasm32")))]
pub fn hydrate() {}
