#![cfg(feature = "ssr")]

#[test]
fn pagination_ssr_snapshot() {
    let document = navigation_pagination_leptos::render_document();
    insta::assert_snapshot!("pagination_ssr", document);
}
