#[cfg(feature = "dioxus")]
#[test]
fn dioxus_feature_compiles() {
    mui_material::placeholder();
}

#[cfg(feature = "sycamore")]
#[test]
fn sycamore_feature_compiles() {
    mui_material::placeholder();
}
