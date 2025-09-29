//! Dioxus showcase for RusticUI's speed dial navigation primitives with telemetry and SSR guidance.

pub mod app;
#[cfg(feature = "ssr")]
pub mod ssr;
pub mod telemetry;

pub use app::{SpeedDialApp, TELEMETRY_CHANNEL};
#[cfg(feature = "ssr")]
pub use ssr::render_document;

#[cfg(all(feature = "csr", target_arch = "wasm32"))]
pub fn hydrate() {
    dioxus_web::launch(app::SpeedDialApp);
}

#[cfg(all(feature = "csr", not(target_arch = "wasm32")))]
pub fn hydrate() {}
