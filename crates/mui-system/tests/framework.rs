#[cfg(feature = "dioxus")]
#[test]
fn dioxus_feature_compiles() {
    mui_system::placeholder();
}

#[cfg(feature = "sycamore")]
#[test]
fn sycamore_feature_compiles() {
    mui_system::placeholder();
}
