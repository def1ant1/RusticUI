#![cfg(feature = "ssr")]

#[test]
fn speed_dial_ssr_snapshot() {
    let document = navigation_speed_dial_dioxus::render_document();
    insta::assert_snapshot!("speed_dial_ssr", document);
}
