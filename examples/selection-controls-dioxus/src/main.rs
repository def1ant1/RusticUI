//! Binary entry point forwarding to the library helpers so both desktop
//! and web pipelines share the same telemetry wiring.

#[cfg(all(feature = "desktop", not(target_arch = "wasm32")))]
fn main() {
    selection_controls_dioxus::run_desktop();
}

#[cfg(all(feature = "web", target_arch = "wasm32"))]
fn main() {
    selection_controls_dioxus::run_web();
}

#[cfg(not(any(
    all(feature = "desktop", not(target_arch = "wasm32")),
    all(feature = "web", target_arch = "wasm32")
),))]
fn main() {
    panic!("enable either the `desktop` or `web` feature to launch the example");
}
