#![cfg(feature = "ssr")]

#[test]
fn bottom_navigation_ssr_snapshot() {
    let document = navigation_bottom_navigation_yew::render_document();
    insta::assert_snapshot!("bottom_navigation_ssr", document);
}
