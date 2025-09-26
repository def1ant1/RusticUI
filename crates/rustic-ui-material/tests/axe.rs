#![cfg(feature = "yew")]

//! Simple bridge to the `axe-core` accessibility engine.
//!
//! The JS implementation lives alongside this file in `axe.js` and is imported
//! via `wasm-bindgen`.  By centralizing the wrapper we keep individual tests
//! focused on assertions rather than boilerplate interop code.

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[wasm_bindgen(module = "/tests/axe.js")]
extern "C" {
    #[wasm_bindgen(catch)]
    async fn runAxe(node: JsValue) -> Result<JsValue, JsValue>;
}

/// Execute an axe-core audit against the provided DOM node.
///
/// The function panics if any accessibility violations are reported, causing
/// the enclosing test to fail.  This gives us a CI gate that enforces a11y
/// compliance for all interactive components.
pub async fn axe_check(node: &web_sys::Element) {
    // Invoke the JS bridge and unwrap the resulting Promise.
    let result = runAxe(node.clone().into())
        .await
        .expect("axe-core execution failed");
    // Extract the `violations` array and assert it is empty.
    let violations = js_sys::Reflect::get(&result, &JsValue::from_str("violations"))
        .expect("missing violations field");
    let arr = js_sys::Array::from(&violations);
    assert_eq!(arr.length(), 0, "Accessibility violations: {:?}", arr);
}
