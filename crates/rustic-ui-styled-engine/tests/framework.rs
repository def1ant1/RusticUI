#[cfg(feature = "dioxus")]
#[test]
fn dioxus_feature_compiles() {
    rustic_ui_styled_engine::placeholder();
}

#[cfg(feature = "sycamore")]
#[test]
fn sycamore_feature_compiles() {
    rustic_ui_styled_engine::placeholder();
}
