#![cfg(feature = "yew")]

//! Thin wasm-bindgen bridge exposing the `axe-core` accessibility engine to
//! Rust-based web integration tests.
//!
//! The `wasm_bindgen` macro wires in the JavaScript implementation that lives
//! alongside the test suite (`axe.js`).  Keeping the interop layer centralized
//! minimizes boilerplate inside individual test cases and makes it trivial to
//! expand coverage across additional Joy UI components.

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[wasm_bindgen(module = "/tests/axe.js")]
extern "C" {
    #[wasm_bindgen(catch)]
    async fn runAxe(node: JsValue) -> Result<JsValue, JsValue>;
}

/// Execute an axe-core audit against the provided DOM node, panicking if any
/// violations are encountered.
///
/// Centralizing this logic ensures every test benefits from consistent
/// assertions and verbose failure messages, delivering actionable feedback when
/// regressions slip in.
pub async fn axe_check(node: &web_sys::Element) {
    // Invoke the JS bridge and unwrap the resulting Promise, bubbling up any
    // underlying errors so the test harness surfaces them immediately.
    let result = runAxe(node.clone().into())
        .await
        .expect("axe-core execution failed");

    // Extract and validate the violations array.  An empty list indicates the
    // component tree satisfied all configured accessibility rules.
    let violations = js_sys::Reflect::get(&result, &JsValue::from_str("violations"))
        .expect("missing violations field");
    let arr = js_sys::Array::from(&violations);
    assert_eq!(arr.length(), 0, "Accessibility violations: {:?}", arr);
}
