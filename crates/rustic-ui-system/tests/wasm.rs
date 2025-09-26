use mui_system::style_props;
use wasm_bindgen_test::*;

#[wasm_bindgen_test]
fn style_macro_produces_css() {
    let style = style_props! { width: "10px" };
    assert_eq!(style, "width:10px;");
}
