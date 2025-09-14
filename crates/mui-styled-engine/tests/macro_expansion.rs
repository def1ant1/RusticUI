use trybuild::TestCases;

#[test]
fn css_with_theme_expands() {
    let t = TestCases::new();
    t.pass("tests/macro/expand_leptos.rs");
    t.pass("tests/macro/expand_dioxus.rs");
    t.pass("tests/macro/expand_sycamore.rs");
    #[cfg(feature = "yew")]
    t.pass("tests/macro/expand_yew.rs");
}
