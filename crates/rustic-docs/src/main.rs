//! Client-side entry point for the `rustic-docs` showcase.
//!
//! The binary is only meaningful when compiled for `wasm32-unknown-unknown`
//! where the exported `main` bootstraps the Leptos renderer in the browser.
//! For native targets we simply log guidance to run the dedicated server
//! binary instead so developers do not accidentally start the wrong target.

#[cfg(target_arch = "wasm32")]
pub fn main() {
    // In the browser we hand control to the shared `client_main` utility.
    rustic_docs::client_main();
}

#[cfg(not(target_arch = "wasm32"))]
pub fn main() {
    eprintln!("rustic-docs: build the `rustic-docs-server` binary for SSR/static export workflows");
}
