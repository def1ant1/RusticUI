//! End-to-end Yew showcase for RusticUI's bottom navigation primitives.
//!
//! The example doubles as production-ready reference code: it explains how to
//! wire the headless state machine, Material renderer, telemetry forwarding,
//! and SSR-friendly document generation so enterprise teams can lift the module
//! directly into staging.

pub mod app;
#[cfg(feature = "ssr")]
pub mod ssr;
pub mod telemetry;

pub use app::{BottomNavigationApp, HYDRATION_CONTAINER_ID};
#[cfg(feature = "ssr")]
pub use ssr::render_document;

/// Hydrates the SSR snapshot when executed in a browser context.
#[cfg(all(feature = "csr", target_arch = "wasm32"))]
pub fn hydrate() {
    use wasm_bindgen::prelude::*;
    use yew::Renderer;

    #[wasm_bindgen(start)]
    pub fn run() {
        Renderer::<BottomNavigationApp>::new().hydrate();
    }
}

/// Native targets compile an empty shim to keep dependency graphs minimal.
#[cfg(all(feature = "csr", not(target_arch = "wasm32")))]
pub fn hydrate() {}
